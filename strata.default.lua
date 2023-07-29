local strata = require("strata")

return {
    autostart {
        "kitty --tile Terminal",
        "kagi"
    },
    general {
        workspaces = 1,
        gaps_in = 8,
        gaps_out = 12,
        kb_repeat = {500, 250}
    },
    decorations {
        border {
            width = 2,
            active = "#FFF",
            inactive = "#131418",
            radius = 5,
        },
        window {
            opacity = 0.9
        },
        blur {
            enabled = true,
            size = 2,
            passes = 3,
            optimize = true,
        },
        shadow {
            enabled = true,
            size = 2,
            blur = 3,
            color = "#FFF"
        }
    },
    tiling {
        layout = "dwindle"
    },
    animations {
        enabled = true,
    },
    rules {
        workspaces {
            {
                workspace = 1,
                class_name = "kitty"
            },
            {
                workspace = 2,
                class_name = "Brave-browser"
            }
        },
        floating {
            {
                class_name = "pavucontrol"
            }
        }
    },
    bindings {
        {
            {"CTRL", "SHIFT", "Q"},
            function()
              for window in strata.current_workspace.get_windows() 
              do
                window.close()
              end
            end,
        },
        {
            {"WIN", "RETURN"},
            strata.spawn("kitty --title Terminal");
        }
    }
}