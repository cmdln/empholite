mod actions;
mod types;

use self::types::RecipesPage;
use crate::{
    components::{alert::Context, Alert},
    AppRoute,
};
use bootstrap_rs::{prelude::*, Button, Card, CardBody, Container, Jumbotron};
use shared::Recipe;
use uuid::Uuid;
use yew::{prelude::*, services::fetch::FetchTask};
use yew_router::prelude::*;

pub(crate) struct Home {
    link: ComponentLink<Self>,
    fetch_tsk: Option<FetchTask>,
    state: RecipesPage,
    alert_ctx: Context,
    props: Props,
}

pub(crate) enum Msg {
    Fetch,
    Fetched(String),
    Delete(Uuid),
    Deleted,
    Failure(String),
    ClearAlert,
}

#[derive(Properties, Default, Clone, PartialEq)]
pub(crate) struct Props {
    #[prop_or_default]
    pub(crate) offset: Option<i64>,
}

impl Component for Home {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        link.send_message(Self::Message::Fetch);
        let fetch_tsk = None;
        let state = RecipesPage::default();
        let alert_ctx = Context::default();
        Self {
            link,
            fetch_tsk,
            state,
            alert_ctx,
            props,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        use Msg::*;
        let result = match msg {
            Fetch => self.handle_fetch(),
            Fetched(body) => self.handle_fetched(body),
            Delete(id) => self.handle_delete(id),
            Deleted => self.handle_deleted(),
            Failure(error) => {
                self.alert_ctx = Context::Danger(error);
                Ok(true)
            }
            ClearAlert => {
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

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if render_on_change(&mut self.props, props) {
            self.link.send_message(Self::Message::Fetch);
            true
        } else {
            false
        }
    }

    fn view(&self) -> Html {
        let recipes = &self.state.recipes;
        let view_recipe = move |r: &Recipe| {
            let delete = if let Some(id) = r.id {
                html! {
                    <Button
                        margin=Margin(Edge::Right, 3)
                        color=Color::Secondary
                        on_click=self.link.callback(move |_| Msg::Delete(id.clone()))
                    >
                        { "x" }
                    </Button>
                }
            } else {
                html! {
                    <Button
                        margin=Margin(Edge::Right, 3)
                        color=Color::Secondary
                        disabled=true
                    >
                        { "x" }
                    </Button>
                }
            };
            html! {
                <li class="list-group-item">
                    { delete }
                    <RouterAnchor<AppRoute> route=AppRoute::View(r.id.clone().unwrap().to_string())>
                        { r.url.clone() }
                    </RouterAnchor<AppRoute>>
                </li>
            }
        };
        html! {
            <Container>
                <Jumbotron margin=Margin(Edge::Bottom, 3)>
                    <h1>{ "Empholite" }</h1>
                </Jumbotron>
                <Alert on_close=self.link.callback(|_| Self::Message::ClearAlert) context=self.alert_ctx.clone() />
                { self.view_toolbar() }
                <Card border=Border(Edge::All, Color::Primary)>
                    <CardBody>
                        { self.view_pagination("mb-3") }
                        <ul class="list-group">
                            { for recipes.iter().map(view_recipe) }
                        </ul>
                        { self.view_pagination("mt-3") }
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

    fn view_pagination(&self, class: &str) -> Html {
        html! {
            <div class=format!("btn-toolbar {}", class)>
                <div class="btn-group">
                { self.view_prev_button() }
                { self.view_next_button() }
                </div>
            </div>
        }
    }

    fn view_prev_button(&self) -> Html {
        match show_prev_button(&self.props.offset, self.state.limit) {
            Some(0) => html! {
                <RouterButton<AppRoute> classes="btn btn-secondary" route=AppRoute::Index>
                    { "Previous" }
                </RouterButton<AppRoute>>
            },
            Some(offset) => html! {
                <RouterButton<AppRoute> classes="btn btn-secondary" route=AppRoute::IndexOffset(offset)>
                    { "Previous" }
                </RouterButton<AppRoute>>
            },
            None => html! {
                <Button disabled=true>
                    { "Previous" }
                </Button>
            },
        }
    }

    fn view_next_button(&self) -> Html {
        match show_next_button(&self.props.offset, self.state.limit, self.state.total) {
            Some(offset) => html! {
                <RouterButton<AppRoute> classes="btn btn-secondary" route=AppRoute::IndexOffset(offset)>
                    { "Next" }
                </RouterButton<AppRoute>>
            },
            None => html! {
                <Button disabled=true>
                    { "Next" }
                </Button>
            },
        }
    }
}

fn show_prev_button(offset: &Option<i64>, limit: i64) -> Option<i64> {
    match offset {
        None | Some(0) => None,
        Some(offset) => Some(if offset - limit < 0 {
            0
        } else {
            offset - limit
        }),
    }
}

fn show_next_button(offset: &Option<i64>, limit: i64, total: i64) -> Option<i64> {
    match offset {
        None if total > limit => Some(limit),
        Some(offset) if offset + limit < total => Some(offset + limit),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prev_offset_none() {
        assert_eq!(None, show_prev_button(&None, 25));
    }

    #[test]
    fn test_prev_offset_first() {
        assert_eq!(None, show_prev_button(&Some(0), 25));
    }

    #[test]
    fn test_prev_offset_partial() {
        assert_eq!(Some(0), show_prev_button(&Some(24), 25));
    }

    #[test]
    fn test_prev_offset() {
        assert_eq!(Some(0i64), show_prev_button(&Some(25), 25));
        assert_eq!(Some(1i64), show_prev_button(&Some(26), 25));
    }

    #[test]
    fn test_next_offset_below_total() {
        assert_eq!(Some(25), show_next_button(&None, 25, 26));
        assert_eq!(Some(25), show_next_button(&Some(0), 25, 26));
        assert_eq!(Some(50), show_next_button(&Some(25), 25, 51));
    }

    #[test]
    fn test_next_offset_at_above_total() {
        assert_eq!(None, show_next_button(&None, 25, 25));
        assert_eq!(None, show_next_button(&Some(0), 25, 25));
        assert_eq!(None, show_next_button(&Some(25), 25, 50));
    }
}
