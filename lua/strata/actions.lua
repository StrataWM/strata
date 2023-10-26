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

--- Quits the compositor (safely)
---@return function
function module.quit()
	return function() api.quit() end
end

return module
