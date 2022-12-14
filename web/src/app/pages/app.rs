use super::route::{switch, Route};
use yew::prelude::*;
use yew_router::prelude::*;

pub struct App {}

impl Component for App {
    type Message = ();

    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html!(
            <BrowserRouter>
                <Switch<Route> render={ Switch::render(switch) } />
            </BrowserRouter>
        )
    }
}
