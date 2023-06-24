<template>
  <div class="accordion accordion-flush" :id="`accordion-${name}`">
    <div class="accordion-item">
      <h2 class="accordion-header" :id="`flush-heading-${name}`">
        <button
          class="accordion-button collapsed p-0 pe-2"
          type="button"
          data-bs-toggle="collapse"
          :data-bs-target="`#flush-collapse-${name}`"
        >
          <span class="p-1">{{ name }}</span>
        </button>
      </h2>
      <div
        class="accordion-collapse collapse"
        :id="`flush-collapse-${name}`"
        :data-bs-parent="`#accordion-${name}`"
      >
        <div class="accordion-body">
          <table class="table table-hover">
            <thead>
              <tr class="table-dark">
                <th scope="col">Name</th>
                <th scope="col">Type</th>
              </tr>
            </thead>
            <tbody v-if="type === 'object'">
              <tr v-for="(property, name, index) in properties">
                <th scope="row">{{ name }}</th>
                <td>{{ property.type }}</td>
              </tr>
            </tbody>
            <tbody v-else>
              <tr>
                <th scope="row">{{ enum }}</th>
                <td>Enum</td>
              </tr>
            </tbody>
          </table>
        </div>
      </div>
    </div>
  </div>
</template>

<script lang="ts" setup>
defineProps<{
  name: string;
  description?: string;
  properties?: any;
  enum?: string[];
  type: string;
  schemaIds: string[];
}>();
</script>
