<template>
  <div class="m-2 h-100">
    <div class="input-group input-group-sm mb-2 align-items-center">
      <button class="btn btn-secondary rounded me-2" @click="clear">
        <PhBroom weight="duotone" color="white" size="24" />
      </button>
      <span class="input-group-text">Filter</span>
      <input type="text" class="form-control me-4" v-model="filter" />
      <div class="form-check form-switch me-4">
        <input
          class="form-check-input"
          type="checkbox"
          role="switch"
          id="attach-switch"
          v-model="isAttached"
        />
        <label class="form-check-label" for="attach-switch">Attach</label>
      </div>
      <div class="form-check form-switch">
        <input
          class="form-check-input"
          type="checkbox"
          role="switch"
          id="tail-switch"
          v-model="isTailing"
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
          v-for="event in filteredEvents"
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
  overscroll-behavior-y: contain;
  scroll-snap-type: y mandatory;
}
.content {
  height: 95%;
  position: absolute;
}

.content > button:last-child {
  scroll-snap-align: v-bind(snap);
}
</style>

<script lang="ts" setup>
import { PhBroom } from "@phosphor-icons/vue";
import { ref, Ref, computed } from "vue";
import { listen } from "@tauri-apps/api/event";

const events: Ref<any[]> = ref([]);
const filter = ref("");
const filteredEvents = computed(() => {
  return events.value.filter((x) => {
    if (filter.value === "") return true;
    const { timestamp, eventType, uri } = x.payload as any;
    return (
      `${timestamp} ${eventType} ${uri}`
        .toLowerCase()
        .indexOf(filter.value.toLowerCase()) > -1
    );
  });
});
const isAttached = ref(true);
const isTailing = ref(false);
const snap = computed(() => {
  return isTailing.value ? "end" : "inherit";
});

await listen("lcu-ws-event", (event: any) => {
  if (isAttached.value) events.value.push(event);
});

function getTimestamp(milliseconds: number) {
  return new Date(milliseconds).toISOString().slice(11, -1);
}

function clear() {
  events.value = [];
}
</script>
