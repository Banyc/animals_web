use super::{edit, item, list, new};

use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Routable, PartialEq, Clone, Debug)]
pub enum Route {
    #[at("/animals/list")]
    List,
    #[at("/animals/new")]
    New,
    #[at("/animals/:id")]
    Item { id: i64 },
    #[at("/animals/edit/:id")]
    Edit { id: i64 },
}

pub fn switch(route: &Route) -> Html {
    match route {
        Route::List => html! (
            <list::List />
        ),
        Route::New => html! (
            <new::New />
        ),
        Route::Item { id } => html!(
            <item::Item id={ *id } is_new={ false } />
        ),
        Route::Edit { id } => html!(
            <edit::Edit id={ *id } />
        ),
    }
}
