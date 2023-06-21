<template>
  <div class="p-2">
    <h3>{{ path }}</h3>
  </div>
</template>

<script setup lang="ts">
import { invoke } from "@tauri-apps/api";
const route = useRoute();

const path = route.params.path;
const endpoint = await invoke("get_endpoint", { name: path });
console.log(endpoint);
</script>
