import { defineConfig } from 'vitepress'

export default defineConfig({
  title: "Strata",
  description: "A cutting-edge, robust and sleek Wayland compositor with batteries included.",
  base: "/strata/",
  markdown: {
    theme: { dark: 'catppuccin-mocha', light: 'catppuccin-latte' }
  },
  themeConfig: {
    nav: [
      { text: 'Home', link: 'https://stratawm.github.io' },
      { text: 'Get Started', link: '/get-started/installing' },
      { text: 'Configuration', link: '/configuration/basics' },
    ],

    sidebar: [
      {
        text: 'Get Started',
        items: [
          { text: 'Installing', link: '/get-started/installing' },
          { text: 'Troubleshooting', link: '/get-started/troubleshooting' }
        ]
      },
      {
        text: 'Configuration',
        items: [
          { text: 'Basics', link: '/configuration/basics' },
          { text: 'Keybindings', link: '/configuration/keybindings' },
          { text: 'Window rules', link: '/configuration/window-rules' },
        ]
      }
    ],

    socialLinks: [
      { icon: 'github', link: 'https://github.com/stratawm/strata' },
      { icon: 'discord', link: 'https://discord.gg/tcb5cRW4ZQ' }
    ],
    
    search: {
      provider: "local",
    },

    footer: {
      copyright: "Copyright © 2023-present Anant Narayan",
      message: "Licensed under the <a href='https://github.com/stratawm/strata/blob/main/LICENSE'>GPL v3 License</a>."
    }
  }
})
