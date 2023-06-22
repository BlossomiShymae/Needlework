<template>
  <div style="padding-left: calc(100vw - 100%)">
    <header
      class="container container-grid sticky-top p-2 bg-accent bg-accent-20"
      style="z-index: 2000; backdrop-filter: blur(4px)"
    >
      <div class="grid-aside w-100">
        <div class="d-flex align-content-center align-items-center w-100">
          <div class="img-fluid me-2">
            <img src="/favicon.png" class="rounded" width="32" height="32" />
          </div>
          <h4 class="fw-light">LCU Infiltrator</h4>
        </div>
      </div>
      <div class="grid-content ps-4 w-100">
        <div class="d-flex justify-content-end align-items-center w-100 h-100">
          <a
            href="https://github.com/BlossomiShymae"
            class="me-2 hover-dim rounded p-2"
          >
            <!-- phosphor icon -->
          </a>
        </div>
      </div>
    </header>
    <main class="container container-grid">
      <aside id="aside-parent">
        <nav
          id="aside-navbar"
          class="sticky-top overflow-y-auto h-100 d-flex flex-column p-3"
        >
          <ul
            class="mb-2 list-unstyled flex-column border-bottom border-accent"
          >
            <!-- Plugin endpoints -->
            <li
              class="mb-1"
              v-for="(_value, key, index) in endpoints"
              :key="index"
            >
              <NuxtLink
                class="btn rounded text-decoration-none hover-dim w-100 text-start"
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
        class="order-1 grid-content ps-4 pb-4 overflow-x-hidden"
      >
        <NuxtPage />
      </article>
    </main>
    <div class="container">
      <footer class="py-3 my-4">
        <p class="text-center text-body-secondary">
          © 2023 - Blossomi Shymae 🌸💔
        </p>
      </footer>
    </div>
  </div>
</template>

<script setup lang="ts">
import { invoke } from "@tauri-apps/api";

const endpoints = await invoke("get_endpoints");
</script>
