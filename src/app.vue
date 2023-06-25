<template>
  <NuxtLayout>
    <!-- Use keepalive page caching to greatly improve user experience
       *faster loading for already navigated pages and preserve state* !!! 💜 -->
    <NuxtPage keepalive />
  </NuxtLayout>
</template>

<style>
.page-enter-active {
  transition: all 0.2s ease-out;
}
.page-leave-active {
  transition: all 0.2s cubic-bezier(1, 0.5, 0.8, 1);
}
.page-enter-from,
.page-leave-to {
  transform: translateX(10%);
  opacity: 0;
}
</style>

<script setup lang="ts">
import { Invoker } from "~/composables/invoker";
import { provide } from "vue";

provide(Invoker.Key, new Invoker());
</script>
