<template>
  <div class="p-2">
    <h5>Schemas</h5>
    <div v-for="schema in schemas" class="border mb-2">
      <Schema
        :name="schema.name"
        :description="schema.description"
        :properties="schema.properties"
        :enum="schema.enum"
        :type="schema.type"
        :schema-ids="schema.schemaIds"
      />
    </div>
  </div>
</template>

<script lang="ts" setup>
import { invoke } from "@tauri-apps/api";

const schemas = (await invoke("get_schemas")) as any;

definePageMeta({
  layout: "window",
});
</script>
