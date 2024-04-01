<template>
  <div v-if="schemas.length > 0">
    <slot></slot>
    <div
      v-for="schema in schemas"
      class="mb-1 border-start border-accent border-4 ps-2"
    >
      <p class="fw-bold">{{ schema.name }}</p>
      <table class="table table-hover">
        <thead>
          <tr class="table-dark">
            <th scope="col">Name</th>
            <th scope="col">Type</th>
          </tr>
        </thead>
        <tbody v-if="schema.type === 'object'">
          <tr v-for="(property, name, index) in schema.properties">
            <th scope="row">{{ name }}</th>
            <!-- Check if type is set, if not, $ref should be set -->
            <td>{{ property.type ?? property.$ref }}</td>
          </tr>
        </tbody>
        <tbody v-else>
          <tr>
            <th scope="row">{{ schema.enum }}</th>
            <td>Enum</td>
          </tr>
        </tbody>
      </table>
    </div>
  </div>
</template>

<script lang="ts" setup>
defineProps<{
  schemas: any;
}>();
</script>
