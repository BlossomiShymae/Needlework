<template>
  <div>
    <header
      class="container-fluid container-grid sticky-top p-2 bg-accent bg-accent-20"
      style="z-index: 2000; backdrop-filter: blur(4px)"
    >
      <div class="grid-aside w-100">
        <div class="d-flex align-content-center align-items-center w-100 h-100">
          <div class="img-fluid img-thumbnail me-2">
            <img src="/favicon.png" width="32" />
          </div>
          <NuxtLink
            class="btn rounded text-decoration-none hover-dim me-2"
            to="/"
            title="Home"
          >
            <PhHouse weight="fill" color="black" size="24" />
          </NuxtLink>
          <NuxtLink
            class="btn rounded text-decoration-none hover-dim me-2"
            to="/console"
            title="Request Console"
          >
            <PhTerminal weight="fill" color="black" size="24" />
          </NuxtLink>
          <NuxtLink
            class="btn rounded text-decoration-none hover-dim me-2"
            to="/websocket"
            title="WebSocket Data Viewer"
          >
            <PhPlugs weight="fill" color="black" size="24" />
          </NuxtLink>
          <button
            class="btn rounded text-decoration-none hover-dim me-2"
            @click="openSchemasWindow"
            title="Schemas"
          >
            <PhFiles weight="fill" color="black" size="24" />
          </button>
        </div>
      </div>
      <div class="grid-content ps-4 w-100">
        <div class="d-flex justify-content-end align-items-center w-100 h-100">
          <span class="badge bg-dark me-2">{{ appVersion }}</span>
          <ExternalLink
            button
            href="https://github.com/BlossomiShymae/lcu-helper"
            class="me-2 hover-dim rounded p-2 border-0"
            title="GitHub Repository"
          >
            <PhGithubLogo weight="fill" color="black" size="24" />
          </ExternalLink>
          <div class="vr me-2"></div>
          <button
            class="btn btn-secondary hover-dim rounded border-0"
            @click="restartApplication"
            title="Restart Application"
          >
            <PhArrowsClockwise weight="fill" color="black" size="24" />
          </button>
        </div>
      </div>
    </header>
    <main class="container-fluid container-grid">
      <aside id="aside-parent">
        <nav
          id="aside-navbar"
          class="sticky-top overflow-y-auto h-100 d-flex flex-column p-3 pt-0"
        >
          <ul class="mb-2 list-unstyled flex-column">
            <!-- Plugin endpoints -->
            <li v-for="(_value, key, index) in endpoints" :key="index">
              <NuxtLink
                class="btn fw-light rounded text-decoration-none hover-dim w-100 text-start"
                :to="`/endpoint/${key}`"
              >
                {{ key }}
              </NuxtLink>
            </li>
          </ul>
        </nav>
      </aside>
      <article
        id="main-content"
        class="order-1 grid-content ps-4 pb-4 overflow-hidden"
      >
        <slot></slot>
      </article>
    </main>
    <div class="container-fluid">
      <footer
        class="py-3 my-4 d-flex flex-wrap justify-content-between align-items-start border-top"
      >
        <div class="col-md-4 mb-0 text-muted fs-tiny">
          <small
            ><ApplicationTitle /> isn't endorsed by Riot Games and doesn't
            reflect the views or opinions of Riot Games or anyone officially
            involved in producing or managing Riot Games properties. Riot Games,
            and all associated properties are trademarks or registered
            trademarks of Riot Games, Inc.</small
          >
        </div>
        <div
          class="col-md-4 d-flex align-items-center justify-content-center mb-3 mb-md-0 me-md-auto"
        >
          <div class="text-secondary">© 2023 - Blossomi Shymae 🌸💔</div>
        </div>
        <ul class="nav col-md-4 justify-content-end">
          <li class="nav-item rounded hover-dim me-2">
            <ExternalLink
              button
              class="nav-link p-0 px-2 text-muted"
              href="https://hextechdocs.dev/tag/lcu/"
              >Hextech Docs</ExternalLink
            >
          </li>
          <li class="nav-item rounded hover-dim">
            <ExternalLink
              button
              class="nav-link p-0 px-2 text-muted"
              target="_blank"
              href="https://hextechdocs.dev/getting-started-with-the-lcu-api/"
              >Getting Started</ExternalLink
            >
          </li>
        </ul>
      </footer>
    </div>
  </div>
</template>

<script setup>
// Cannot use lang="ts" here for some reason...
import {
  PhFiles,
  PhGithubLogo,
  PhHouse,
  PhPlugs,
  PhArrowsClockwise,
  PhTerminal,
} from "@phosphor-icons/vue";
import { Invoker } from "~/composables/invoker";
import { inject } from "vue";
import { WebviewWindow } from "@tauri-apps/api/window";
import { getVersion } from "@tauri-apps/api/app";

const appVersion = await getVersion();

const invoker = inject(Invoker.Key);

const endpoints = await invoker.endpoints();

async function openSchemasWindow() {
  const webview = new WebviewWindow("schemas-window", {
    url: "/schemas",
  });
  webview.once("tauri://created", () => {
    webview.setTitle("LCU Helper - Schemas");
  });
  webview.once("tauri://error", console.error);
}

async function restartApplication() {
  await invoker.restart_application();
}
</script>
