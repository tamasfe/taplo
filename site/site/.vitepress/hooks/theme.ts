import { usePreferredDark } from "@vueuse/core";
import { computed, onBeforeUnmount, ref } from "vue";

export function useViteDark() {
  const preferDark = usePreferredDark();
  const viteDark = ref(
    localStorage.getItem("vitepress-theme-appearance") ?? "auto"
  );

  // Ugly but works.
  const intervalId = setInterval(() => {
    viteDark.value =
      localStorage.getItem("vitepress-theme-appearance") ?? "auto";
  }, 500);

  onBeforeUnmount(() => {
    clearInterval(intervalId);
  });

  return computed(() => {
    if (viteDark.value === "light") {
      return false;
    } else if (viteDark.value === "auto") {
      return preferDark.value;
    } else {
      return true;
    }
  });
}
