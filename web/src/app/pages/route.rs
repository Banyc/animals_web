use super::{super::animals, index};

use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Routable, PartialEq, Clone)]
pub enum Route {
    #[at("/animals/*")]
    Animals,
    #[at("/animals")]
    AnimalsIndex,
    #[at("/")]
    Index,
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn switch(route: &Route) -> Html {
    match route {
        Route::Index => html! (
            <index::Index />
        ),
        Route::Animals => html! (
            <Switch<animals::pages::route::Route> render={ Switch::render(animals::pages::route::switch) } />
        ),
        Route::AnimalsIndex => html!(
            <Redirect<animals::pages::route::Route> to={animals::pages::route::Route::List} />
        ),
        Route::NotFound => html!(
            <>
                <p>{ "404" }</p>
            </>
        ),
    }
}
