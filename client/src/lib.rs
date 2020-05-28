use log::info;
use wasm_bindgen::prelude::*;
use yew::prelude::*;

pub struct Index {}

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
            { "Index" }
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
