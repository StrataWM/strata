local strata = require("strata")

local function close_all_windows()
	for _, window in ipairs(strata.current_workspace:get_windows()) do
		window:close()
	end
end

-- This us a *function call*, not a global variable affectation. This can be called multiple times and should trivially
-- enable plugins to do anything they want dynamically. It also prevents the user's editor from throwing errors for
-- unknown global variables.
-- Can also be written that way:
--     strata.set_bindings({ ... })
-- strata.set_bindings {
-- 	{
-- 		keys = { "CTRL", "SHIFT", "Q" },
-- 		cmd = close_all_windows,
-- 	},
-- 	{
-- 		keys = { "WIN", "RETURN" },
-- 		-- `strata.cmd` should contain a bunch of common built-in functions that generate callbacks to be used
-- 		-- here, so that the boilerplate is reduced for most use-cases.
-- 		cmd = strata.cmd.spawn("kitty --title Terminal"),
-- 	},
-- }

-- This is my take to improve the "rules" system and make it much more flexible.
-- It's a simple association between a trigger, and an action. Honestly, the whole `set_bindings` function could be
-- implemented with this as well, but as it's more verbose it makes sense to keep the above one as a sugar over this
-- more generic function.
-- This is very close to how neovim handles events with autocmds. I guess it would make sense to add an optional `group`
-- parameter as well here, so that we can bundle multiple rules together and disable them all at once with another
-- function (this is useful for plugins).
-- strata.set_rules {
-- 	-- Always bind firefox to the workspace 1, before the window is opened
-- 	{
-- 		triggers = { { event = "win_open_pre", class_name = "firefox" } },
-- 		action = function(window) window.send_to_workspace(1) end,
-- 	},
--
-- 	-- A single action that can be ran from multiple triggers.
-- 	{
-- 		triggers = {
-- 			-- Set mpv to floating, always
-- 			{ event = "win_open_pre", class_name = "mpv" },
-- 			-- Set terminals to floating only if on the workspace 1 (where we put firefox previously)
-- 			{ event = "win_open_pre", workspace = 1, class_name = "kitty" },
-- 		},
-- 		action = function(window) window.set_floating() end,
-- 	},

-- Here comes the nice stuff: helper functions! This is important, as these functions help make the config file much
-- easier to write for the majority of users. Ideally, most people won't ever need to write something detailed like
-- what's above.

-- This one does the same thing as the first rule
-- strata.rules.bind_to_workspace(1, "firefox"),
--
-- -- Nothing preventing us from some more sugar, to make things even less verbose:
-- strata.rules.bind_to_workspace({
-- 	{ 1, "firefox" },
-- 	{ 2, "neovide" },
-- 	{ 10, "slack" },
-- }),
--
-- -- This does the same thing as the first trigger of the second rule
-- strata.rules.set_floating("mpv"),
-- }

-- Same as above, this is a function call. I think as it stands it should *overwrite* the whole config every time it's
-- called. For a "merge" behavior, we could have a `strata.update_config()` function instead. This can one day be useful
-- for plugins.
strata.set_config {
	autostart = {
		{ "kitty --title Terminal" },
		{ "kagi" },
	},
	general = {
		workspaces = 10,
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

	-- I guess we can have the same systems as above here for convenience for most users:
	bindings = {
		{
			keys = { "CTRL", "SHIFT", "Q" },
			cmd = close_all_windows,
		},
		{
			keys = { "WIN", "RETURN" },
			-- `strata.cmd` should contain a bunch of common built-in functions that generate callbacks to be used
			-- here, so that the boilerplate is reduced for most use-cases.
			cmd = strata.cmd.spawn("kitty --title Terminal"),
		},
	}, -- same thing as the argument for `strata.set_bindings()`
	rules = {
		{
			triggers = { { event = "win_open_pre", class_name = "firefox" } },
			action = function(window) window.send_to_workspace(1) end,
		},

		-- A single action that can be ran from multiple triggers.
		{
			triggers = {
				-- Set mpv to floating, always
				{ event = "win_open_pre", class_name = "mpv" },
				-- Set terminals to floating only if on the workspace 1 (where we put firefox previously)
				{ event = "win_open_pre", workspace = 1, class_name = "kitty" },
			},
			action = function(window) window.set_floating() end,
		},
	}, -- same thing as the argument for `strata.set_rules()`
}
