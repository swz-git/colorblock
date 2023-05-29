import esbuild from "esbuild";
import { wasmLoader } from "esbuild-plugin-wasm";

esbuild.build({
  entryPoints: ["./build/colorblock.js"],
  outdir: "dist",
  bundle: true,
  sourcemap: true,
  minify: true,
  splitting: true,
  format: "esm",
  define: { global: "window" },
  target: ["esnext"],
  plugins: [
    wasmLoader({
      // (Default) Deferred mode copies the WASM binary to the output directory,
      // and then `fetch()`s it at runtime. This is the default mode.
      mode: "deferred",

      // Embedded mode embeds the WASM binary in the javascript bundle as a
      // base64 string. Note this will greatly bloat the resulting bundle
      // (the binary will take up about 30% more space this way)
      mode: "embedded",
    }),
  ],
});
