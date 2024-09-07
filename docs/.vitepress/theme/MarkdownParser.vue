<template>
  <div v-html="parsedMarkdown"></div>
</template>

<script>
import { marked } from 'marked';

export default {
  props: {
    markdownString: {
      type: String,
      required: true,
    },
  },
  computed: {
    parsedMarkdown() {
      return marked(this.markdownString);
    },
  },
};
</script>