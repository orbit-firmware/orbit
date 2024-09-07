import { defineConfig } from 'vitepress'

export default defineConfig({
  title: "RMK Firmware",
  description: "Documentaion for the RMK firmware",
  base: '/rmk/',
  head: [['link', { rel: 'icon', href: 'favicon.ico' }]],
  themeConfig: {
    logo: 'logo.svg',
    search: {
      provider: 'local'
    },

    nav: [
      { text: 'Home', link: '/' },
      { text: 'Documentation', link: '/getting-started' }
    ],

    sidebar: [
      {

        items: [
          { text: 'Getting Started', link: '/getting-started' },
          { text: 'Concepts', link: '/' },
          { text: 'Keymaps', link: '/' },
          { text: 'Keycodes', link: '/' },
          { text: 'Runtime API Examples', link: '/' }
        ],
      },
      {
        text: 'Features',
        items: [
          { text: 'Keypress', link: '/' },
          { text: 'Layers', link: '/' },
          { text: 'Modify', link: '/' },
          { text: 'Tap', link: '/' },
          { text: 'OS', link: '/' },
        ]
      }
    ],

    socialLinks: [
      { icon: 'github', link: 'https://github.com/rmk-firmware' }
    ]
  }
})
