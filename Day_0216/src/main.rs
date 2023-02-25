use std::ops::Deref;

use gloo::console::log;

use reqwasm::http::Request;

use serde::{Deserialize, Serialize};
use serde_json::json;
use yew::classes;
use yew::events::InputEvent;
use yew::prelude::*;

use web_sys::HtmlInputElement;

#[derive(Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct User {
    pub username: String,
    pub id: i64,
}

#[derive(Serialize, Deserialize)]
struct ResponseData {
    data: User,
}

#[derive(Serialize, Deserialize, Debug)]
struct Msg {
    id: i64,
    user_id: i64,
    text: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct ResMsg {
    data: Vec<Msg>,
}

#[function_component]
fn App() -> Html {
    let id = use_state(|| "".to_string());
    let pw = use_state(|| "".to_string());

    let oninput_id = Callback::from({
        let id = id.clone();
        move |input_event: InputEvent| {
            let target: HtmlInputElement = input_event.target_dyn_into().unwrap();

            id.set(target.value());
        }
    });
    let oninput_pw = Callback::from({
        let pw = pw.clone();
        move |input_event: InputEvent| {
            let target: HtmlInputElement = input_event.target_dyn_into().unwrap();

            pw.set(target.value());
        }
    });

    let user = use_state(|| User::default());
    let _cloned_user = user.clone();
    let __cloned_user = user.clone();

    let onsubmit = Callback::from({
        move |form_event: SubmitEvent| {
            let id = id.clone();
            let pw = pw.clone();
            let cloned_user = _cloned_user.clone();
            form_event.prevent_default();

            log!("Submit!".to_string());

            log!(id.to_string());
            log!(pw.to_string());

            // HTTP GET
            wasm_bindgen_futures::spawn_local(async move {
                let mut user = cloned_user.deref().clone();
                let response = Request::post("http://localhost:8000/auth")
                    .header("content-type", "application/json")
                    .body(
                        json!({
                            "username": id.to_string(),
                        })
                        .to_string(),
                    )
                    .send()
                    .await
                    .unwrap()
                    .json::<User>()
                    .await
                    .unwrap();

                user.username = response.username;
                user.id = response.id;

                cloned_user.set(user);
            })
        }
    });

    let msg = use_state(|| Vec::<Msg>::new());
    let _clone_msg = msg.clone();

    let comment = use_state(|| "".to_string());
    let oninput_comment = Callback::from({
        let comment = comment.clone();
        move |input_event: InputEvent| {
            let target: HtmlInputElement = input_event.target_dyn_into().unwrap();

            comment.set(target.value());
        }
    });
    let onsubmit_comment = Callback::from({
        move |form_event: SubmitEvent| {
            form_event.prevent_default();
            let user = __cloned_user.clone();
            let comment = comment.clone();
            let msg = _clone_msg.clone();

            log!("Comment!".to_string());

            // HTTP GET
            wasm_bindgen_futures::spawn_local(async move {
                Request::post("http://localhost:8000/msg")
                    .header("content-type", "application/json")
                    .body(
                        json!({
                            "user_id": user.id,
                            "text": *comment,
                        })
                        .to_string(),
                    )
                    .send()
                    .await
                    .unwrap()
                    .json::<Msg>()
                    .await
                    .unwrap();

                let response = Request::get(&*format!("http://localhost:8000/{}", user.id))
                    .header("content-type", "application/json")
                    .send()
                    .await
                    .unwrap()
                    .json::<ResMsg>()
                    .await
                    .unwrap();

                msg.set(response.data);
            })
        }
    });

    html! {
        <div class={classes!("main")}>
        if user.deref().clone().eq(&User::default()) {
                <form class={classes!("form")} {onsubmit}>
                    <div class={classes!("title")}>{"Rust Tutorial"}</div>
                    <input class={classes!("input")} type="text" placeholder="Please input ID" oninput={oninput_id} />
                    <input class={classes!("input")} type="password" placeholder="Please input PW" oninput={oninput_pw} />
                    <button class={classes!("enter")}>{"입장"}</button>
                </form>
            } else {
                <div class={classes!("container")}>
                    <div class={classes!("container__title")}>{&*user.username}{"-"}{&user.id}</div>
                    <ul class={classes!("container__list")}>
                        {
                            msg.deref().clone().into_iter().map(|m| {
                                html! {<li key={&*m.text} class={classes!("container__item")}>{&*m.text}</li>}
                        }).collect::<Html>()
                    }
                    </ul>
                    <form class={classes!("container__form")} onsubmit={onsubmit_comment}>
                        <input class={classes!("container__input")} type="text" oninput={oninput_comment} />
                        <button class={classes!("container__button")}>{"입력"}</button>
                    </form>
                </div>
            }
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
