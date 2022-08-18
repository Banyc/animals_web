use crate::app::animals::services::persistence::PERSISTENCE_INSTANCE;

use super::route::Route;

use yew::prelude::*;
use yew_router::prelude::*;

pub struct List {}

impl Component for List {
    type Message = Msg;

    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::None => false,
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let animals = PERSISTENCE_INSTANCE.get_all();
        html!(
            <>
                <h1>{ "Animals" }</h1>
                {
                    for animals.iter().map(|animal| {
                        html! (
                            <>
                                <h2>{ "Face: " } { &animal.face }</h2>
                                <Link<Route> to={ Route::Item { id: animal.id } }>{ "Show this animal" }</Link<Route>>
                                // <p>{ "Show this animal" }</p>
                                <br/>
                            </>
                        )
                    })
                }
                // {
                //     html! (
                //         <>
                //             <h2>{ "Face: " } { "🐒" }</h2>
                //             <Link<Route> to={ Route::Item { id: 0 } }>{ "Show this animal" }</Link<Route>>
                //             // <p>{ "Show this animal" }</p>
                //             <br/>
                //         </>
                //     )
                // }
                <Link<Route> to={ Route::New }>{ "New animal" }</Link<Route>>
                // <p>{ "New animal" }</p>
            </>
        )
    }
}

pub enum Msg {
    None,
}
