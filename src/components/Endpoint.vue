<template>
  <div class="accordion accordion-flush" :id="`accordion-${hash}`">
    <div class="accordion-item">
      <h2 class="accordion-header" :id="`flush-heading-${hash}`">
        <button
          class="accordion-button collapsed"
          type="button"
          data-bs-toggle="collapse"
          :data-bs-target="`#flush-collapse-${hash}`"
        >
          <span :class="`badge ${bgClass} text-uppercase p-2 px-3 me-3`">{{
            method
          }}</span>
          {{ path }}
        </button>
      </h2>
      <div
        :id="`flush-collapse-${hash}`"
        class="accordion-collapse collapse"
        :data-bs-parent="`#accordion-${hash}`"
      >
        <div class="accordion-body">
          <div class="">
            <div v-if="summary" class="mb-2">
              <h6>Summary</h6>
              <p>{{ summary }}</p>
            </div>
            <div v-if="description" class="mb-2">
              <h6>Description</h6>
              <p>{{ description }}</p>
            </div>
            <div v-if="responses" class="mb-2">
              <h6>Responses</h6>
              <table class="table table-hover">
                <thead>
                  <tr class="table-dark">
                    <th scope="col">Code</th>
                    <th scope="col">Description</th>
                  </tr>
                </thead>
                <tbody>
                  <tr v-for="(response, key, index) in responses">
                    <th scope="row">{{ key }}</th>
                    <td>{{ response.description }}</td>
                  </tr>
                </tbody>
              </table>
              <h6>Return value</h6>
            </div>
            <hr />
            <div class="d-flex justify-content-start mb-2">
              <button class="btn btn-danger fw-semibold me-2" @click="execute">
                <span
                  ><PhGearSix weight="duotone" color="white" size="24"
                /></span>
                Execute
              </button>
              <button class="btn btn-secondary fw-semibold me-2" @click="clear">
                <span>
                  <PhBroom weight="duotone" color="white" size="24" />
                </span>
                Clear
              </button>
            </div>
            <div v-if="response">
              <h6>Response Body</h6>
              <div class="alert alert-secondary">
                <pre>{{ response }}</pre>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api";
import { PhGearSix, PhBroom } from "@phosphor-icons/vue";

const props = defineProps<{
  method: string;
  path: string;
  description?: string;
  parameters: any[];
  responses?: any;
  summary?: string;
  requestBody?: any;
}>();

let bgClass = "";
switch (props.method) {
  case "get":
    bgClass = "bg-primary";
    break;
  case "post":
    bgClass = "bg-success";
    break;
  case "put":
    bgClass = "bg-warning";
    break;
  case "delete":
    bgClass = "bg-danger";
    break;
  case "patch":
    bgClass = "bg-secondary";
    break;
  default:
    bgClass = "bg-primary";
    break;
}

const hash =
  props.method +
  props.path.replace("/", "-").replace("{", "-").replace("}", "-");

const response = ref(null);

async function execute() {
  const data: any = await invoke("send_request", {
    method: props.method,
    path: props.path,
  });
  response.value = JSON.stringify(data, null, 2) as any;
  console.log(response.value);
}

async function clear() {
  response.value = null;
}
</script>
