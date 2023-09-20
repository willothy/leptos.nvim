local sourced_filename = (function()
	return vim.fn.fnamemodify(vim.fs.normalize(debug.getinfo(2, "S").source:sub(2)), ":p")
end)()

local cwd = vim.fn.fnamemodify(sourced_filename, ":h")

vim.system({
	"cargo",
	"build",
	"--release",
}, {
	cwd = cwd,
}, function(o)
	if o.code ~= 0 then
		error(o.stderr)
	end

	vim.uv.fs_copyfile(cwd .. "/target/release/libleptos.so", "lua/leptos.so")
end)
