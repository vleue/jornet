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
    let result = ureq::get(url).call().unwrap().into_json().ok();
    #[cfg(target_arch = "wasm32")]
    let result = request::<(), T>(url, None).await;

    result
}

pub(crate) async fn post<T: Serialize, U: DeserializeOwned>(url: &str, body: T) -> Option<U> {
    #[cfg(not(target_arch = "wasm32"))]
    let result = ureq::post(url).send_json(body).unwrap().into_json().ok();

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
                &serde_json::to_string(&body).unwrap(),
            )))
            .headers(&JsValue::from_serde(&headers).unwrap());
    }

    let request = Request::new_with_str_and_init(&url, &opts).unwrap();

    let window = web_sys::window().unwrap();
    let resp_value = JsFuture::from(window.fetch_with_request(&request))
        .await
        .unwrap();
    let resp: Response = resp_value.dyn_into().unwrap();
    let val = JsFuture::from(resp.json().unwrap()).await.unwrap();
    val.into_serde().unwrap()
}
