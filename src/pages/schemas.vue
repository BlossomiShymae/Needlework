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
import { inject } from "vue";
import { Invoker } from "~/composables/invoker";

const invoker = inject(Invoker.Key) as Invoker;

const schemas = await invoker.schemas();

definePageMeta({
  layout: "window",
});
</script>
