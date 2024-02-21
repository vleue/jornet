use serde::{de::DeserializeOwned, Serialize};

#[cfg(target_arch = "wasm32")]
use std::collections::HashMap;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::{JsCast, JsValue};
#[cfg(target_arch = "wasm32")]
use wasm_bindgen_futures::JsFuture;
#[cfg(target_arch = "wasm32")]
use web_sys::{Request, RequestInit, Response};

pub(crate) async fn get<T: DeserializeOwned>(url: &str) -> Option<T> {
    #[cfg(not(target_arch = "wasm32"))]
    let result = ureq::get(url).call().ok().and_then(|r| r.into_json().ok());
    #[cfg(target_arch = "wasm32")]
    let result = request::<(), T>(url, None).await;

    result
}

pub(crate) async fn post<T: Serialize, U: DeserializeOwned>(url: &str, body: T) -> Option<U> {
    #[cfg(not(target_arch = "wasm32"))]
    let result = ureq::post(url)
        .send_json(body)
        .ok()
        .and_then(|r| r.into_json().ok());
    #[cfg(target_arch = "wasm32")]
    let result = request(url, Some(body)).await;

    result
}

#[cfg(target_arch = "wasm32")]
async fn request<B: Serialize, R: DeserializeOwned>(url: &str, body: Option<B>) -> Option<R> {
    let mut headers = HashMap::new();
    let mut opts = RequestInit::new();
    if body.is_some() {
        headers.insert("Content-Type", "application/json");
        opts.method("POST")
            .body(Some(&JsValue::from_str(
                // serializing the body - can't fail
                &serde_json::to_string(&body).unwrap(),
            )))
            // building headers - can't fail
            .headers(&serde_wasm_bindgen::to_value(&headers).unwrap());
    }

    // building the request - can't fail
    let request = Request::new_with_str_and_init(&url, &opts).unwrap();

    // getting the window - can't fail
    let window = web_sys::window().unwrap();
    // can fail on error response
    let resp_value = JsFuture::from(window.fetch_with_request(&request))
        .await
        .ok()?;
    // converting the JsValue to the correct type - can't fail
    let resp: Response = resp_value.dyn_into().unwrap();
    JsFuture::from(resp.json().unwrap())
        .await
        .ok()
        .and_then(|value|
            // can fail if value is not of the correct type
            serde_wasm_bindgen::from_value(value).ok())
}
