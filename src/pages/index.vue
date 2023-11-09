<template>
  <div class="m-2">
    <div class="row mb-4">
      <div class="col-lg-8">
        <p class="fs-2">Welcome to <ApplicationTitle /></p>
        <p class="fs-5 fw-bold text-light">Get started with LCU development by clicking on any endpoints in 
          the left panel.
        </p>
        <div class="d-flex flex-column">
          <NuxtLink
            to="/console"
            class="btn btn-secondary hover-dim text-start mb-3"
          >
            <PhTerminal weight="fill" size="32" class="me-2" />
            Request Console
          </NuxtLink>
          <NuxtLink
            to="/websocket"
            class="btn btn-secondary hover-dim text-start"
          >
            <PhPlugs weight="fill" size="32" class="me-2" />
            WebSocket Data Viewer
          </NuxtLink>
        </div>
      </div>
      <div class="col-lg-4 d-flex justify-content-center align-items-center">
        <div class="img-fluid">
            <img class="rounded-circle" src="/favicon.png"/>
        </div>
      </div>
    </div>
    <div class="row">
      <div class="col">
        <div
          class="alert alert-warning py-1 rounded-0 border-start border-warning border-4 border-0"
        >
          <h5 class="m-0 mt-2">
            <span class="me-2"
              ><PhWarning weight="fill" size="24" /></span
            >Disclaimer
          </h5>
          <p style="font-size: x-small;">
            THE PROGRAM IS PROVIDED “AS IS” WITHOUT WARRANTY OF ANY KIND, EITHER EXPRESS OR IMPLIED, INCLUDING WITHOUT LIMITATION THE IMPLIED WARRANTIES OF MERCHANTABILITY, NONINFRINGMENT, OR OF FITNESS FOR A PARTICULAR PURPOSE. LICENSOR DOES NOT WARRANT THAT THE FUNCTIONS CONTAINED IN THE PROGRAM WILL MEET YOUR REQUIREMENTS OR THAT OPERATION WILL BE UNINTERRUPTED OR ERROR FREE. LICENSOR MAKES NO WARRANTIES RESPECTING ANY HARM THAT MAY BE CAUSED BY MALICIOUS USE OF THIS SOFTWARE. LICENSOR FURTHER EXPRESSLY DISCLAIMS ANY WARRANTY OR REPRESENTATION TO AUTHORIZED USERS OR TO ANY THIRD PARTY.
          </p>
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
                ><PhGithubLogo weight="fill" size="16" /></span
              >Repository
            </ExternalLink>
          </div>
          <div>
            <ExternalLink button href="https://crates.io/crates/irelia">
              <span class="me-2"
                ><PhPackage weight="fill" size="16" /></span
              >crates.io
            </ExternalLink>
          </div>
        </div>

        <h5>OpenAPI</h5>
        <p class="text-secondary">
          Endpoints and schemas are provided by dysolix's
          <ExternalLink href="https://raw.githubusercontent.com/dysolix/hasagi-types/main/swagger.json">
            generated OpenAPI file.
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
