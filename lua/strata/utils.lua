local module = {}

---Parses a command string into a table of arguments
---@param cmd string
---@return string[]
function module.parse_cmd(cmd)
	local args = {} ---@type string[]
	for arg in cmd:gmatch("%S+") do
		table.insert(args, arg)
	end
	return args
end

return module
