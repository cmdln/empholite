extern crate serde;
#[macro_use]
extern crate yew;
#[macro_use]
extern crate serde_derive;

use yew::prelude::*;

#[derive(Serialize, Deserialize)]
pub struct Model {
    state: State,
}

#[derive(Serialize, Deserialize, Default)]
pub struct State {
    context: Option<String>,
    message: Option<String>,
}

pub enum Msg {
    DoIt,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Model {
            state: State::default(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::DoIt => true,
        }
    }
}

impl Renderable<Model> for Model {
    fn view(&self) -> Html<Self> {
        html! {
            // Render your model here
            <button onclick=|_| Msg::DoIt,>{ "Click me!" }</button>
        }
    }
}
