use wasm_bindgen::prelude::*;
use sdr2hdr::{embed_icc, icc};

// 当调用者出现 panic 时，将其转换为 JavaScript 的异常
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

// 定义一个函数来处理图像转换
#[wasm_bindgen]
pub fn convert_to_hdr(image_data: &[u8], icc_type: u8) -> Result<Vec<u8>, JsValue> {
    let icc_profile = icc::by_type(icc_type).map_err(|e| JsValue::from_str(&e.to_string()))?;
    let result = embed_icc(image_data, icc_profile).map_err(|e| JsValue::from_str(&e.to_string()))?;
    Ok(result)
}