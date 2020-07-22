mod actions;
mod rule_editor;
mod types;
mod view;

pub(crate) use self::types::Mode;
use crate::{
    components::{alert::Context, Alert},
    Recipe, Rule,
};
use bootstrap_rs::{prelude::*, Card, Container, Jumbotron};
use uuid::Uuid;
use validator::ValidationErrors;
use yew::{prelude::*, services::fetch::FetchTask};

pub(crate) struct Editor {
    link: ComponentLink<Self>,
    fetch_tsk: Option<FetchTask>,
    props: Props,
    state: Recipe,
    mode: Mode,
    alert_ctx: Context,
    errors: Option<ValidationErrors>,
}

pub(crate) enum Msg {
    Edit,
    Cancel,
    Fetch,
    Fetched(String),
    UrlChanged(String),
    PayloadChanged(String),
    Post,
    Posted(String),
    Failure(String),
    ClearAlert,
    AddRule,
    RuleChanged(Rule, usize),
    RemoveRule(usize),
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
        if props.id.is_some() {
            link.send_message(Self::Message::Fetch);
        }
        let fetch_tsk = None;
        let state = Recipe::default();
        let alert_ctx = Context::default();
        let mode = props.mode.clone();
        let errors = None;
        Self {
            link,
            fetch_tsk,
            props,
            state,
            mode,
            alert_ctx,
            errors,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        use Msg::*;
        let result = match msg {
            Edit => self.handle_edit(),
            Cancel => self.handle_cancel(),
            Fetch => self.handle_fetch(),
            Fetched(body) => self.handle_fetched(body),
            Post => self.handle_post(),
            Posted(body) => self.handle_posted(body),
            UrlChanged(url) => self.handle_url_change(url),
            PayloadChanged(payload) => self.handle_payload_change(payload),
            Failure(error) => self.handle_failure(error),
            ClearAlert => {
                self.alert_ctx = Context::None;
                Ok(true)
            }
            AddRule => self.handle_add_rule(),
            RuleChanged(rule, index) => self.handle_rule_changed(rule, index),
            RemoveRule(index) => self.handle_remove_rule(index),
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
                <Alert on_close=self.link.callback(|_| Self::Message::ClearAlert) context=self.alert_ctx.clone() />
                { self.render_breadcrumbs() }
                { self.render_toolbar() }
                <Card border=Border(Edge::All, Color::Primary)>
                { self.render_body() }
                </Card>
            </Container>
        }
    }
}
