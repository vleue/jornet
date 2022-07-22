use serde::{de::DeserializeOwned, Serialize};

#[cfg(target_arch = "wasm32")]
use std::collections::HashMap;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::{JsCast, JsValue};
#[cfg(target_arch = "wasm32")]
use wasm_bindgen_futures::JsFuture;
#[cfg(target_arch = "wasm32")]
use web_sys::{Request, RequestInit, Response};

pub(crate) async fn get<T: DeserializeOwned>(url: &str) -> T {
    #[cfg(not(target_arch = "wasm32"))]
    let result = ureq::get(url).call().unwrap().into_json().unwrap();
    #[cfg(target_arch = "wasm32")]
    let result = {
        let window = web_sys::window().unwrap();
        let resp_value = JsFuture::from(window.fetch_with_str(url)).await.unwrap();
        let resp: Response = resp_value.dyn_into().unwrap();
        let val = JsFuture::from(resp.json().unwrap()).await.unwrap();
        val.into_serde().unwrap()
    };

    result
}

pub(crate) async fn post<T: Serialize>(url: &str, body: T) {
    #[cfg(not(target_arch = "wasm32"))]
    let _ = ureq::post(url).send_json(body).unwrap();
    #[cfg(target_arch = "wasm32")]
    {
        let mut headers = HashMap::new();
        headers.insert("Content-Type", "application/json");
        let mut opts = RequestInit::new();
        opts.method("POST")
            .body(Some(&JsValue::from_str(
                &serde_json::to_string(&body).unwrap(),
            )))
            .headers(&JsValue::from_serde(&headers).unwrap());

        let request = Request::new_with_str_and_init(&url, &opts).unwrap();

        let window = web_sys::window().unwrap();
        let resp_value = JsFuture::from(window.fetch_with_request(&request))
            .await
            .unwrap();
        let resp: Response = resp_value.dyn_into().unwrap();
        JsFuture::from(resp.json().unwrap()).await.unwrap();
    }
}
