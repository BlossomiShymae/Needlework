<template>
  <div v-if="parameters.length > 0">
    <slot></slot>
    <table class="table table-hover">
      <thead>
        <tr class="table-dark">
          <th scope="col">Name</th>
          <th scope="col">Value</th>
          <th scope="col">Data Type</th>
        </tr>
      </thead>
      <tbody>
        <tr v-for="parameter in parameters">
          <th scope="row">
            {{ parameter.name }}
            <span v-if="parameter.required" class="badge bg-danger"
              >Required</span
            >
          </th>
          <td><input class="form-control" v-model="parameter.data" /></td>
          <td>
            {{
              parameter.type === "array"
                ? `${parameter.type}[]`
                : parameter.type
            }}
          </td>
        </tr>
      </tbody>
    </table>
  </div>
</template>

<script lang="ts" setup>
defineProps<{
  parameters: any;
}>();
</script>
