<template>
  <div class="p-2">
    <h5>Schemas</h5>
    <div class="input-group input-group-sm mb-2 align-items-center">
      <span class="input-group-text">Filter</span>
      <input type="text" class="form-control" v-model="filter" />
    </div>
    <div v-for="schema in filteredSchemas" class="border mb-2">
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
import { inject, ref, computed } from "vue";
import { Invoker } from "~/composables/invoker";

const invoker = inject(Invoker.Key) as Invoker;

const schemas: any[] = [];
const _schemas = (await invoker.schemas()) as any;
for (const [k, v] of Object.entries(_schemas)) {
  schemas.push(v);
}
const filter = ref("");
const filteredSchemas = computed(() => {
  return schemas.filter((x) => {
    if (filter.value === "") return true;
    return x.name.toLowerCase().indexOf(filter.value.toLowerCase()) > -1;
  });
});

definePageMeta({
  layout: "window",
});
</script>
