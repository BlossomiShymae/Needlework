<template>
  <NuxtLayout>
    <NuxtPage />
  </NuxtLayout>
</template>

<script setup lang="ts">
import { Invoker } from "~/composables/invoker";
import { provide } from "vue";

provide(Invoker.Key, new Invoker());
</script>
