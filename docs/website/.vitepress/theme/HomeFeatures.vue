<script setup>
import { Icon } from "@iconify/vue";

const features = [
  {
    icon: "lucide:circle-check-big",
    title: "Opinionated Defaults",
    description:
      "Sensible formatting out of the box. Zero config required — just run it.",
  },
  {
    icon: "lucide:sliders-horizontal",
    title: "44 Configuration Options",
    description:
      "Fine-grained control when you need it. Tune line width, casing, alignment, sorting, and more.",
  },
  {
    icon: "lucide:plug",
    title: "dprint Integration",
    description:
      "Use as a standalone CLI or as a dprint WASM plugin for seamless editor and CI integration.",
  },
  {
    icon: "lucide:zap",
    title: "Fast",
    description:
      "Written in Rust. Handles large codebases efficiently with a single-pass formatting pipeline.",
  },
];
</script>

<template>
  <div class="vp-doc">
    <div class="features">
      <div v-for="feature in features" :key="feature.title" class="feature">
        <div class="icon">
          <Icon :icon="feature.icon" width="24" height="24" />
        </div>
        <h3>{{ feature.title }}</h3>
        <p>{{ feature.description }}</p>
      </div>
    </div>
  </div>
</template>

<style scoped>
.features {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(240px, 1fr));
  gap: 20px;
  padding: 0 24px 48px;
  max-width: 1152px;
  margin: 0 auto;
}

.feature {
  padding: 24px;
  border-radius: 12px;
  background-color: var(--vp-c-bg-soft);
}

.feature .icon {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 48px;
  height: 48px;
  border-radius: 8px;
  background-color: var(--vp-c-default-soft);
  margin-bottom: 16px;
  color: var(--vp-c-brand-1);
}

.feature h3 {
  font-size: 16px;
  font-weight: 600;
  margin-bottom: 8px;
}

.feature p {
  font-size: 14px;
  color: var(--vp-c-text-2);
  line-height: 1.6;
}
</style>
