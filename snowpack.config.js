// Snowpack Configuration File
// See all supported options: https://www.snowpack.dev/reference/configuration
/** @type {import("snowpack").SnowpackUserConfig } */
module.exports = {
  plugins: [
    [
      "@internetarchive/snowpack-files-hash",
      {
        hashFiles: ["js", "css", "svg", "png", "jpg", "webp"],
        exclude: ["snowpack.config.js"],
        hashLength: 12,
        searchImportsIn: ["html", "js"],
      },
    ]
  ],
  optimize: {
    bundle: true,
    minify: true,
    target: 'es2020',
  },
};
