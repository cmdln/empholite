use crate::components::{alert::Context, Alert};
use bootstrap_rs::{prelude::*, Container};
use yew::prelude::*;

pub(crate) struct Error {
    link: ComponentLink<Self>,
    props: Props,
}

#[derive(Properties, Debug, Default, Clone, PartialEq)]
pub(crate) struct Props {
    pub(crate) context: Context,
}

impl Component for Error {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link, props }
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        render_on_change(&mut self.props, props)
    }

    fn view(&self) -> Html {
        html! {
            <Container>
                <Alert on_close=self.link.callback(|_| ()) context=self.props.context.clone() />
            </Container>
        }
    }
}
