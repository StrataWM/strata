local api = require("strata.api")

local module = {
	actions = require("actions"),
	rules = require("rules"),
	api = api, -- mlua module

	-- Exposed mlua API functions
	set_bindings = api.set_bindings,
	set_rules = api.set_rules,
	set_config = api.set_config,
}

return module
