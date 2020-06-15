mod actions;

use bootstrap_rs::{prelude::*, Card, CardBody, CardText, Container, Jumbotron};
use shared::Recipe;
use yew::{
    prelude::*,
    services::{fetch::FetchTask, FetchService},
};

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
                debug!("Url changed, {}", url);
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
                            <Input input_type=InputType::Text value=self.state.url.clone() on_change=self.link.callback(|value| Msg::UrlChanged(value))/>
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
