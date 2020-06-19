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
    context: Context,
}

#[derive(Properties, Debug, Default, Clone)]
pub(crate) struct Props {
    pub(crate) context: Context,
}

impl Component for Alert {
    type Properties = Props;
    type Message = ();

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let Props { context } = props;
        Self { link, context }
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        self.context = Context::None;
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.context = props.context;
        true
    }

    fn view(&self) -> Html {
        match self.context {
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
                    <span aria-hidden="true">{ "x" }</span>
                </button>
            </div>
        }
    }

    fn as_class(&self) -> &str {
        match self.context {
            Context::Success(_) => "success",
            Context::Danger(_) => "danger",
            Context::None => "",
        }
    }
}

fn view_none() -> Html {
    html! {}
}
