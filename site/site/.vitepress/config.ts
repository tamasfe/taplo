import { defineConfigWithTheme } from "vitepress";
import type { DefaultTheme } from "vitepress";

export default defineConfigWithTheme<DefaultTheme.Config>({
  title: "Taplo",
  description: "A versatile TOML toolkit",
  lastUpdated: true,
  head: [
    ['link', { rel: "icon", type: "image/png", sizes: "32x32", href: "/favicon-32x32.png"}],
    ['link', { rel: "icon", type: "image/svg+xml", href: "/favicon.svg"}],
  ],
  themeConfig: {
    nav: [
      // TODO: unhide editor
      // {
      //   text: "Editor",
      //   activeMatch: "^/editor",
      //   link: "/editor",
      // },
      {
        text: "Command Line",
        activeMatch: "^/cli",
        link: "/cli/introduction",
      },
      {
        text: "Library",
        activeMatch: "^/lib",
        link: "/lib/introduction",
      },
      {
        text: "Configuration",
        items: [
          {
            text: "Configuration File",
            link: "/configuration/file",
          },
          {
            text: "Formatter Options",
            link: "/configuration/formatter-options",
          },
          {
            text: "Directives",
            link: "/configuration/directives",
          },
          {
            text: "Developing Schemas",
            link: "/configuration/developing-schemas",
          },
          {
            text: "Using Schemas",
            link: "/configuration/using-schemas",
          },
        ],
      },
    ],
    socialLinks: [{ icon: "github", link: "https://github.com/tamasfe/taplo" }],
    sidebar: {
      "/cli": [
        {
          text: "About",
          items: [
            {
              text: "Introduction",
              link: "/cli/introduction",
            },
          ],
        },
        {
          text: "Installation",
          items: [
            {
              text: "Binary Releases",
              link: "/cli/installation/binary",
            },
            { text: "Cargo", link: "/cli/installation/cargo" },
            { text: "Homebrew", link: "/cli/installation/homebrew" },
            { text: "NPM", link: "/cli/installation/npm" },
            { text: "Docker", link: "/cli/installation/docker" },
          ],
        },
        {
          text: "Usage",
          items: [
            {
              text: "Configuration",
              link: "/cli/usage/configuration",
            },
            {
              text: "Validation",
              link: "/cli/usage/validation",
            },
            {
              text: "Formatting",
              link: "/cli/usage/formatting",
            },
            {
              text: "Conversion and Extraction",
              link: "/cli/usage/conversion-and-extraction",
            },
            {
              text: "Language Server",
              link: "/cli/usage/language-server",
            },
          ],
        },
      ],
      "/lib": [
        {
          text: "About",
          items: [
            {
              text: "Introduction",
              link: "/lib/introduction",
            },
          ],
        },
        {
          text: "Rust",
          items: [
            {
              text: "Taplo Core",
              link: "/lib/rust/taplo",
            },
            { text: "Taplo LSP", link: "/lib/rust/taplo-lsp" },
            { text: "Taplo Common", link: "/lib/rust/taplo-common" },
          ],
        },
        {
          text: "JavaScript",
          items: [
            {
              text: "Taplo Lib",
              link: "/lib/javascript/lib",
            },
            { text: "Taplo LSP", link: "/lib/javascript/lsp" },
          ],
        },
      ],
      "/configuration": [
        {
          text: "General",
          items: [
            {
              text: "Configuration File",
              link: "/configuration/file",
            },
            {
              text: "Formatter Options",
              link: "/configuration/formatter-options",
            },
            {
              text: "Directives",
              link: "/configuration/directives",
            },
          ],
        },
        {
          text: "JSON Schemas",
          items: [
            {
              text: "Developing Schemas",
              link: "/configuration/developing-schemas",
            },
            {
              text: "Using Schemas",
              link: "/configuration/using-schemas",
            },
          ],
        },
      ],
    },
  },
});
