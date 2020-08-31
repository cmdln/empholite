#![recursion_limit = "1024"]
mod components;
mod prelude;
mod types;

#[macro_use]
extern crate validator_derive;

use self::{
    components::{alert::Context, editor::Mode, Editor, Error, Home},
    types::{HttpVerb, Recipe, Rule, RuleType},
};
use log::info;
use uuid::Uuid;
use wasm_bindgen::prelude::*;
use yew::prelude::*;
use yew_router::{prelude::*, switch::Permissive, Switch};

#[derive(Debug, Switch, Clone, PartialEq)]
pub(crate) enum AppRoute {
    #[to = "/page-not-found"]
    PageNotFound(Permissive<String>),
    #[to = "/add"]
    Add,
    #[to = "/view/{url}"]
    View(String),
    #[to = "/offset/{offset}"]
    IndexOffset(i64),
    #[to = "/"]
    Index,
}

pub(crate) struct Index {}

impl Component for Index {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        // TODO add route for offset
        html! {
            <Router<AppRoute>
                render = Router::render(|switch: AppRoute| {
                    match switch {
                        AppRoute::Index => html! { <Home /> },
                        AppRoute::IndexOffset(offset) => html! { <Home offset=offset /> },
                        AppRoute::Add=> html! { <Editor mode=Mode::Edit /> },
                        AppRoute::View(id) =>
                            if let Ok(id) = id.parse::<Uuid>() {
                                html! { <Editor id=id /> }
                            } else {
                                html! { <Error context=Context::Danger(format!("Could not parse ID, {}", id)) /> }
                            }
                        ,
                        AppRoute::PageNotFound(Permissive(None)) => html!{"Page not found"},
                        AppRoute::PageNotFound(Permissive(Some(missed_route))) => html!{format!("Page '{}' not found", missed_route)}
                    }
                })
                redirect = Router::redirect(|route: Route| {
                    AppRoute::PageNotFound(Permissive(Some(route.route)))
                })
            />
        }
    }
}

#[wasm_bindgen]
pub fn run_app() -> std::result::Result<(), JsValue> {
    #[cfg(debug_assertions)]
    let config = wasm_logger::Config::new(log::Level::Debug);
    #[cfg(not(debug_assertions))]
    let config = wasm_logger::Config::new(log::Level::Info);
    wasm_logger::init(config);
    yew::initialize();
    App::<Index>::new().mount_to_body();
    info!("Application initialized, mounted, and started");
    Ok(())
}
