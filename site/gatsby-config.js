const remarkSlug = require("remark-slug");

module.exports = {
  siteMetadata: {
    title: "Taplo",
    siteUrl: "http://localhost:9000",
  },
  plugins: [
    {
      resolve: `gatsby-plugin-sass`,
      options: {
        implementation: require("sass"),
      },
    },
    "gatsby-plugin-sharp",
    "gatsby-plugin-react-helmet",
    "gatsby-plugin-sitemap",
    {
      resolve: "gatsby-plugin-manifest",
      options: {
        icon: "src/assets/taplo-icon.svg",
      },
    },
    {
      resolve: `gatsby-plugin-mdx`,
      options: {
        defaultLayouts: {
          default: require.resolve("./src/components/doc-page-layout.tsx"),
        },
        remarkPlugins: [remarkSlug],
      },
    },
    "gatsby-transformer-sharp",
    {
      resolve: "gatsby-source-filesystem",
      options: {
        name: "pages",
        path: "./src/pages/",
      },
      __key: "pages",
    },
    {
      resolve: "gatsby-source-filesystem",
      options: {
        name: "schemas",
        path: "./static/schemas",
      },
      __key: "schemas",
    },
    "gatsby-transformer-json",
    {
      resolve: "gatsby-plugin-antd",
      options: {
        style: true,
      },
    },
    {
      resolve: `gatsby-plugin-less`,
      options: {
        lessOptions: {
          modifyVars: {
            "primary-color": "#de591b",
          },
          javascriptEnabled: true,
        },
      },
    },
    {
      resolve: `gatsby-plugin-typegen`,
    },
  ],
};
