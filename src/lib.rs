#![recursion_limit = "500"]

use serde::Deserialize;
use serde_json::Value;
use wasm_bindgen::prelude::*;
use yew::{
    format::{Json, Nothing},
    html,
    services::fetch::{FetchService, FetchTask, Request, Response},
    App, Component, ComponentLink, Html,
};

#[derive(Deserialize, Debug, Clone)]

pub enum Msg {
    StartFetch(String),
    SuccessFetch(serde_json::Value),
    FailFetch,
}

#[derive(Debug)]
pub struct Model {
    ft: Option<FetchTask>,
    is_loading: bool,
    data: Option<serde_json::Value>,
    link: ComponentLink<Self>,
    error: Option<String>,
}

pub enum Animal {
    Dog,
    Fox
}

impl Model {
    fn success(&self) -> Html {
        match self.data {
            Some(ref res) => {
                let base_html = html! {
                    <>
                        <p class="sum">{&r"\ empty /"}</p>
                    </>
                };

                match res {
                    Value::Object(map) => {
                        if map.contains_key("message") {
                            let message = map.get("message").unwrap().as_str();
                            html! {
                                <>
                                    <p class="sum">{&r"\ success /"}</p>
                                    <img src={message.unwrap()} />
                                </>
                            }
                        } else if map.contains_key("image") {
                            let image = map.get("image").unwrap().as_str();
                            html! {
                                <>
                                    <p class="sum">{&r"\ success /"}</p>
                                    <img src={image.unwrap()} />

                                </>
                            }
                        } else {
                            base_html
                        }

                    }
                    _ => {
                        base_html
                    }

                }
                    
            }
            None => {
                html! {
                     <>{"none"}</>
                }
            }
        }
    }

    fn fetching(&self) -> Html {
        html! {
            <div>{"fetching"}</div>
        }
    }

    fn fail(&self) -> Html {
        html! {
            <div>{"fail"}</div>
        }
    }
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    // コンポーネント作成時に呼ばれるライフサイクルメソッド
    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        link.send_message(Msg::StartFetch("Dog".to_string()));

        Self {
            ft: None,
            is_loading: true,
            data: None,
            link,
            error: None,
        }
    }

    fn rendered(&mut self, first_render: bool) {
        if first_render {}
    }

    // 親の再レンダリングで呼ばれる
    fn change(&mut self, _props: Self::Properties) -> bool {
        false
    }

    // msg が送られるたびに呼ばれる関数
    fn update(&mut self, msg: Self::Message) -> bool {
        match msg {
            Msg::StartFetch(animal) => {
                let uri = match animal.as_str() {
                    "Dog" => "https://dog.ceo/api/breeds/image/random",
                    "Fox" => "https://randomfox.ca/floof/",
                    _ => "https://dog.ceo/api/breeds/image/random",
                };

                let request = Request::get(
                    uri,
                )
                .body(Nothing)
                .expect("Could not build request.");

                // callbackの組み立て
                let callback = self.link.callback(
                    |response: Response<Json<Result<Value, anyhow::Error>>>| {
                        let Json(data) = response.into_body();

                        match data {
                            Ok(data) => Msg::SuccessFetch(data),
                            Err(_) => {
                                log::info!("{:?}", data);
                                Msg::FailFetch
                            }
                        }
                    },
                );
                let task = FetchService::fetch(request, callback).expect("failed to start request");
                self.is_loading = true;
                self.ft = Some(task)
            }
            Msg::SuccessFetch(response) => {
                self.is_loading = false;
                self.data = Some(response);
            }
            Msg::FailFetch => {
                self.error = Some("error".to_string());
                self.is_loading = false;
            }
        }
        true
    }

    fn view(&self) -> Html {
        html! {
            <div class="container">
                <h2 class="title">{&"Random Dog Or Fox"}</h2>
                <button onclick=self.link.callback(|_| Msg::StartFetch("Dog".to_string()))>{"わんこ"}</button>
                <button onclick=self.link.callback(|_| Msg::StartFetch("Fox".to_string()))>{"きつね"}</button>
                {
                    match (self.is_loading, self.data.as_ref(), self.error.as_ref()) {
                        (true, _, _) => {
                            self.fetching()
                        }
                        (false, Some(ResponseData), None) => {
                            self.success()
                        }
                        (false, None, None) => {
                            self.fail()
                        }
                        (_,_,_)=>{
                            self.fail()
                        }

                    }
                }
            </div>
        }
    }
}

// wasm module からのエントリポイント
#[wasm_bindgen(start)]
pub fn run_app() {
    wasm_logger::init(wasm_logger::Config::default());
    App::<Model>::new().mount_to_body();
}