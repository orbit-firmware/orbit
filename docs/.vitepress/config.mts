import { defineConfig } from 'vitepress'

import { MarkdownOptions } from "vitepress";
import { LanguageRegistration } from "shikiji";

import grammar from "./theme/tmorbit.json"
import pages from '../_pages.mts'


const orbit: LanguageRegistration = {
  id: "orbit",
  aliases: ["orbit-alias"],
  ...grammar,
};

const md: MarkdownOptions = {
  languages: [orbit],
  // theme: "one-dark-pro",
};

export default defineConfig({
  title: "orbit",
  description: "Documentaion for the orbit firmware",
  base: '/orbit/',
  head: [['link', { rel: 'icon', href: '/orbit/favicon.ico' }]],
  markdown: md,
  themeConfig: {
    logo: '/logo.svg',
    search: {
      provider: 'local'
    },

    sidebar: pages,

    socialLinks: [
      { icon: 'discord', link: 'https://discord.gg/SrESTtBKV5' },
      { icon: 'github', link: 'https://github.com/orbit-firmware/orbit' }
    ]
  }
})


