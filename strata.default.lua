local strata = require("strata")

local function close_all_windows()
	for _, window in ipairs(strata.current_workspace:get_windows()) do
		window:close()
	end
end

local config = {
	autostart = {
		{ "kitty --title Terminal" },
	},
	general = {
		workspaces = 9,
		gaps_in = 8,
		gaps_out = 12,
		kb_repeat = { 500, 250 },
	},
	decorations = {
		border = {
			enable = true,
			width = 2,
			active = "#FFF",
			inactive = "#131418",
			radius = 5,
		},
		window = {
			opacity = 0.9,
		},
		blur = {
			enable = true,
			size = 2,
			passes = 3,
			optimize = true,
		},
		shadow = {
			enable = true,
			size = 2,
			blur = 3,
			color = "#FFF",
		},
	},
	tiling = {
		layout = "dwindle",
	},
	animations = {
		enable = true,
	},
	bindings = {
		{
			keys = { "Super_L", "Return" },
			action = strata.actions.spawn("kitty --title Terminal"),
		},
		{
			keys = { "Super_L", "space" },
			action = strata.actions.spawn("rofi --show drun"),
		},
		{
			keys = { "Alt_L", "q" },
			action = strata.window.close,
		},
		{
			keys = { "Alt_L", "m" },
			action = strata.actions.quit,
		},
		{
			keys = { "Super_L", "b" },
			action = function() -- Toggle border
				local border_enabled = strata.get_config().decorations.border.enable
				strata.update_config {
					decorations = {
						border = {
							enable = not border_enabled,
						},
					},
				}
			end,
		},
	},
	rules = {
		{
			triggers = { { event = "win_open_pre", class_name = "firefox" } },
			action = function(window) window:send_to_workspace(1) end,
		},
		{
			triggers = {
				{ event = "win_open_pre", class_name = "mpv" },
				{ event = "win_open_pre", workspace = 1, class_name = "kitty" },
			},
			action = function(window) window:set_floating() end,
		},
		strata.rules.bind_to_workspace {
			{ 1, "firefox" },
			{ 2, "neovide" },
			{ 10, "slack" },
		},

		strata.rules.set_floating {
			"mpv",
		},
	},
}

for i = 1, 10 do
	table.insert(config.bindings, {
		keys = { "Alt_L", tostring(i) },
		action = strata.workspace.switch(i)
	})
end

for i = 1, 10 do
	table.insert(config.bindings, {
		keys = { "Alt_L", "Shift_L", tostring(i) },
		action = strata.window.move(i, { follow = true })
	})
end

strata.set_config(config)