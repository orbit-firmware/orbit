// https://vitepress.dev/guide/custom-theme
import { h, watch } from 'vue'
import type { Theme } from 'vitepress'
import DefaultTheme from 'vitepress/theme'
import './style.css'
import fs from 'fs'

const anchor_flash = (id: string) => {
  if (typeof window === 'undefined') return;
  const target = document.querySelector(id);
  if (!target) return;

  try {
    if (target?.firstChild?.matches('span.anchor-flash')) return;
  } catch (e) {
    // anchor-flash not persent, continue
  }

  const span = document.createElement('span');
  span.className = 'anchor-flash';
  while (target.firstChild) {
    span.appendChild(target.firstChild);
  }
  target.appendChild(span);

  setTimeout(() => {
    while (span.firstChild) {
      target.insertBefore(span.firstChild, span);
    }
    span.remove();
  }, 1250)
}


const load_orbit_language = async () => {
  const myLang = JSON.parse(fs.readFileSync('my-lang.json', 'utf8'))

  await highlighter.loadLanguage(myLang)
}

export default {
  extends: DefaultTheme,
  Layout: () => {
    return h(DefaultTheme.Layout, null, {
      // https://vitepress.dev/guide/extending-default-theme#layout-slots
    })
  },
  enhanceApp({ app, router, siteData }) {
    if (typeof window === 'undefined') return;

    document.body.addEventListener('click', (e) => {
      if (!e.target?.matches('a[href^="#"]')) return;
      anchor_flash(e.target.getAttribute('href'));
    });

    watch(() => router.route.data.relativePath, (path) => {
      if (!window.location.hash) return;
      setTimeout(() => {
        const el = document.querySelector(".header-anchor[href='" + window.location.hash + "']");
        if (el) el.click();
      }, 10);
    }, { immediate: true });
  }
} satisfies Theme