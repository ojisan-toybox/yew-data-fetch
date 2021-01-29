use wasm_bindgen::prelude::*;
use yew::{format::{Json, Nothing}, prelude::*, services::{FetchService, fetch::{Request, Response}}};
use serde::Deserialize;

struct Model {
    link: ComponentLink<Self>,
    data: Option<ResponseData>,
    loading: bool,
    error: Option<String>
}

#[derive(Deserialize, Debug, Clone)]
pub struct ResponseData {
    by : String,
    descendants : i32,
    id : i32,
    kids : Vec<i32>,
    score : i32,
    time : i32,
    title : String,
    type: String,
    url : String
}


enum Msg {
    StartFetchData,
    SuccessFetchData(Result<ResponseData, anyhow::Error>),
    FailFetchData(String)
}



impl Component for Model {
    type Message = Msg;
    type Properties = ();
    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        link.callback(|_| Msg::StartFetchData);

        let endpoint = "https://hacker-news.firebaseio.com/v0/item/8863.json?print=pretty";
        let request = Request::get(endpoint)
        .body(Nothing)
        .expect("Could not build request.");

        let callback = link.callback(
            |response: Response<Json<Result<ResponseData, anyhow::Error>>>| {
                let Json(data) = response.into_body();
                match data {
                    ResponseData => {
                        link.callback(()=> Msg::SuccessFetchData(data));
                    }
                    anyhow::Error => Msg::FailFetchData("error")
                }
            },
        );
        let task = FetchService::fetch(request, callback).expect("failed to start request");
        Self {
            link,
            data: Some(),
            loading: false,
            error: Some(String)
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::SuccessFetchData(response) => {
                match response {
                    Ok(data) => {
                        self.loading = false;
                        self.data = Some(data);
                    }
                    Err(error) => {
                        self.loading = false;
                        self.error = Some(error.to_string());
                    },
                }
                self.fetch_task = None;
                true
            }
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div>
                <button onclick=self.link.callback(|_| Msg::AddOne)>{ "+1" }</button>
                <p>{ self.value }</p>
            </div>
        }
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    App::<Model>::new().mount_to_body();
}