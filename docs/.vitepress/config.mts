import { defineConfig } from 'vitepress'

export default defineConfig({
  title: "Strata",
  description: "A cutting-edge, robust and sleek Wayland compositor with batteries included.",
  themeConfig: {
    nav: [
      { text: 'Home', link: '/' },
      { text: 'Get Started', link: '/get-started/installing' }
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
          { text: 'General', link: '/configuration/general' },
          { text: 'Keybindings', link: '/configuration/keybindings' },
          { text: 'Window rules', link: '/configuration/window-rules' },
        ]
      }
    ],

    socialLinks: [
      { icon: 'github', link: 'https://github.com/stratawm/strata' },
      { icon: 'discord', link: 'https://discord.gg/tcb5cRW4ZQ' }
    ]
  }
})
