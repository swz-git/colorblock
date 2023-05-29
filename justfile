init:
    cd pkg && npm install

build:
    cargo build --release
    wasm-bindgen --out-dir ./pkg/build ./target/wasm32-unknown-unknown/release/colorblock.wasm
    cd pkg && node esbuild
    cd pkg && cp build/colorblock.d.ts dist

serve:
    cd pkg && npm start