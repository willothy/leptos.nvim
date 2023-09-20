build:
  cargo build --release
  cp ./target/release/libleptos_nvim.so ./lua/leptos.so
