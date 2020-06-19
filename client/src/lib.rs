#![recursion_limit = "1024"]
mod components;
mod types;

#[macro_use]
extern crate validator_derive;

use self::{
    components::{alert::Context, editor::Mode, Editor, Error, Home},
    types::Recipe,
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
        html! {
            <Router<AppRoute>
                render = Router::render(|switch: AppRoute| {
                    match switch {
                        AppRoute::Index => html! { <Home /> },
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
    wasm_logger::init(wasm_logger::Config::default());
    yew::initialize();
    App::<Index>::new().mount_to_body();
    info!("Application initialized, mounted, and started");
    Ok(())
}
