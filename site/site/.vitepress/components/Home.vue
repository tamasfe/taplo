<template>
  <div
    class="tw-w-full tw-mt-16 tw-flex tw-flex-col tw-items-center tw-justify-center"
  >
    <div class="tw-flex tw-flex-row tw-items-center tw-justify-center">
      <img :src="TaploIcon" class="tw-w-36 tw-mr-4" />
      <div class="tw-flex-col">
        <div class="tw-font-bold tw-text-6xl tw-text-primary mb-4">Taplo</div>
        <div class="tw-text-2xl">A versatile, feature-rich TOML toolkit.</div>
      </div>
    </div>
    <div class="tw-flex tw-flex-row tw-flex-wrap tw-items-center tw-justify-center tw-gap-4 tw-mt-24">
      <!-- TODO: unhide editor -->
      <!-- <a href="/editor">
        <button class="home-link-button">
          <span v-html="EditorIcon" style="fill: white" />
          <span class="tw-ml-1">Try it here!</span>
        </button>
      </a> -->

      <a
        href="https://marketplace.visualstudio.com/items?itemName=tamasfe.even-better-toml"
      >
        <button class="home-link-button">
          <span v-html="VSCodeIcon" style="fill: white" />
          <span class="tw-ml-1">Visual Studio Code</span>
        </button>
      </a>
      <a href="/cli/introduction">
        <button class="home-link-button">
          <span v-html="ConsoleIcon" style="fill: white" />
          <span class="tw-ml-1">Command Line</span>
        </button>
      </a>
      <a href="/lib/introduction">
        <button class="home-link-button">
          <span v-html="LibraryIcon" style="fill: white" />
          <span class="tw-ml-1">As a library</span>
        </button>
      </a>
    </div>
    <div
      class="tw-flex tw-flex-row tw-gap-4 tw-flex-wrap tw-justify-center tw-items-center tw-mt-12"
      style="width: 60vw"
    >
      <div class="home-info-box">
        <div class="home-info-box-top">Latest TOML Version</div>
        <div class="home-info-box-main">
          <span>
            Supports TOML
            <a href="https://toml.io/en/v1.0.0">1.0.0</a>
            .
          </span>
        </div>
      </div>
      <div class="home-info-box">
        <div class="home-info-box-top">Validation</div>
        <div class="home-info-box-main">
          <span>
            Validate TOML documents syntactically or against JSON schemas.
          </span>
        </div>
      </div>
      <div class="home-info-box">
        <div class="home-info-box-top">Customizable Formatter</div>
        <div class="home-info-box-main">
          Formatter with fine-grained options.
        </div>
      </div>
      <div class="home-info-box">
        <div class="home-info-box-top">Embeddable Language Server</div>
        <div class="home-info-box-main">
          Embeddable language server with features based on JSON schemas.
        </div>
      </div>
      <div class="home-info-box">
        <div class="home-info-box-top">Cross-Platform</div>
        <div class="home-info-box-main">Available wherever Rust compiles.</div>
      </div>
    </div>
  </div>
  <div class="home-curved-bg"></div>
</template>

<script setup lang="ts">
import TaploIcon from "../assets/taplo-icon.svg";
import EditorIcon from "@mdi/svg/svg/code-brackets.svg?raw";
import VSCodeIcon from "@mdi/svg/svg/microsoft-visual-studio-code.svg?raw";
import ConsoleIcon from "@mdi/svg/svg/console.svg?raw";
import LibraryIcon from "@mdi/svg/svg/file-code.svg?raw";
</script>

<style lang="scss">
.home-link-button {
  @apply hover:tw-bg-primary-lighter tw-bg-primary-light tw-transition-colors tw-rounded-2xl tw-p-2.5 tw-text-white tw-flex tw-items-center tw-justify-center;
}

.home-info-box {
  @apply tw-rounded-2xl tw-flex tw-flex-col tw-p-3 tw-shadow-md tw-flex-shrink-0 tw-h-44;

  max-width: 300px;
  background-color: var(--vp-c-bg);

  a {
    @apply tw-text-primary;
  }

  .home-info-box-top {
    @apply tw-flex tw-flex-row tw-items-center tw-justify-between tw-w-full tw-p-2 tw-font-bold;
    border-bottom: 1px solid rgba(0, 0, 0, 0.1);
  }

  .home-info-box-main {
    @apply tw-flex tw-items-center tw-justify-center tw-h-full tw-w-full tw-text-center tw-p-4;
  }
}
</style>

<style lang="scss">
.home-curved-bg {
  z-index: -1;
  position: fixed;
  height: 130vh;
  width: 250vw;
  top: 30vh;
  left: -75vw;
  overflow: hidden;
  pointer-events: none;
  border-radius: 50%;
  background: linear-gradient(rgba(240, 242, 245, 0.5), rgb(240, 242, 245));
}

html.dark {
  .home-curved-bg {
    background: rgb(31, 31, 31);
  }
}
</style>
