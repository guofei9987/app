# WASM SDR to HDR Converter

This project wraps the `sdr2hdr` Rust crate in WebAssembly to convert SDR images to HDR by embedding ICC profiles.

## Usage

1. Build the WASM package:
   ```
   wasm-pack build --target web --out-dir pkg
   ```

2. Serve the `index.html` file with a local server.

3. Upload an image, select HDR type, and download the converted HDR image.

## Dependencies

- `sdr2hdr`: The core library for embedding ICC profiles.
- `wasm-bindgen`: For WASM bindings.