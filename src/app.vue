<template>
  <NuxtLayout>
    <!-- Use keepalive page caching to greatly improve user experience
       *faster loading for already navigated pages and preserve state* !!! 💜 -->
    <NuxtPage keepalive />
  </NuxtLayout>
</template>

<script setup lang="ts">
import { Invoker } from "~/composables/invoker";
import { provide } from "vue";

provide(Invoker.Key, new Invoker());
</script>
