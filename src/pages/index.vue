<template>
  <p>{{ info }}</p>
</template>

<script lang="ts" setup>
import { invoke } from "@tauri-apps/api";

const info = await invoke("get_info");
const endpoints = await invoke("get_endpoints");
console.log(endpoints);
</script>
