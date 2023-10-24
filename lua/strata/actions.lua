local api = require("strata.api")
local utils = require("utils")

local module = {}

---Returns a callback that spawns a given command
---@param cmd string|string[]
---@return function
function module.spawn(cmd)
	if type(cmd) == "string" then
		cmd = utils.parse_cmd(cmd)
	elseif type(cmd) ~= "table" then
		error("Invalid argument type for strata.cmd.spawn")
	end

	return function() api.spawn(cmd) end
end

--- Switches to a workspace
---@param id number
---@return function
function module.switch_to_ws(id)
	return function() api.switch_to_ws(id) end
end

--- Moves a window to a different workspace
---@param id number
---@return function
function module.move_window(id)
	return function() api.move_window(id) end
end

--- Moves a window to a different workspace and switches to the same one
---@param id number
---@return function
function module.move_window_and_follow(id)
	return function() api.move_window_and_follow(id) end
end

--- Closes the currently active window
---@return function
function module.close_window()
	return function() api.close_window() end
end

--- Quits the compositor (safely)
---@return function
function module.quit()
	return function() api.quit() end
end

return module
