use crate::app::animals::services::persistence::PERSISTENCE_INSTANCE;

use super::route::Route;

use yew::prelude::*;
use yew_router::prelude::*;

pub struct Item {
    redirect: RedirectTo,
}

impl Component for Item {
    type Message = Msg;

    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            redirect: RedirectTo::No,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Destroy(id) => {
                PERSISTENCE_INSTANCE.delete(id);
                self.redirect = RedirectTo::ListPage;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        match self.redirect {
            RedirectTo::ListPage => {
                html! {
                    <Redirect<Route> to={ Route::List } />
                }
            }
            RedirectTo::No => {
                let animal = PERSISTENCE_INSTANCE.get(ctx.props().id);
                match animal {
                    Some(animal) => {
                        let destroy_link_on_click =
                            ctx.link().callback(move |_| Msg::Destroy(animal.id));
                        html!(
                            <>
                                {
                                    if ctx.props().is_new {
                                        html! (
                                            <>
                                                <p>{ "Animal was successfully created." }</p>
                                                <br/>
                                            </>
                                        )
                                    } else {
                                        html! ()
                                    }
                                }
                                <h1>{ "Face: " } { animal.face }</h1>

                                <Link<Route> to={ Route::Edit { id: animal.id } }>{ "Edit this animal" }</Link<Route>>
                                { " | " }
                                <Link<Route> to={ Route::List }>{ "Back to animals" }</Link<Route>>
                                <br/>
                                <button onclick={ destroy_link_on_click }>{ "Destroy this animal" }</button>
                            </>
                        )
                    }
                    None => {
                        html! {
                            <>
                                <h1>{ "Animal not found" }</h1>
                                <Link<Route> to={ Route::List }>{ "Back to animals" }</Link<Route>>
                            </>
                        }
                    }
                }
            }
        }
    }
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub id: i64,
    pub is_new: bool,
}

pub enum Msg {
    Destroy(i64),
}

enum RedirectTo {
    ListPage,
    No,
}
