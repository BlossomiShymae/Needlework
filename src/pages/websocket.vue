<template>
  <div class="m-2 h-100">
    <div class="input-group input-group-sm mb-2 align-items-center">
      <button class="btn btn-secondary rounded me-2" @click="clear">
        <PhBroom weight="duotone" color="white" size="24" />
      </button>
      <span class="input-group-text">Filter</span>
      <input type="text" class="form-control me-4" />
      <div class="form-check form-switch me-4">
        <input
          class="form-check-input"
          type="checkbox"
          role="switch"
          id="attach-switch"
        />
        <label class="form-check-label" for="attach-switch">Attach</label>
      </div>
      <div class="form-check form-switch">
        <input
          class="form-check-input"
          type="checkbox"
          role="switch"
          id="tail-switch"
        />
        <label class="form-check-label" for="tail-switch">Tail</label>
      </div>
    </div>
    <div
      class="alert alert-secondary overflow-y-auto overflow-x-hidden parent-content p-0"
    >
      <div class="content list-group w-100">
        <button
          type="button"
          class="list-group-item list-group-item-action"
          v-for="event in events"
          :key="event"
        >
          <span class="text-primary-subtle me-2">{{
            `${getTimestamp(event.payload.timestamp)}`
          }}</span>
          <span class="text-uppercase me-2">{{
            `${event.payload.eventType}`
          }}</span>
          <span>{{ `${event.payload.uri}` }}</span>
        </button>
      </div>
    </div>
  </div>
</template>

<style scoped>
.parent-content {
  height: 95%;
  position: relative;
}
.content {
  height: 95%;
  position: absolute;
}
</style>

<script lang="ts" setup>
import { PhBroom } from "@phosphor-icons/vue";
import { ref, Ref, computed } from "vue";
import { listen } from "@tauri-apps/api/event";

const events: Ref<any[]> = ref([]);

await listen("lcu-ws-event", (event: any) => {
  console.log(event);
  events.value.push(event);
});

function getTimestamp(milliseconds: number) {
  return new Date(milliseconds).toISOString().slice(11, -1);
}

function clear() {
  events.value = [];
}
</script>
