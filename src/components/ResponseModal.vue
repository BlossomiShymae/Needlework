<template>
  <div
    class="modal fade"
    tabindex="-1"
    :id="`modal-${hash}`"
    style="z-index: 10000"
  >
    <div class="modal-dialog modal-fullscreen">
      <div class="modal-content">
        <div class="modal-header">
          <h5 class="modal-title">Response Body</h5>
          <button
            type="button"
            class="btn-close"
            data-bs-dismiss="modal"
          ></button>
        </div>
        <div class="modal-body">
          <div class="alert alert-secondary">
            <pre>
<code class="language-json" v-html="html"></code>
            </pre>
          </div>
        </div>
        <div class="modal-footer">
          <button class="btn btn-outline-secondary fw-semibold" @click="copyToClipboard">
            <span
              ><PhClipboard color="white" side="24"
            /></span>
            Copy
          </button>
          <button type="button" class="btn btn-outline-primary fw-semibold" data-bs-dismiss="modal">
            <span>
              <PhX color="white" side="24" />
            </span>
            Close
          </button>
        </div>
      </div>
    </div>
  </div>
</template>

<script lang="ts" setup>
import { PhClipboard, PhX } from "@phosphor-icons/vue";
import { writeText } from "@tauri-apps/api/clipboard";
import { computed, Ref } from "vue";
import hljs from "highlight.js";
import "highlight.js/styles/default.css";

const props = defineProps<{
  hash: string;
  responseText: Ref<string>;
}>();

const html = computed(() => {
  if (props.responseText == null) return "";
  if (props.responseText.value == "") return "";
  return hljs.highlight(props.responseText as any, { language: "json" }).value;
});

async function copyToClipboard() {
  await writeText(`${props.responseText}`);
}
</script>
