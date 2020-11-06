<template>
  <div class="taplo-interactive">
    <div v-if="loading">Loading interactive demo...</div>
    <div v-else class="taplo-demo">
      <div class="taplo-demo-row">
        <div class="toml-wrapper">
          <div class="wrapper-title">TOML Input</div>
          <div class="toml-editor">
            <ace-editor
              v-model="taploInput"
              @init="editorInit"
              lang="toml"
              :theme="theme"
            />
          </div>
        </div>
        <div class="toml-wrapper">
          <div class="wrapper-title">Nicely Formatted</div>
          <textarea class="formatted-toml" readonly v-model="formattedToml" />
        </div>
      </div>
    </div>
  </div>
</template>

<script>
import { Taplo } from "@taplo/lib";
import Vue from "vue";

Vue.component("ace-editor", () => import("vue2-ace-editor"))

export default {
  data: function() {
    return {
      loading: false,
      taplo: null,
      editor: null,
      markers: [],
      theme: "solarized_light",
      taploInput: `[package]
   name = "taplo-demo"
 example_string = "fixme
[lib]`,
    };
  },
  async created() {
    this.loading = true;
    this.taplo = await Taplo.initialize();
    this.loading = false;

    this.$root.$on("theme", theme => {
      if (theme == "theme-dark") {
        this.theme = "twilight";
      } else {
        this.theme = "solarized_light";
      }
    });

    this.setMarkers();
  },
  mounted() {
    if (document.body.classList.contains("theme-dark")) {
      this.theme = "twilight";
    } else {
      this.theme = "solarized_light";
    }
  },
  watch: {
    taploInput: function(input) {
      if (!this.editor) {
        return;
      }

      this.setMarkers();
    },
  },
  computed: {
    formattedToml() {
      if (!this.taplo) {
        return "";
      }

      return this.taplo.format(this.taploInput, { ignoreErrors: true });
    },
  },
  methods: {
    editorInit(editor) {
      editor.setAutoScrollEditorIntoView(true);
      editor.setShowPrintMargin(false);
      editor.session.setOption("useWorker", false);
      require("brace/ext/language_tools");
      require("brace/mode/toml");
      require("brace/theme/twilight");
      require("brace/theme/solarized_light");
      this.editor = editor;
      this.setMarkers();
    },
    setMarkers() {
      if (!this.editor || !this.taplo) {
        return;
      }

      for (let i = 0; i < this.markers.length; i++) {
        this.editor.session.removeMarker(this.markers[i]);
      }
      this.$set(this, "markers", []);

      const lintResult = this.taplo.lint(this.taploInput);

      for (let i = 0; i < lintResult.errors.length; i++) {
        const err = lintResult.errors[i];
        this.markers.push(
          this.editor.session.addMarker(
            this.createRange(err.range.start, err.range.end),
            "editor-error",
            "text"
          )
        );
      }
    },
    createRange(start, end) {
      const ace = require("brace");
      const Range = ace.acequire("ace/range").Range;

      const s = this.createPosition(start);
      const e = this.createPosition(end);

      return new Range(s[0], s[1], e[0], e[1]);
    },
    createPosition(pos) {
      let line = 0;
      let character = 1;
      for (let i = 0; i < this.taploInput.length; i++) {
        character += 1;
        const c = this.taploInput[i];

        if (i === pos) {
          break;
        }

        if (c === "\n") {
          line += 1;
          character = 0;
        }
      }

      return [line, character];
    },
  },
};
</script>

<style lang="stylus">
.taplo-interactive
    display: flex
    align-items: center
    justify-content center
    margin-bottom 1.5rem

    .load-button
        color: white
        background-color #3f68bf
        padding: 0.5rem
        border-radius 0.5rem
        box-shadow: 0px 0px 5px 0px rgba(0,0,0,0.75);
        font-size 1.2rem

        cursor pointer

        &:hover
            background-color #4371cc
  .taplo-demo
    display flex
    flex-direction column
    min-width 70vw
    .taplo-demo-row
      display flex
      flex-direction row
      flex-wrap nowrap

    .toml-wrapper
      display flex
      flex-direction column
      width 100%
      height 15rem
      margin-left 1rem
      background-color rgba(0,0,0, 0.02)
      border-radius 0.5rem

      .toml-editor
        height 15rem
        border-radius 0.3rem
        overflow hidden
        margin 1rem
        margin-top 0

      .wrapper-title
        margin 0.5rem
        color var(--text-color-l10) !important
        font-weight bold

      .formatted-toml
        resize: none
        height 100%
        margin 1rem
        margin-top 0
        border none
        color #415e7d
        background-color #fdf6e3

.theme-dark
  .toml-wrapper
      background-color rgba(0,0,0, 0.1)

    .formatted-toml
      background-color #141414 !important
      color #d0d0d0 !important

.editor-error
  position: absolute;
  background-color: rgba(255, 0,0, 0.2)
  z-index 999
</style>
