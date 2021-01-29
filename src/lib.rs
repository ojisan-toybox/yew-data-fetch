use serde::Deserialize;
use wasm_bindgen::prelude::*;
use yew::{
    format::{Json, Nothing},
    prelude::*,
    services::fetch::{FetchService, FetchTask, Request, Response},
};

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
    SuccessFetchData(Result<ResponseData, anyhow::Error>),
}

#[derive(Debug)]
pub struct Model {
    fetch_task: Option<FetchTask>,
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
                            {for res.data.iter().map(|e| self.renderItem(e)) }
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
        if self.fetch_task.is_some() {
            html! { <p>{ "Fetching data..." }</p> }
        } else {
            html! { <p></p> }
        }
    }
    fn error(&self) -> Html {
        if let Some(ref error) = self.error {
            html! { <p>{ error.clone() }</p> }
        } else {
            html! {}
        }
    }

    fn renderItem(&self, item: &Item) -> Html {
        html! {
            <a class="item" href=format!("https://www.mercari.com/jp/search/?keyword={}", &item.item_name) target="_blank">
                  <div class="left">{ &item.item_name }</div>
                   <div class="right">{ &item.item_price }</div>
            </a>
        }
    }
}
impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let request = Request::get(
            "https://receipten-backend.ojisan.vercel.app/api/get-items?id=JtvoNq7CnSUU6HvB1QPK",
        )
        .body(Nothing)
        .expect("Could not build request.");
        // 2. construct a callback
        let callback = link.callback(
            |response: Response<Json<Result<ResponseData, anyhow::Error>>>| {
                let Json(data) = response.into_body();
                Msg::SuccessFetchData(data)
            },
        );
        let task = FetchService::fetch(request, callback).expect("failed to start request");
        Self {
            fetch_task: Some(task),
            data: None,
            link,
            error: None,
        }
    }
    fn change(&mut self, _props: Self::Properties) -> bool {
        false
    }
    fn update(&mut self, msg: Self::Message) -> bool {
        use Msg::*;

        match msg {
            SuccessFetchData(response) => {
                match response {
                    Ok(data) => {
                        self.data = Some(data);
                    }
                    Err(error) => self.error = Some(error.to_string()),
                }
                self.fetch_task = None;
                true
            }
        }
    }
    fn view(&self) -> Html {
        html! {
            <div class="container">
                { self.fetching() }
                { self.success() }
            </div>
        }
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    App::<Model>::new().mount_to_body();
}