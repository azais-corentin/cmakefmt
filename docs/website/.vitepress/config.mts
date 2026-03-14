import { defineConfig } from "vitepress";

export default defineConfig({
  base: "/cmakefmt/",
  title: "cmakefmt",
  description: "An opinionated formatter for CMake files",
  head: [
    ["link", { rel: "icon", href: "/cmakefmt/favicon.svg", type: "image/svg+xml" }],
  ],
  themeConfig: {
    nav: [
      { text: "Guide", link: "/guide/getting-started" },
      { text: "Configuration", link: "/guide/configuration" },
      { text: "CLI", link: "/guide/cli" },
      { text: "Benchmarks", link: "/benchmarks" },
    ],
    sidebar: {
      "/guide/": [
        {
          text: "Guide",
          items: [
            { text: "Getting Started", link: "/guide/getting-started" },
            { text: "Configuration", link: "/guide/configuration" },
            { text: "CLI Reference", link: "/guide/cli" },
          ],
        },
      ],
    },
    socialLinks: [
      { icon: "github", link: "https://github.com/azais-corentin/cmakefmt" },
    ],
  },
});
