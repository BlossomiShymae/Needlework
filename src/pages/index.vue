<template>
  <h1>Hello world!</h1>
  <p>{{ word }}</p>
</template>

<script lang="ts" setup>
import { invoke } from "@tauri-apps/api";

const word = await invoke("greet", { name: "World" });
</script>
