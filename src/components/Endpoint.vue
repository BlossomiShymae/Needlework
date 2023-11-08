<template>
  <div class="accordion accordion-flush" :id="`accordion-${hash}`">
    <div class="accordion-item">
      <h2 class="accordion-header" :id="`flush-heading-${hash}`">
        <button
          class="accordion-button collapsed p-0 pe-2"
          type="button"
          data-bs-toggle="collapse"
          :data-bs-target="`#flush-collapse-${hash}`"
        >
          <span
            :class="`badge ${bgClass} rounded-0 text-uppercase p-2 px-3 me-3 h-auto`"
            style="width: 70px"
            >{{ method }}</span
          >
          <span class="p-1">
            {{ path }}
          </span>
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
            <SchemaTable :schemas="requestSchemas" class="mb-2">
              <h6>Request Classes</h6>
            </SchemaTable>
            <div v-if="responses" class="mb-2">
              <div class="alert alert-info p-2">
                <h6 class="m-0">
                  Return value:
                  <span class="font-monospace">{{ returnType ?? "None" }}</span>
                </h6>
              </div>
              <SchemaTable :schemas="responseSchemas" class="mb-2">
                <h6>Response Classes</h6>
              </SchemaTable>
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
            </div>
            <ParameterTable :parameters="queryParameters" class="mb-2">
              <h6>Query Parameters</h6>
            </ParameterTable>
            <ParameterTable :parameters="pathParameters" class="mb-2">
              <h6>Path Parameters</h6>
            </ParameterTable>
            <div v-if="requestBody" class="mb-2">
              <h6>Request Body</h6>
              <textarea
                v-model="jsonBody"
                class="form-control"
                rows="3"
              ></textarea>
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
            <div
              v-if="errorMessage"
              class="alert alert-danger border-start rounded-0 border-danger border-4 border-0 mb-2"
            >
              <p
                class="fw-bold m-0 mb-1 border-bottom border-1 border-danger-subtle pb-2"
              >
                <span
                  ><PhWarningCircle
                    weight="fill"
                    color="#800"
                    size="24"
                    class="me-2" /></span
                >Error
              </p>
              <p class="m-0 font-monospace">{{ errorMessage }}</p>
            </div>
            <div v-if="requestUrl && clientInfo" class="mb-2">
              <h6>Request Url</h6>
              <div class="alert alert-secondary mb-1">
                https://{{ clientInfo.url }}{{ requestUrl }}
              </div>
              <h6>Authentication</h6>
              <div class="alert alert-secondary">
                <p class="m-0">
                  <span
                    ><PhLockKey
                      weight="fill"
                      size="16"
                      class="me-2"
                  /></span>
                  {{ clientInfo.authHeader }}
                </p>
                <hr />
                <p class="m-0">
                  <span>
                    <PhLockKeyOpen
                      weight="fill"
                      size="16"
                      class="me-2"
                    />
                  </span>
                  {{
                    clientInfo.authHeader
                      .split(" ")
                      .map((x: any) => {
                        if (x != "Basic") return base64.decode(x);
                        return x;
                      })
                      .join(" ")
                  }}
                </p>
              </div>
            </div>
            <div v-if="responseBody">
              <h6>Response Body</h6>
              <div class="alert alert-secondary overflow-auto mb-1">
                <pre style="max-height: 400px">
<code class="language-json" v-html="html"></code>
                </pre>
                <hr />
                <div class="d-flex justify-content-end">
                  <button
                    class="btn btn-secondary ms-2"
                    @click="copyToClipboard"
                  >
                    <span
                      ><PhClipboard weight="duotone" color="white" side="24"
                    /></span>
                    Copy
                  </button>
                  <button
                    class="btn btn-secondary ms-2"
                    @click="expandResponseBody"
                    :data-bs-target="`#modal-${hash}`"
                    data-bs-toggle="modal"
                  >
                    <span
                      ><PhArrowsOut weight="duotone" color="white" side="24" />
                    </span>
                    Expand
                  </button>
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
  <ResponseModal :hash="hash" :responseText="responseBody ?? ref('')" />
</template>

<script setup lang="ts">
import { ref, inject, computed } from "vue";
import base64 from "base-64";
import { writeText } from "@tauri-apps/api/clipboard";
import { Invoker } from "~/composables/invoker";
import {
  PhGearSix,
  PhBroom,
  PhClipboard,
  PhArrowsOut,
  PhWarningCircle,
  PhLockKeyOpen,
  PhLockKey,
} from "@phosphor-icons/vue";
import hljs from "highlight.js";
import "highlight.js/styles/default.css";

const props = defineProps<{
  method: string;
  path: string;
  description?: string;
  parameters: any[];
  responses?: any;
  summary?: string;
  requestBody?: any;
}>();

const invoker = inject(Invoker.Key) as Invoker;

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
    bgClass = "bg-info";
    break;
  case "head":
    bgClass = "bg-dark";
    break;
  default:
    bgClass = "bg-primary";
    break;
}
const hash =
  props.method +
  props.path.replace("/", "-").replace("{", "-").replace("}", "-");

// Collect query and path parameters
const queryParameters: any[] = [];
const pathParameters: any[] = [];
for (const parameter of props.parameters) {
  parameter.data = null;
  if (parameter.in === "query") {
    queryParameters.push(parameter);
    continue;
  } else if (parameter.in == "path") {
    pathParameters.push(parameter);
    continue;
  }
}

let requestSchemas: any[] = [];
if (props.requestBody != null) {
  const _schema = props.requestBody.content["application/json"]?.schema ?? null;
  if (_schema != null) {
    let key = _schema.$ref;
    if (key != null) {
      let schema = await invoker.schemaByName(key);
      schema.name = key.replace("#/components/schemas/", "");

      if (schema.type === "object") {
        for (const [k, v] of Object.entries(schema.properties)) {
          const ref = (v as any).$ref;
          if (ref != null) {
            const targetType = ref.replace("#/components/schemas/", "");
            schema.properties[k].type = targetType;
          }
        }
      }
      requestSchemas.push(schema);
    }
  }
}

let returnType: any = null;
let returnKey: any = null;
if (props.responses != null) {
  const code = "2XX";
  if (props.responses[code] != null && props.responses[code].content != null) {
    const schema = props.responses[code].content["application/json"].schema;
    if (schema != null) {
      const type = schema.type;
      let key = null;
      if (type === "array" || type === "object") {
        if (type === "array") {
          const ref = schema.items.$ref;
          returnKey = ref;
          if (ref != null) key = ref + "[]";
        } else {
          const ref = schema.properties?.$ref ?? schema.additionalProperties.$ref;
          returnKey = ref;
          key = ref;
        }
      } else if (schema.$ref != null) {
        const ref = schema.$ref;
        returnKey = ref;
        key = ref;
      }

      if (key != null) {
        returnType = key.replace("#/components/schemas/", "");
      } else {
        returnType = type;
      }
    }
  }
}
let responseSchemas: any[] = [];
if (returnKey != null) {
  let schema = await invoker.schemaByName(returnKey);
  responseSchemas.push(schema);
}

const errorMessage = ref(null);
const requestUrl = ref(null);
const jsonBody = ref(undefined);
const responseBody = ref(null);
const clientInfo: any = ref(null);

const html = computed(() => {
  if (responseBody.value == null) return "";
  if (responseBody.value == "") return "";
  return hljs.highlight(responseBody.value, { language: "json" }).value;
});

async function execute() {
  clearMessageData();
  let computedPath = props.path;
  for (const path of pathParameters) {
    if (path.data != null) {
      computedPath = computedPath.replace(
        `{${path.name}}`,
        encodeURIComponent(path.data)
      );
    }
  }
  let query = "";
  for (const path of queryParameters) {
    const char = query.length == 0 ? "?" : "&";
    if (path.data != null) {
      query = query + char + path.name + "=" + encodeURIComponent(path.data);
    }
  }
  computedPath += query;

  // Send a request to the LCU! 💚
  try {
    const data = await invoker.sendRequest(
      props.method,
      computedPath,
      jsonBody.value
    );

    requestUrl.value = computedPath as any;
    responseBody.value = JSON.stringify(data, null, 2) as any;

    // Get request client information. 💻
    clientInfo.value = await invoker.clientInfo();
  } catch (e: any) {
    errorMessage.value = e;
  }
}

async function clear() {
  clearParameterData();
  clearMessageData();
}

async function copyToClipboard() {
  await writeText(`${responseBody.value}`);
}

function expandResponseBody() {}

function clearParameterData() {
  for (const parameter of queryParameters) {
    parameter.data = null;
  }
  for (const parameter of pathParameters) {
    parameter.data = null;
  }
}

function clearMessageData() {
  requestUrl.value = null;
  responseBody.value = null;
  jsonBody.value = undefined;
  errorMessage.value = null;
  clientInfo.value = null;
}
</script>
