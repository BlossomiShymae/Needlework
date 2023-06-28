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
import { Invoker } from "~/composables/invoker";
import { inject } from "vue";
import hljs from "highlight.js";
import "highlight.js/styles/default.css";

const invoker = inject(Invoker.Key) as Invoker;

const route = useRoute();
const key = route.query.key as string;
const payload = await invoker.get_data_payload(key);
console.log(key, payload);

const html = hljs
  .highlight(JSON.stringify(payload, null, 2), { language: "json" })
  .value.trimStart();

definePageMeta({
  layout: "window",
});
</script>
