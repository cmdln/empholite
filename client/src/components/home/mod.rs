mod actions;

use bootstrap_rs::{
    input::InputType, prelude::*, Card, CardBody, Container, Input, InputGroup, Jumbotron, TextArea,
};
use serde::Serialize;
use yew::{
    prelude::*,
    services::{fetch::FetchTask, FetchService},
};

#[derive(Serialize, Default)]
struct Recipe {
    url: String,
    payload: String,
}

pub(crate) struct Home {
    link: ComponentLink<Self>,
    fetch_svc: FetchService,
    fetch_tsk: Option<FetchTask>,
    state: Recipe,
    message: String,
}

pub(crate) enum Msg {
    UrlChanged(String),
    RecipeChanged(String),
    Fetch,
    Fetched(String),
    Failure(String),
}

impl Component for Home {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let fetch_svc = FetchService::new();
        let fetch_tsk = None;
        let state = Recipe::default();
        let message = String::default();
        Self {
            link,
            fetch_svc,
            fetch_tsk,
            state,
            message,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        let result = match msg {
            Self::Message::Fetch => self.handle_fetch(),
            Self::Message::Fetched(body) => self.handle_fetched(body),
            Self::Message::UrlChanged(url) => {
                self.state.url = url;
                Ok(true)
            }
            Self::Message::RecipeChanged(payload) => {
                self.state.payload = payload;
                Ok(true)
            }
            Self::Message::Failure(error) => {
                self.message = error;
                Ok(true)
            }
        };
        match result {
            Ok(should_render) => should_render,
            Err(error) => {
                self.message = format!("{}", error);
                true
            }
        }
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let value = "http://test.local:8989/api/foo".to_owned();
        html! {
            <Container>
                <Jumbotron margin=Margin(Edge::Bottom, 3)>
                    <h1>{ "Empholite" }</h1>
                </Jumbotron>
                <Card>
                    <CardBody class="bg-danger">
                        { self.message.clone() }
                    </CardBody>
                </Card>
                <Card border=Border(Edge::All, Color::Primary)>
                    <CardBody>
                        <InputGroup>
                            <Input input_type=InputType::Text value=value on_change=self.link.callback(|value| Msg::UrlChanged(value))/>
                        </InputGroup>
                        <TextArea name="recipe" on_change=self.link.callback(|value| Msg::RecipeChanged(value))>
                        </TextArea>
                        <div class="mt-3">
                            <button
                                class="btn btn-primary"
                                type="button"
                                onclick=self.link.callback(|_| Msg::Fetch)
                            >{ "Save" }</button>
                        </div>
                    </CardBody>
                </Card>
            </Container>
        }
    }
}
