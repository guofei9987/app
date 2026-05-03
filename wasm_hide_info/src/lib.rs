use hide_info::hide_as_img::HideAsImg as InnerHideAsImg;
use hide_info::mirage_tank;
use wasm_bindgen::prelude::*;

fn to_js_error(error: impl ToString) -> JsValue {
    JsValue::from_str(&error.to_string())
}

#[wasm_bindgen]
pub struct HideAsImg {
    inner: InnerHideAsImg,
}

#[wasm_bindgen]
impl HideAsImg {
    #[wasm_bindgen(constructor)]
    pub fn new() -> HideAsImg {
        HideAsImg {
            inner: InnerHideAsImg::new(),
        }
    }

    pub fn encode(&self, bytes_data: &[u8]) -> Result<Vec<u8>, JsValue> {
        self.inner.encode(bytes_data).map_err(to_js_error)
    }

    pub fn decode(&self, data_bytes: &[u8]) -> Result<Vec<u8>, JsValue> {
        self.inner.decode(data_bytes).map_err(to_js_error)
    }
}

#[wasm_bindgen]
pub fn mirage_tank_from_bytes(
    img1_bytes: &[u8],
    img2_bytes: &[u8],
    a: f32,
    b: Option<f32>,
) -> Result<Vec<u8>, JsValue> {
    mirage_tank::mirage_tank_from_bytes(img1_bytes, img2_bytes, a, b).map_err(to_js_error)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hide_as_img_round_trips_bytes() {
        let source = b"hello from wasm_hide_info";
        let hide_as_img = HideAsImg::new();

        let encoded = hide_as_img.encode(source).unwrap();
        let decoded = hide_as_img.decode(&encoded).unwrap();

        assert_eq!(decoded, source);
    }
}
