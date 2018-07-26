#[macro_use]
extern crate yew;
extern crate failure;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
use failure::Error;
use yew::format::{Json, Nothing};
use yew::prelude::*;
use yew::services::console::ConsoleService;
use yew::services::fetch::{FetchService, FetchTask, Request, Response};

/**
 * Contents for mapping json response
 */
#[derive(Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
struct Card {
    title: String,
    body: String,
    url: String,
}

struct Model {
    fetch: FetchService,
    link: ComponentLink<Model>,
    console: ConsoleService,
    ft: Option<FetchTask>,
    cards: Vec<Card>,
}

#[derive(Debug)]
enum Msg {
    FetchData,
    Ready(Result<Vec<Card>, Error>),
    // Jump(String),
    Ignore,
}

#[derive(PartialEq, Clone, Default)]
struct Props {
    cards: Vec<Card>,
}

impl Component for Model {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let mut m = Model {
            fetch: FetchService::new(),
            link,
            console: ConsoleService::new(),
            ft: None,
            cards: props.cards,
        };
        m.update(Msg::FetchData);
        m
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        self.console.log(&format!("Receiving msg: {:#?}", msg));
        match msg {
            Msg::FetchData => {
                let callback = self.link.send_back(
                    move |response: Response<Json<Result<Vec<Card>, Error>>>| {
                        let (meta, Json(data)) = response.into_parts();
                        if meta.status.is_success() {
                            Msg::Ready(data)
                        } else {
                            Msg::Ignore
                        }
                    },
                );
                match Request::get("/blog_posts").body(Nothing) {
                    Ok(r) => {
                        self.ft = Some(self.fetch.fetch(r, callback));
                    }
                    Err(e) => {
                        self.console.log(&format!("Parse URL error: {:?}", e));
                    }
                }
                false
            }
            Msg::Ready(data) => match data {
                Ok(d) => {
                    self.console.log(&format!("Fetch data success: {:?}", d));
                    self.cards = d;
                    true
                }
                Err(e) => {
                    self.console.log(&format!("Fetch data failed: {:?}", e));
                    false
                }
            },
            // Msg::Jump(url) => {
            //     self.console.log(&url);
            //     false
            // }
            Msg::Ignore => false,
        }
    }
}

fn title(t: &str) -> Html<Model> {
    html! {
        <h1 class="fx-h-center text-primary bg-secondary py-2", onclick=|_| Msg::FetchData,>{t}</h1>
    }
}

fn description(d: &str) -> Html<Model> {
    html!{
        <div class="fx-h-center text-dark pb-2",>
            {d}
        </div>
    }
}

fn cards(contents: Vec<Card>) -> Html<Model> {
    html!{
        <div class="fx-row fx-wrap px-2",>
            {
                for contents.into_iter().map(|c|{
                    html!{
                        <div class="card mx-2",>
                            <div class="card-header",>
                                <a class="card-title h5 c-hand", href={c.url}, target="_blank",>{c.title}</a>
                                // TODO: maybe some tags?
                                // <div class="card-subtitle text-gray",>{"content"}</div>
                            </div>
                            <div class="card-body",>
                            {c.body}
                            </div>
                        </div>
                    }
                })
            }
        </div>
    }
}

impl Renderable<Model> for Model {
    fn view(&self) -> Html<Self> {
        html! {
            // Render your model here
            <div id="root", class="fx-col",></div>
            {title("The Rock")}
            {description("My personal blog. @xbgxwh@outlook.com")}
            {cards(self.cards.clone())}
            <div class="fx-1",></div>
        }
    }
}

fn main() {
    yew::initialize();
    App::<Model>::new().mount_to_body();
    yew::run_loop();
}
