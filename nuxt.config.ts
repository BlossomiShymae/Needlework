import path from "path";

// https://nuxt.com/docs/api/configuration/nuxt-config
export default defineNuxtConfig({
  devtools: { enabled: true },
  ssr: false,
  srcDir: "src",
  app: {
    pageTransition: { name: "page", mode: "out-in" },
    head: {
      meta: [
        { name: "viewport", content: "width=device-width, initial-scale=1" },
      ],
      link: [
        {
          rel: "stylesheet",
          href: "/css/bootstrap.min.css",
        },
        {
          rel: "stylesheet",
          href: "/css/application.css",
        },
        {
          rel: "icon",
          type: "image/png",
          href: "/favicon.png",
        },
      ],
      script: [
        {
          src: "/js/bootstrap.bundle.min.js",
        },
      ],
    },
  },
  experimental: {
    payloadExtraction: false,
  },
});
