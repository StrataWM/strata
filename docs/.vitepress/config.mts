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
        text: 'Get Started',
        items: [
          { text: 'Binary Packages', link: '/get-started/binary' },
          { text: 'Compiling', link: '/get-started/compiling' }
        ]
      }
    ],

    socialLinks: [
      { icon: 'github', link: 'https://github.com/stratawm/strata' }
    ]
  }
})
