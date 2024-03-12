import { defineConfig } from 'vitepress'

export default defineConfig({
  title: "Strata",
  description: "A cutting-edge, robust and sleek Wayland compositor with batteries included.",
  themeConfig: {
    nav: [
      { text: 'Home', link: '/' },
      { text: 'Get Started', link: '/get-started/' }
    ],

    sidebar: [
      {
        text: 'Examples',
        items: [
          { text: 'Markdown Examples', link: '/markdown-examples' },
          { text: 'Runtime API Examples', link: '/api-examples' }
        ]
      }
    ],

    socialLinks: [
      { icon: 'github', link: 'https://github.com/stratawm/strata' }
    ]
  }
})
