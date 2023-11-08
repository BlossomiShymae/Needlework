<template>
  <div>
    <div class="m-2">
      <h5>Console</h5>
      <div class="row g-2 mb-2">
        <div class="col-md-2">
          <select
            :class="`form-select h-100 fw-bold ${cssClass}`"
            v-model="method"
            @change="changeCssClass"
          >
            <option selected value="get">GET</option>
            <option value="post">POST</option>
            <option value="put">PUT</option>
            <option value="delete">DELETE</option>
            <option value="patch">PATCH</option>
            <option value="head">HEAD</option>
          </select>
        </div>
        <div class="col-md-10">
          <div class="input-group h-100">
            <span class="input-group-text">
              <PhLink weight="regular" size="16" />
            </span>
            <div class="form-floating">
              <input
                v-model="requestPath"
                class="form-control"
                type="text"
                id="request-path"
                placeholder="/lol-summoner/v1/current-summoner"
              />
              <label for="request-path"
                >/lol-summoner/v1/current-summoner</label
              >
            </div>
          </div>
        </div>
      </div>
      <div class="mb-2">
        <h6>Request Body</h6>
        <textarea
          :disabled="isRequestBodyEnabled ? false : true"
          v-model="requestBody"
          class="form-control"
          rows="3"
        ></textarea>
      </div>
      <hr />
      <div class="d-flex justify-content-start mb-2">
        <button
          class="btn btn-outline-danger fw-semibold me-2"
          @click="execute"
          :disabled="!canExecute"
        >
          <span><PhGearSix color="white" size="24" /></span>
          Execute
        </button>
        <button class="btn btn-outline-secondary fw-semibold me-2" @click="clear">
          <span><PhBroom color="white" size="24" /></span>
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
            ><PhWarningCircle weight="fill" color="#800" size="24" class="me-2"
          /></span>
          Error
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
              ><PhLockKey weight="fill" size="16" class="me-1"
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
            <span style="color: #fffd89">
            {{
              clientInfo.authHeader
                .split(" ")
                .map((x: any) => {
                  if (x != "Basic") return base64.decode(x);
                  return x;
                })
                .join(" ")
            }}
            </span>
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
            <button class="btn btn-outline-secondary fw-semibold ms-2" @click="copyToClipboard">
              <span
                ><PhClipboard color="white" side="24"
              /></span>
              Copy
            </button>
            <button
              class="btn btn-outline-secondary fw-semibold ms-2"
              data-bs-target="#modal-console"
              data-bs-toggle="modal"
            >
              <span
                ><PhArrowsOut color="white" side="24" />
              </span>
              Expand
            </button>
          </div>
        </div>
      </div>
    </div>
    <ResponseModal hash="console" :response-text="responseBody" />
  </div>
</template>

<script lang="ts" setup>
import {
  PhBroom,
  PhGearSix,
  PhClipboard,
  PhArrowsOut,
  PhLockKey,
  PhLockKeyOpen,
  PhWarningCircle,
  PhLink,
} from "@phosphor-icons/vue";
import base64 from "base-64";
import { inject, ref, computed } from "vue";
import { Invoker } from "~/composables/invoker";
import { writeText } from "@tauri-apps/api/clipboard";
import hljs from "highlight.js";
import "highlight.js/styles/default.css";

const invoker = inject(Invoker.Key) as Invoker;

const cssClass = ref("bg-primary-subtle");
const errorMessage: Ref<any> = ref(null);
const clientInfo: Ref<any> = ref(null);
const requestUrl: Ref<any> = ref(null);
const requestPath: Ref<any> = ref(null);
const requestBody: Ref<any> = ref(null);
const responseBody: Ref<any> = ref(null);
const html = ref("");
const method: Ref<any> = ref("get");
const isRequestBodyEnabled = computed(() => {
  return method.value === "get" ? false : true;
});
const canExecute = computed(() => {
  const path = requestPath.value;
  if (path == null) return false;
  return path.trim() == "" ? false : true;
});

function changeCssClass() {
  switch (method.value) {
    case "get":
      cssClass.value = "bg-primary-subtle";
      break;
    case "post":
      cssClass.value = "bg-success-subtle";
      break;
    case "put":
      cssClass.value = "bg-warning-subtle";
      break;
    case "delete":
      cssClass.value = "bg-danger-subtle";
      break;
    case "patch":
      cssClass.value = "bg-info-subtle";
      break;
    case "head":
      cssClass.value = "bg-dark-subtle";
      break;
    default:
      cssClass.value = "bg-primary-subtle";
      break;
  }
}

async function execute() {
  const url = encodeURI(requestPath.value);
  clearMessageData();
  try {
    const data = await invoker.sendRequest(
      method.value,
      url,
      requestBody.value
    );

    responseBody.value = JSON.stringify(data, null, 2) as any;
    html.value = hljs.highlight(responseBody.value, { language: "json" }).value;
    clientInfo.value = await invoker.clientInfo();
    requestUrl.value = url;
  } catch (e: any) {
    errorMessage.value = e;
  }
}

async function clear() {
  requestPath.value = null;
  requestBody.value = null;
  clearMessageData();
}

async function copyToClipboard() {
  await writeText(`${responseBody.value}`);
}

function clearMessageData() {
  errorMessage.value = null;
  clientInfo.value = null;
  responseBody.value = null;
  requestUrl.value = null;
}
</script>
