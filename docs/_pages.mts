import keycodes from './_keycodes.mts'

export default [
  {
    text: 'Setup',
    link: '/getting-started',
    items: [
      { text: 'Introduction', link: '/' },
      { text: 'Getting Started', link: '/getting-started' },
      { text: 'Configuration', link: '/configuration' },
      { text: 'Roadmap', link: '/roadmap' },
    ],
  },
  {
    text: 'Keymap',
    link: '/keymap',
    items: [
      { text: 'Examples', link: '/keymap/examples.md' },
    ]
  },
  {
    text: 'Behaviors',
    link: '/behaviors',
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
    link: '/actions',
    items: [
      { text: 'Layers', link: '/actions/layers' },
      { text: 'Modifiers', link: '/actions/modifiers' },
      { text: 'Mouse', link: '/actions/mouse' },
      { text: 'RGB', link: '/actions/rgb' },
    ]
  },
  {
    text: 'Flavors',
    link: '/flavors',
    items: [
      { text: 'Space Cadet', link: '/flavors/space_cadet' },
    ]
  },
  {
    text: 'Keycodes',
    link: '/keycodes',
    items: keycodes
  },
]