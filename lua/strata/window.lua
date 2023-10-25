local api = require("strata.api")

local module = {}

--- Moves a window to a different workspace
---@param id number
---@param opts table
---@return function
function module.move(id, opts)
    if opts and opts.follow then
	    return function() api.move_window_and_follow(id) end
    else
	    return function() api.move_window(id) end
    end
end

--- Closes the currently active window
---@return function
function module.close()
	return function() api.close_window() end
end

return module