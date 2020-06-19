mod actions;
mod types;
mod view;

pub(crate) use self::types::Mode;
use crate::components::{alert::Context, Alert};
use bootstrap_rs::{prelude::*, Card, Container, Jumbotron};
use shared::Recipe;
use uuid::Uuid;
use yew::{
    prelude::*,
    services::{fetch::FetchTask, FetchService},
};

pub(crate) struct Editor {
    link: ComponentLink<Self>,
    fetch_svc: FetchService,
    fetch_tsk: Option<FetchTask>,
    props: Props,
    state: Recipe,
    alert_ctx: Context,
}

pub(crate) enum Msg {
    Fetch,
    Fetched(String),
    UrlChanged(String),
    PayloadChanged(String),
    Post,
    Posted(String),
    Failure(String),
}

#[derive(Properties, Debug, Clone)]
pub(crate) struct Props {
    #[prop_or_default]
    pub(crate) id: Option<Uuid>,
    #[prop_or_default]
    pub(crate) mode: Mode,
}

impl Component for Editor {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        link.send_message(Self::Message::Fetch);
        let fetch_svc = FetchService::new();
        let fetch_tsk = None;
        let state = Recipe::default();
        let alert_ctx = Context::default();
        Self {
            link,
            fetch_svc,
            fetch_tsk,
            props,
            state,
            alert_ctx,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        let result = match msg {
            Self::Message::Fetch => self.handle_fetch(),
            Self::Message::Fetched(body) => self.handle_fetched(body),
            Self::Message::Post => self.handle_post(),
            Self::Message::Posted(body) => self.handle_posted(body),
            Self::Message::UrlChanged(url) => self.handle_url_change(url),
            Self::Message::PayloadChanged(payload) => self.handle_payload_change(payload),
            Self::Message::Failure(error) => self.handle_failure(error),
        };
        match result {
            Ok(should_render) => should_render,
            Err(error) => {
                self.alert_ctx = Context::Danger(format!("{}", error));
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
                <Alert context=self.alert_ctx.clone() />
                <Card border=Border(Edge::All, Color::Primary)>
                {
                    if self.props.mode == Mode::View {
                        self.view_view_body()
                    } else {
                        self.view_edit_body()
                    }
                }
                </Card>
            </Container>
        }
    }
}
