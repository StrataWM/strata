local api = require("strata.api")

local module = {
	actions = require("actions"),
	rules = require("rules"),
	api = api, -- mlua module

	-- Exposed mlua API functions
	set_config = api.set_config,
}

return module
