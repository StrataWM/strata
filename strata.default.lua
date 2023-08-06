local strata = require("strata")

function close_all_windows()
    for _, window in ipairs(strata.current_workspace:get_windows()) do
        window:close()
    end
end

bindings = {
    {
        keys = { "CTRL", "SHIFT", "Q" },
        cmd = close_all_windows,
    },
    {
        keys = { "WIN", "RETURN" },
        cmd = function()
            strata.spawn("kitty --title Terminal")
        end,
    },
}

config = {
    autostart = {
        { "kitty", "--title", "Terminal" },
        { "kagi" } 
    },
    general = {
        workspaces = 1,
        gaps_in = 8,
        gaps_out = 12,
        kb_repeat = { 500, 250 }
    },
    decorations = {
        border = {
            width = 2,
            active = "#FFF",
            inactive = "#131418",
            radius = 5,
        },
        window = {
            opacity = 0.9
        },
        blur = {
            enabled = true,
            size = 2,
            passes = 3,
            optimize = true,
        },
        shadow = {
            enabled = true,
            size = 2,
            blur = 3,
            color = "#FFF"
        }
    },
    tiling = {
        layout = "dwindle"
    },
    animations = {
        enabled = true,
    },
    rules = {
        workspaces = {
            {
                workspace = 1,
                class_name = "kitty"
            },
            {
                workspace = 2,
                class_name = "Brave-browser"
            }
        },
        floating = {
            {
                class_name = "pavucontrol"
            }
        }
    },
}
