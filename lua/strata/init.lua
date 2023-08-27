local api = require("strata.api")

local module = {
	actions = require("actions"),
	rules = require("rules"),
	api = api, -- mlua module

	-- Exposed mlua API functions
	set_config = api.set_config,
	get_config = api.get_config,
	update_config = api.update_config,
}

return module
