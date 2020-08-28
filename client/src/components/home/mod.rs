mod actions;
mod types;

use self::types::RecipesPage;
use crate::{
    components::{alert::Context, Alert},
    AppRoute,
};
use bootstrap_rs::{prelude::*, Card, CardBody, Container, Jumbotron};
use shared::Recipe;
use yew::{prelude::*, services::fetch::FetchTask};
use yew_router::prelude::*;

pub(crate) struct Home {
    link: ComponentLink<Self>,
    fetch_tsk: Option<FetchTask>,
    state: RecipesPage,
    alert_ctx: Context,
}

pub(crate) enum Msg {
    Fetch,
    Fetched(String),
    Failure(String),
    ClearAlert,
}

impl Component for Home {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        link.send_message(Self::Message::Fetch);
        let fetch_tsk = None;
        let state = RecipesPage::default();
        let alert_ctx = Context::default();
        Self {
            link,
            fetch_tsk,
            state,
            alert_ctx,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        let result = match msg {
            Self::Message::Fetch => self.handle_fetch(),
            Self::Message::Fetched(body) => self.handle_fetched(body),
            Self::Message::Failure(error) => {
                self.alert_ctx = Context::Danger(error);
                Ok(true)
            }
            Self::Message::ClearAlert => {
                self.alert_ctx = Context::None;
                Ok(true)
            }
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
        let recipes = &self.state.recipes;
        html! {
            <Container>
                <Jumbotron margin=Margin(Edge::Bottom, 3)>
                    <h1>{ "Empholite" }</h1>
                </Jumbotron>
                <Alert on_close=self.link.callback(|_| Self::Message::ClearAlert) context=self.alert_ctx.clone() />
                { self.view_toolbar() }
                <Card border=Border(Edge::All, Color::Primary)>
                    <CardBody>
                        <ul class="list-group">
                            { for recipes.iter().map(view_recipe) }
                        </ul>
                    </CardBody>
                </Card>
            </Container>
        }
    }
}

impl Home {
    fn view_toolbar(&self) -> Html {
        html! {
            <div class="btn-toolbar mb-3">
                <div class="btn-group">
                    <RouterButton<AppRoute> classes="btn btn-primary" route=AppRoute::Add>
                        { "Add Recipe" }
                    </RouterButton<AppRoute>>
                </div>
            </div>
        }
    }
}

fn view_recipe(r: &Recipe) -> Html {
    html! {
        <li class="list-group-item">
            <RouterAnchor<AppRoute> route=AppRoute::View(r.id.clone().unwrap().to_string())>
                { r.url.clone() }
            </RouterAnchor<AppRoute>>
        </li>
    }
}
