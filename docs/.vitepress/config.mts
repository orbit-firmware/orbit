import { defineConfig } from 'vitepress'


import keycodes from './keycodes.mts'

export default defineConfig({
  title: "orbit",
  description: "Documentaion for the orbit firmware",
  base: '/orbit/',
  head: [['link', { rel: 'icon', href: '/orbit/favicon.ico' }]],

  themeConfig: {
    logo: '/logo.svg',
    search: {
      provider: 'local'
    },

    sidebar: [
      {
        text: 'Setup',
        items: [
          { text: 'Introduction', link: '/' },
          { text: 'Getting Started', link: '/getting-started' },
          { text: 'Configuration', link: '/configuration' },
        ],
      },
      {
        text: 'Behaviors',
        items: [
          { text: 'Press', link: '/behaviors/press' },
          { text: 'Tap', link: '/behaviors/tap' },
          { text: 'Hold', link: '/behaviors/hold' },
          { text: 'Modify', link: '/behaviors/modify' },
          { text: 'OS', link: '/behaviors/os' },
        ]
      },
      {
        text: 'Actions',
        items: [
          { text: 'Layers', link: '/actions/layers' },
          { text: 'Mouse', link: '/actions/mouse' },
          { text: 'RGB', link: '/actions/rgb' },
        ]
      },
      {
        text: 'Flavors',
        items: [
          { text: 'Space Cadet', link: '/flavors/space_cadet' },
        ]
      },
      {
        text: 'Keycodes',
        items: keycodes
      },
    ],

    socialLinks: [
      { icon: 'discord', link: 'https://discord.gg/SrESTtBKV5' },
      { icon: 'github', link: 'https://github.com/orbit-firmware/orbit' }
    ]
  }
})
