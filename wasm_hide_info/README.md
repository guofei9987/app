# wasm_hide_info

WebAssembly bindings for the local `hide_info` crate.

## Build

```shell
wasm-pack build --target web
```

## API

- `new HideAsImg()`
- `HideAsImg.encode(bytes: Uint8Array): Uint8Array`
- `HideAsImg.decode(pngBytes: Uint8Array): Uint8Array`
- `mirage_tank_from_bytes(img1Bytes: Uint8Array, img2Bytes: Uint8Array, a: number, b?: number): Uint8Array`
