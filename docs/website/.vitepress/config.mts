import { readFile } from "node:fs/promises";
import { resolve } from "node:path";
import { defineConfig } from "vitepress";

export default defineConfig({
  base: "/cmakefmt/",
  title: "cmakefmt",
  description: "An opinionated formatter for CMake files",
  head: [
    ["link", { rel: "icon", href: "/cmakefmt/favicon.svg", type: "image/svg+xml" }],
  ],
  themeConfig: {
    logo: {
      light: "/cmakefmt-logo-light.svg",
      dark: "/cmakefmt-logo-dark.svg",
    },
    nav: [
      { text: "Guide", link: "/guide/getting-started" },
      { text: "Configuration", link: "/guide/configuration" },
      { text: "CLI", link: "/guide/cli" },
      { text: "Inline Pragmas", link: "/guide/inline-pragmas" },
      { text: "Benchmarks", link: "/benchmarks" },
      { text: "Playground", link: "/playground" },
    ],
    sidebar: {
      "/guide/": [
        {
          text: "Guide",
          items: [
            { text: "Getting Started", link: "/guide/getting-started" },
            { text: "Configuration", link: "/guide/configuration" },
            { text: "CLI Reference", link: "/guide/cli" },
            { text: "Inline Pragmas", link: "/guide/inline-pragmas" },
          ],
        },
      ],
    },
    socialLinks: [
      { icon: "github", link: "https://github.com/azais-corentin/cmakefmt" },
    ],
  },
  vite: {
    worker: {
      format: "es",
    },
    plugins: [
      {
        name: "serve-wasm-dev",
        configureServer(server) {
          server.middlewares.use((req, res, next) => {
            if (req.url !== "/cmakefmt/cmakefmt-dprint.wasm") return next();

            const wasmPath = resolve(
              __dirname,
              "../../../target/wasm32-unknown-unknown/debug/cmakefmt_dprint.wasm",
            );

            readFile(wasmPath)
              .then((buf) => {
                res.setHeader("Content-Type", "application/wasm");
                res.setHeader("Content-Length", buf.byteLength.toString());
                res.end(buf);
              })
              .catch(() => {
                const msg = "WASM plugin not found. Build it with:\n\n  `mise run build:debug:wasm`\n";
                res.writeHead(404, { "Content-Type": "text/plain" });
                res.end(msg);
              });
          });
        },
      },
    ],
  },
});
