<template>
  <div class="p-2">
    <h5>{{ path }}</h5>
    <div class="mt-2 border" v-for="plugin in endpoint.plugins">
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
import { inject } from "vue";
import { Invoker } from "~/composables/invoker";

const invoker = inject(Invoker.Key) as Invoker;
const route = useRoute();

const path = route.params.path;
const endpoint = await invoker.endpointByName(path as string);
</script>
