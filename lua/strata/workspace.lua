local api = require("strata.api")

local module = {}

--- Switches to a workspace
---@param id number
---@return function
function module.switch(id)
	return function() api.switch_to_ws(id) end
end

return module
