# leptos.nvim

Experimental Lua/Neovim wrapper for [`leptos_reactive`](https://github.com/leptos-rs/leptos).

## Installation

With `lazy.nvim`

```lua
_ = {
  "willothy/leptos.nvim",
  event = "VeryLazy"
}
```

With other package managers, ensure to specify `just build` as the build script.

## Example usage

Count stars for a github repo using `gh api` and `jq`, reactively update a buffer as stdout is read.

```lua
-- this example relies on nui.nvim for rendering
local Text = require("nui.text")
local Line = require("nui.line")

local rx = require("leptos")

---@param user string?
local function count_stars(user)
  -- create the window
  user = user or vim.env.USER
  local width = math.max(12, #user + 2)
  local buf, win = vim.lsp.util.open_floating_preview({}, "", {
    focus = true,
    focusable = true,
    wrap = true,
    width = width,
    height = 1,
    border = "single",
    title = user,
    title_pos = "center",
  })

  -- create the signals that will handle state
  local total = rx.create_signal(0)
  local err = rx.create_signal()

  -- this will run whenever `total` or `err` are updated
  rx.create_effect(function()
    -- signals will be automatically subscribed to the running effect
    local val = total.get()
    local star
    -- check for errors
    if err.get() then
      val = Text(err.get())
      star = Text("", "DiagnosticError")
    else
      val = Text(tostring(val))
      star = Text("", "DiagnosticWarn")
    end
    -- check that the window can fit the text
    if (val:length() + 2) > width then
      width = val:length() + 2
      local config = vim.api.nvim_win_get_config(win)
      config.width = val:length() * 2
      vim.api.nvim_win_set_config(win, config)
    end
    -- construct the text object
    local lpad =
      Text(string.rep(" ", math.floor((width - val:length()) / 2) - 1))
    local line = Line({ lpad, star, Text(" "), val })
    -- render to the buffer
    if vim.api.nvim_buf_is_valid(buf) then
      vim.bo[buf].modifiable = true
      line:render(buf, -1, 1)
      vim.bo[buf].modifiable = false
    end
  end)

  -- buffer for received data
  local buffer = ""

  vim.system({
    "gh",
    "api",
    "-X",
    "GET",
    "/users/" .. user .. "/repos",
    "--paginate",
    "-F",
    "per_page=10", -- only 10 repos per-page
    "--jq",
    ".[].stargazers_count", -- use jq to retrieve the star count for each repo
  }, {
    text = true,
    -- Incrementally update the total star count as pages are received from the API
    stdout = function(e, data)
      if e then
        err.set(e)
        return
      end
      if not data then
        return
      end
      -- keep all of the data for json decoding
      buffer = buffer .. data
      local num = vim
        .iter(vim.split(data, "\n"))
        :map(tonumber)
        :fold(0, function(acc, num)
          return acc + num
        end)
      -- Update tables in-place, or return a new value to update by value
      --
      -- This is equivalent to calling get() and set(), with the exception of
      -- table signals, which can be mutated in place.
      total.update(function(t)
        return (t or 0) + num
      end)
    end,
  }, function(obj)
    -- Handle errors by setting the error signal
    if obj.code ~= 0 or #obj.stderr ~= 0 then
      local output = vim.json.decode(buffer)
      err.set(output.message)
    end
  end)
end
```
