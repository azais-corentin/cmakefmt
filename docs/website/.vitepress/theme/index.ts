import { Icon } from "@iconify/vue";
import DefaultTheme from "vitepress/theme";
import BenchmarkCharts from "./BenchmarkCharts.vue";
import HomeFeatures from "./HomeFeatures.vue";

export default {
  extends: DefaultTheme,
  enhanceApp({ app }) {
    app.component("Icon", Icon);
    app.component("HomeFeatures", HomeFeatures);
    app.component("BenchmarkCharts", BenchmarkCharts);
  },
};
