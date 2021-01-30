use std::any;
use any::Any;
use serde::Deserialize;
use wasm_bindgen::prelude::*;
use yew::{App, Component, ComponentLink, Html, format::{Json, Nothing}, html, services::fetch::{FetchService, FetchTask, Request, Response}};

#[derive(Deserialize, Debug, Clone)]
pub struct Item {
    itemName: String,
    itemPrice: String,
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
    ft: Option<FetchTask>,
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

    fn render_item(&self, item: &Item) -> Html {
        html! {
            <>
                  <div class="left">{ &item.itemName }</div>
                   <div class="right">{ &item.itemPrice }</div>
            </>
        }
    }
}
impl Component for Model {
    type Message = Msg;
    type Properties = ();

    // コンポーネント作成時に呼ばれるライフサイクルメソッド
    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        link.send_message(Msg::StartFetch);

        Self {
            ft: None,
            is_loading: true,
            data: None,
            link,
            error: None,
        }
    }

    fn rendered(&mut self, first_render: bool) {
        if first_render {
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
                log::info!("firee!");
                let request = Request::get(
                    "https://receipten-backend.ojisan.vercel.app/api/get-items?id=JtvoNq7CnSUU6HvB1QPK",
                )
                .body(Nothing)
                .expect("Could not build request.");
        
                // callbackの組み立て
                let callback = self.link.callback(
                    |response: Response<Json<Result<ResponseData, anyhow::Error>>>| {
                        let Json(data) = response.into_body();
        
                        match data {
                            Ok(data) => {
                               Msg::SuccessFetch(data)
                            }
                            Err(_) => {
                                log::info!("{:?}", data);
                                Msg::FailFetch
                            },
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
           <button onclick=self.link.callback(|_| Msg::StartFetch)>{"refetch"}</button>
            </div>
        }
    }
}

// wasm module からのエントリポイント
#[wasm_bindgen(start)]
pub fn run_app() {
    wasm_logger::init(wasm_logger::Config::default());
    App::<Model>::new().mount_to_body(); // til: turbofish
}