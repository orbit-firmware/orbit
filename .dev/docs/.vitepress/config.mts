import { defineConfig } from 'vitepress'


import keycodes from './keycodes.mts'

export default defineConfig({
  title: "Orbit Firmware",
  description: "Documentaion for the Orbit firmware",
  base: '/orbit/',
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
          { text: 'Configuration', link: '/configuration' },
        ],
      },
      {
        text: 'Keycodes',
        link: '/keycodes',
        items: keycodes
      },
      {
        text: 'Behaviors',
        items: [
          { text: 'Press', link: '/' },
          { text: 'Hold', link: '/' },
          { text: 'Modify', link: '/' },
          { text: 'Tap', link: '/' },
          { text: 'OS', link: '/' },
        ]
      },
      {
        text: 'Actions',
        items: [
          { text: 'Layers', link: '/' },
          { text: 'Mouse', link: '/' },
          { text: 'RGB', link: '/' },
        ]
      }
    ],

    socialLinks: [
      { icon: 'discord', link: 'https://discord.gg/SrESTtBKV5' },
      { icon: 'github', link: 'https://github.com/orbit-firmware/orbit' }
    ]
  }
})
