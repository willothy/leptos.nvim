local sourced_filename = (function()
	return vim.fn.fnamemodify(vim.fs.normalize(debug.getinfo(2, "S").source:sub(2)), ":p")
end)()

local cwd = vim.fn.fnamemodify(sourced_filename, ":h")

local o = vim.system({
	"cargo",
	"build",
	"--release",
}, {
	cwd = cwd,
	stderr = vim.schedule_wrap(function(_, data)
		vim.api.nvim_out_write(data or "")
	end),
}):wait()

if o.code ~= 0 then
	error(o.stderr)
end
vim.uv.fs_copyfile(cwd .. "/target/release/libleptos_nvim.so", cwd .. "/lua/leptos.so")
