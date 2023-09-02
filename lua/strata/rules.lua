local module = {}

---@alias TriggerEvent
---| "win_open_pre" -- Before the window is opened
---| "win_open_post" -- After the window is opened

---@class Trigger
---@field event TriggerEvent|TriggerEvent[] The event name or a list of event names
---@field class_name string|string[] The class name or a list of class names, as regexes

---@class Rule
---@field triggers Trigger|Trigger[] The trigger or a list of triggers
---@field action function

---Transform a list of arbitrary inputs into proper rules
---@generic T
---@param inputs T[] A list of arbitrary inputs
---@param callback fun(input: T): Rule
---@return Rule[]
local function map_rules(inputs, callback)
	local rules = {} ---@type Rule[]

	for _, input in ipairs(inputs) do
		table.insert(rules, callback(input))
	end

	return rules
end

---@class BindToWorkspaceArgs
---@field [1] number The workspace number
---@field [2] string|string[] The class name or a list of class names

---Bind a list of classes to a workspace
---@param inputs BindToWorkspaceArgs[] A list of workspace/class mappings
---@return Rule[]
function module.bind_to_workspace(inputs)
	return map_rules(inputs, function(input)
		return {
			triggers = { { event = "win_open_pre", class_name = input[2] } },
			action = function(window) window:move_to_workspace(input[1]) end,
		}
	end)
end

---Set a list of classes to floating
---@param inputs string[] A list of class names
---@return Rule[]
function module.set_floating(inputs)
	return map_rules(inputs, function(input)
		return {
			triggers = { { event = "win_open_pre", class_name = input[1] } },
			action = function(window) window:set_floating() end,
		}
	end)
end

return module
