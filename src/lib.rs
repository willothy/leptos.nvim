use leptos_reactive::{SignalGet, SignalSet, SignalUpdate};
use mlua::{Function, Lua, Table, Value};

fn create_effect(_: &'static Lua, args: (Function<'static>,)) -> mlua::Result<()> {
    leptos_reactive::create_effect(move |v: Option<Value>| {
        return args.0.call((v,)).unwrap_or_else(|_| Value::Nil);
    });
    Ok(())
}

fn create_signal(lua: &'static Lua, args: (Value<'static>,)) -> mlua::Result<Table<'static>> {
    let (read, write) = leptos_reactive::create_signal(args.0);
    let t = lua.create_table()?;
    t.set("get", Function::wrap(move |_, _: ()| Ok(read.get())))?;
    t.set(
        "set",
        Function::wrap(move |_, args: (Value,)| {
            write.set(args.0);
            Ok(())
        }),
    )?;
    t.set(
        "update",
        Function::wrap(move |_, args: (Function<'static>,)| {
            write.update(move |v| {
                let val = args.0.call((v.to_owned(),)).unwrap_or_else(|_| Value::Nil);
                if !val.is_nil() {
                    *v = val;
                }
            });
            Ok(())
        }),
    )?;
    Ok(t)
}

#[mlua::lua_module]
fn leptos(l: &'static Lua) -> mlua::Result<Table> {
    leptos_reactive::set_current_runtime(leptos_reactive::create_runtime());

    let t = l.create_table()?;
    t.set("create_signal", Function::wrap(create_signal))?;
    t.set("create_effect", Function::wrap(create_effect))?;

    Ok(t)
}
