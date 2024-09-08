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

    sidebar: [
      {
        items: [
          { text: 'Introduction', link: '/' },
        ],
      },
      {
        text: 'Setup',
        items: [
          { text: 'Getting Started', link: '/getting-started' },
          { text: 'Keycodes', link: '/keycodes' },
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
      { icon: 'discord', link: 'https://discord.gg/SrESTtBKV5' },
      { icon: 'github', link: 'https://github.com/rmk-firmware/rmk' }
    ]
  }
})
