<template>
  <div class="p-2">
    <h3>{{ path }}</h3>
    <div class="mt-2" v-for="plugin in endpoint.plugins">
      <Endpoint
        :method="plugin.method"
        :path="plugin.path"
        :parameters="plugin.parameters"
        :description="plugin.description"
        :responses="plugin.responses"
        :summary="plugin.summary"
        :requestBody="plugin.requestBody"
      />
    </div>
  </div>
</template>

<script setup lang="ts">
import { invoke } from "@tauri-apps/api";
const route = useRoute();

const path = route.params.path;
const endpoint: any = await invoke("get_endpoint", { name: path });
console.log(endpoint);
</script>
