<template>
  <div class="m-2">
    <pre class="bg-dark-subtle rounded">
      <code class="language-json" v-html="html"></code>
    </pre>
  </div>
</template>

<style scoped>
span:first-child {
  white-space: pre-line;
}
</style>

<script lang="ts" setup>
import hljs from "highlight.js";
import "highlight.js/styles/default.css";

const route = useRoute();
const payload = JSON.stringify(JSON.parse(route.query.payload), null, 2);
const html = hljs.highlight(payload, { language: "json" }).value.trimStart();

definePageMeta({
  layout: "window",
});
</script>
