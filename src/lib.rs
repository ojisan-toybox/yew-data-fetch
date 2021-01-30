use serde::Deserialize;
use wasm_bindgen::prelude::*;
use yew::{App, Component, ComponentLink, Html, format::{Json, Nothing}, html,Properties, services::fetch::{FetchService, Request, Response}};

#[derive(Clone, Debug, Eq, PartialEq, Properties)]
pub struct Props {
    pub id: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Item {
    item_name: String,
    item_price: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ResponseData {
    data: Vec<Item>,
    sum: i32
}

#[derive(Debug)]
pub enum Msg {
    StartFetch,
    SuccessFetch(ResponseData),
    FailFetch
}

#[derive(Debug)]
pub struct Model {
    is_loading: bool,
    data: Option<ResponseData>,
    link: ComponentLink<Self>,
    error: Option<String>,
}

impl Model {
    fn success(&self) -> Html {
        match self.data {
            Some(ref res) => {
                html! {
                    <>
                            {for res.data.iter().map(|e| self.render_item(e)) }
                            <p class="sum">{"小計: "}{res.sum}<span>{"円"}</span></p>
                    </>
                }
            }
            None => {
                html! {
                     <></>
                }
            }
        }
    }

    fn fetching(&self) -> Html {
        html! {
            <div>{"fetching"}</div>
        }
    }

    fn render_item(&self, item: &Item) -> Html {
        html! {
            <>
                  <div class="left">{ &item.item_name }</div>
                   <div class="right">{ &item.item_price }</div>
            </>
        }
    }
}
impl Component for Model {
    type Message = Msg;
    type Properties = ();

    // コンポーネント作成時に呼ばれるライフサイクルメソッド
    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let request = Request::get(
            "https://receipten-backend.ojisan.vercel.app/api/get-items?id=JtvoNq7CnSUU6HvB1QPK",
        )
        .body(Nothing)
        .expect("Could not build request.");

        // callbackの組み立て
        let callback = link.callback(
            |response: Response<Json<Result<ResponseData, anyhow::Error>>>| {
                let Json(data) = response.into_body();
                match data {
                    Ok(data) => {
                       Msg::SuccessFetch(data)
                    }
                    Err(_) => Msg::FailFetch,
                }
            },
        );

        FetchService::fetch(request, callback);
        Self {
            is_loading: false,
            data: None,
            link,
            error: None,
        }
    }

    // 親の再レンダリングで呼ばれる
    fn change(&mut self, _props: Self::Properties) -> bool {
        false
    }

    // msg が送られるたびに呼ばれる関数
    fn update(&mut self, msg: Self::Message) -> bool {
        match msg {
            Msg::StartFetch => {
                self.is_loading = true;
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
           {
            if  self.is_loading {
                html! {
                    self.fetching()
                }
              } else {
                html! {
                    self.success() 
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
    App::<Model>::new().mount_to_body(); // til: turbofish
}