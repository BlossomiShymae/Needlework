<template>
  <div class="m-2">
    <h5>Console</h5>
    <h6>Request Url</h6>
    <select class="form-select mb-2" v-model="method">
      <option selected value="get">GET</option>
      <option value="post">POST</option>
      <option value="put">PUT</option>
      <option value="delete">DELETE</option>
      <option value="patch">PATCH</option>
      <option value="head">HEAD</option>
    </select>
    <input class="mb-2 form-control" v-model="requestPath" />
    <div class="mb-2">
      <h6>Request Body</h6>
      <textarea v-model="requestBody" class="form-control" rows="3"></textarea>
    </div>
    <hr />
    <div class="d-flex justify-content-start mb-2">
      <button class="btn btn-danger fw-semibold me-2" @click="execute">
        <span><PhGearSix weight="duotone" color="white" size="24" /></span>
        Execute
      </button>
      <button class="btn btn-secondary fw-semibold me-2" @click="clear">
        <span><PhBroom weight="duotone" color="white" size="24" /></span>
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
            ><PhLockKey weight="fill" color="black" size="16" class="me-2"
          /></span>
          {{ clientInfo.authHeader }}
        </p>
        <hr />
        <p class="m-0">
          <span>
            <PhLockKeyOpen weight="fill" color="black" size="16" class="me-2" />
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
        <pre style="max-height: 400px">{{ responseBody }}</pre>
        <hr />
        <div class="d-flex justify-content-end">
          <button class="btn btn-secondary ms-2" @click="copyToClipboard">
            <span
              ><PhClipboard weight="duotone" color="white" side="24"
            /></span>
            Copy
          </button>
          <button
            class="btn btn-secondary ms-2"
            data-bs-target="#modal-console"
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
  <div
    class="modal fade"
    tabindex="-1"
    id="modal-console"
    style="z-index: 10000"
  >
    <div class="modal-dialog modal-fullscreen">
      <div class="modal-content">
        <div class="modal-header">
          <h5 class="modal-title">Response Body</h5>
          <button
            type="button"
            class="btn-close"
            data-bs-dismiss="modal"
          ></button>
        </div>
        <div class="modal-body">
          <div class="alert alert-secondary">
            <pre>{{ responseBody }}</pre>
          </div>
        </div>
        <div class="modal-footer">
          <button class="btn btn-secondary" @click="copyToClipboard">
            <span
              ><PhClipboard weight="duotone" color="white" side="24"
            /></span>
            Copy
          </button>
          <button type="button" class="btn btn-primary" data-bs-dismiss="modal">
            <span>
              <PhX weight="duotone" color="white" side="24" />
            </span>
            Close
          </button>
        </div>
      </div>
    </div>
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
  PhX,
} from "@phosphor-icons/vue";
import base64 from "base-64";
import { invoke } from "@tauri-apps/api";
import { writeText } from "@tauri-apps/api/clipboard";
import { ref } from "vue";

const errorMessage: Ref<any> = ref(null);
const clientInfo: Ref<any> = ref(null);
const requestUrl: Ref<any> = ref(null);
const requestPath: Ref<any> = ref(null);
const requestBody: Ref<any> = ref(null);
const responseBody: Ref<any> = ref(null);
const method: Ref<any> = ref("get");

async function execute() {
  const url = encodeURI(requestPath.value);
  clearMessageData();
  try {
    console.log(url);
    const data: any = await invoke("send_request", {
      method: method.value,
      path: url,
      body:
        requestBody.value != null
          ? JSON.stringify(JSON.parse(requestBody.value))
          : null,
    });

    responseBody.value = JSON.stringify(data, null, 2) as any;
    clientInfo.value = await invoke("get_client_info");
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