<template>
  <div class="m-2">
    <p class="fs-2">Welcome to <ApplicationTitle /></p>
    <div class="row">
      <div class="col">
        <div
          class="alert alert-warning py-1 rounded-0 border-start border-warning border-4 border-0"
        >
          <h5 class="m-0 mt-2">
            <span class="me-2"
              ><PhWarning weight="fill" color="black" size="24" /></span
            >Warning
          </h5>
          <p>
            This tool is for testing/educational purposes only. Please be aware
            of
            <ExternalLink href="https://developer.riotgames.com/docs/lol"
              >current game policies</ExternalLink
            >
            and submit a production application when developing an app. Okie
            dokie? {{ `(＾▽＾)` }}
          </p>
        </div>
      </div>
    </div>
    <div class="row gx-3">
      <div class="col-lg-6 mb-3">
        <h3 class="fw-semibold">
          <span class="me-2"
            ><PhCircuitry weight="fill" color="black" size="32" /></span
          >Endpoints
        </h3>
        <p class="text-secondary">
          Available LCU endpoints are listed on the left sidebar. Some endpoints
          may be out of date due to this
          <ExternalLink
            href="https://github.com/MingweiSamuel/lcu-schema/issues/5"
          >
            issue
          </ExternalLink>
          but should relatively be the same.
        </p>
      </div>
      <div class="col-lg-6 mb-3">
        <h3 class="fw-semibold">
          <span class="me-2"
            ><PhGlobe weight="fill" color="black" size="32" /></span
          >Data interfaces
        </h3>
        <div class="d-flex flex-column">
          <NuxtLink
            to="/console"
            class="btn btn-secondary hover-dim text-dark text-start mb-3"
          >
            <PhTerminal weight="fill" color="black" size="32" class="me-2" />
            Request Console
          </NuxtLink>
          <NuxtLink
            to="/websocket"
            class="btn btn-secondary hover-dim text-dark text-start"
          >
            <PhPlugs weight="fill" color="black" size="32" class="me-2" />
            WebSocket Data Viewer
          </NuxtLink>
        </div>
      </div>
    </div>
    <hr />
    <div class="row">
      <div class="col">
        <h5>irelia</h5>
        <p class="text-secondary">
          An LCU binding crate created by AlsoSylv. This application wouldn't be
          possible without her work as it made development totes easier. Thank
          you once again! 💜
        </p>
        <div class="d-flex justify-content-start align-content-center mb-3">
          <div class="me-2">
            <ExternalLink button href="https://github.com/AlsoSylv/Irelia">
              <span class="me-2"
                ><PhGithubLogo weight="fill" color="black" size="16" /></span
              >Repository
            </ExternalLink>
          </div>
          <div>
            <ExternalLink button href="https://crates.io/crates/irelia">
              <span class="me-2"
                ><PhPackage weight="fill" color="black" size="16" /></span
              >crates.io
            </ExternalLink>
          </div>
        </div>

        <h5>OpenAPI</h5>
        <p class="text-secondary">
          Endpoints and schemas are provided by MingweiSamuel's
          <ExternalLink href="http://www.mingweisamuel.com/lcu-schema/tool/">
            hosted OpenAPI file.
          </ExternalLink>
        </p>
      </div>
    </div>
  </div>
</template>

<script lang="ts" setup>
import {
  PhCircuitry,
  PhGlobe,
  PhWarning,
  PhGithubLogo,
  PhPackage,
  PhTerminal,
  PhPlugs,
} from "@phosphor-icons/vue";
</script>
