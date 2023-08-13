local strata = require("strata")

local function close_all_windows()
	for _, window in ipairs(strata.current_workspace:get_windows()) do
		window:close()
	end
end

strata.set_bindings {
	{
		keys = { "CTRL", "SHIFT", "Q" },
		cmd = close_all_windows,
	},
	{
		keys = { "WIN", "RETURN" },
		cmd = strata.cmd.spawn("kitty --title Terminal"),
	},
	{
		keys = { "WIN", "SPACE" },
		cmd = strata.cmd.spawn("rofi --show drun"),
	},
}

strata.set_rules {
	{
		triggers = { event = "win_open_pre", class_name = "firefox" },
		action = function(window) window.send_to_workspace(1) end,
	},
	{
		triggers = {
			{ event = "win_open_pre", class_name = "mpv" },
			{ event = "win_open_pre", workspace = 1, class_name = { "kitty", "wezterm" } },
		},
		action = function(window) window.set_floating() end,
	},

	strata.rules.bind_to_workspace(1, "firefox"),
	strata.rules.bind_to_workspace {
		{ 1, "firefox" },
		{ 2, "neovide" },
		{ 10, "slack" },
	},

	strata.rules.set_floating("mpv"),
}

strata.set_config {
	autostart = {
		{ "kitty", "--title", "Terminal" },
	},
	general = {
		workspaces = 9,
		gaps_in = 8,
		gaps_out = 12,
		kb_repeat = { 500, 250 },
	},
	decorations = {
		border_width = 2,
		border_active = "#FFF",
		border_inactive = "#131418",
		border_radius = 5,
		window_opacity = 0.9,
		blur_enable = true,
		blur_size = 2,
		blur_passes = 3,
		blur_optimize = true,
		shadow_enabled = true,
		shadow_size = 2,
		shadow_blur = 3,
		shadow_color = "#FFF",
	},
	tiling = {
		layout = "dwindle",
	},
	animations = {
		enabled = true,
	},
	bindings = {
		{
			keys = { "CTRL", "SHIFT", "Q" },
			cmd = close_all_windows,
		},
		{
			keys = { "WIN", "RETURN" },
			cmd = strata.cmd.spawn("kitty --title Terminal"),
		},
		{
			keys = { "WIN", "SPACE" },
			cmd = strata.cmd.spawn("rofi --show drun"),
		},
	},
	rules = {
		{
			triggers = { event = "win_open_pre", class_name = "firefox" },
			action = function(window) window.send_to_workspace(1) end,
		},
		{
			triggers = {
				{ event = "win_open_pre", class_name = "mpv" },
				{ event = "win_open_pre", workspace = 1, class_name = { "kitty", "wezterm" } },
			},
			action = function(window) window.set_floating() end,
		},
	},
}
