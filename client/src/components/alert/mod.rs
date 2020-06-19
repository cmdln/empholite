use bootstrap_rs::prelude::*;
use yew::prelude::*;

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum Context {
    Success(String),
    Danger(String),
    None,
}

impl Default for Context {
    fn default() -> Self {
        Context::None
    }
}

pub(crate) struct Alert {
    link: ComponentLink<Self>,
    props: Props,
}

#[derive(Properties, Debug, Default, Clone, PartialEq)]
pub(crate) struct Props {
    pub(crate) context: Context,
    pub(crate) on_close: Callback<()>,
}

impl Component for Alert {
    type Properties = Props;
    type Message = ();

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link, props }
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        self.props.on_close.emit(());
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        render_on_change(&mut self.props, props)
    }

    fn view(&self) -> Html {
        match self.props.context {
            Context::None => view_none(),
            Context::Success(ref message) | Context::Danger(ref message) => {
                self.view_context(message)
            }
        }
    }
}

impl Alert {
    fn view_context<S: AsRef<str>>(&self, m: S) -> Html {
        html! {
            <div class={ format!("alert alert-{}", self.as_class()) }>
                { m.as_ref() }
                <button type="button" class="close" data-dismiss="alert" aria-label="Close" onclick=self.link.callback(|_| ())>
                    <span aria-hidden="true">{ "×" }</span>
                </button>
            </div>
        }
    }

    fn as_class(&self) -> &str {
        match self.props.context {
            Context::Success(_) => "success",
            Context::Danger(_) => "danger",
            Context::None => "",
        }
    }
}

fn view_none() -> Html {
    html! {}
}
