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
            <pre>{{ responseText }}</pre>
          </div>
        </div>
        <div class="modal-footer">
          <button class="btn btn-secondary" @click="copyToClipboard">
            <span
              ><PhClipboard weight="duotone" color="white" side="24"
            /></span>
            Copy
          </button>
          <button type="button" class="btn btn-primary" data-bs-dismiss="modal">
            <span>
              <PhX weight="duotone" color="white" side="24" />
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

const props = defineProps<{
  hash: string;
  responseText: string;
}>();

async function copyToClipboard() {
  await writeText(`${props.responseText}`);
}
</script>
