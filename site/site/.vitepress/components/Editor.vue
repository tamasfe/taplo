<template>
  <div
    class="tw-m-16 tw-mt-4 tw-p-4 tw-shadow-lg tw-rounded-lg"
    data-theme="taplo"
  >
    <input
      type="checkbox"
      :model-value="true"
      class="tw-checkbox tw-checkbox-primary"
    />
    <codemirror
      v-if="codeVisible"
      v-model="code"
      placeholder="Insert TOML document here."
      style="height: 80vh"
      :autofocus="true"
      :indent-with-tab="false"
      :tab-size="2"
      :extensions="extensions"
    />
  </div>
</template>

<script setup lang="ts">
// TODO: find a proper way to connect with the LSP web worker.
// import createLspWorker from "../lsp-worker?worker";
import { VueCodemirror } from "vue-codemirror";
import { oneDark } from "@codemirror/theme-one-dark";
import { computed, nextTick, ref, watch } from "vue";
import { toml } from "@codemirror/legacy-modes/mode/toml";
import { StreamLanguage } from "@codemirror/language";
import { useViteDark } from "../hooks/theme";

const codeVisible = ref(true);

const code = ref(`[package]
name = "cargo-example"
version = "0.1.0"
`);

const dark = useViteDark();

watch(dark, async () => {
  codeVisible.value = false;
  await nextTick();
  codeVisible.value = true;
});

const extensions = computed(() => {
  if (dark.value) {
    return [StreamLanguage.define(toml), oneDark];
  } else {
    return [StreamLanguage.define(toml)];
  }
});
</script>
