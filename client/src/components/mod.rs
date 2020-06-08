use bootstrap_rs::{prelude::*, Card, CardBody, Container, Jumbotron, TextArea};
use yew::{
    prelude::*,
    services::{fetch::FetchTask, FetchService},
};

pub(crate) struct Home {
    link: ComponentLink<Self>,
    fetch_svc: FetchService,
    fetch_tsk: Option<FetchTask>,
    state: String,
}

pub(crate) struct Msg(String);

impl Component for Home {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let fetch_svc = FetchService::new();
        let fetch_tsk = None;
        let state = String::default();
        Self {
            link,
            fetch_svc,
            fetch_tsk,
            state,
        }
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        false
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
                <Card border_color=BorderColor::Primary>
                    <CardBody>
                        <TextArea name="recipe" on_signal=self.link.callback(|value| Msg(value))>
                        </TextArea>
                        <div>
                            <input type="button">{ "Save" }</input>
                        </div>
                    </CardBody>
                </Card>
            </Container>
        }
    }
}
