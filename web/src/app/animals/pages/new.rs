use crate::app::animals::services::persistence::PERSISTENCE_INSTANCE;

use super::item;
use super::route::Route;

use internal::animals::models::animal::Animal;

use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_router::prelude::*;

pub struct New {
    redirect: RedirectTo,
}

impl Component for New {
    type Message = Msg;

    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            redirect: RedirectTo::No,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Create(animal) => {
                let id = PERSISTENCE_INSTANCE.create(animal);
                self.redirect = RedirectTo::ItemPage { id };
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        match self.redirect {
            RedirectTo::ItemPage { id } => {
                html! {
                    <item::Item id={ id } is_new={ true } />
                }
            }
            RedirectTo::No => {
                let face_rdf = NodeRef::default();
                let face_rdf_clone = face_rdf.clone();
                let on_click = ctx.link().callback(move |_| {
                    Msg::Create(Animal {
                        id: 0,
                        face: face_rdf_clone.cast::<HtmlInputElement>().unwrap().value(),
                    })
                });
                html!(
                    <>
                        <h1>{ "New animal" }</h1>
                        <p>{ "Face" }</p>
                        <input ref={ face_rdf } />
                        <br/>
                        <button onclick={ on_click }>{ "Create Animal" }</button>
                        <br/>
                        <br/>
                        <Link<Route> to={ Route::List }>{ "Back to animals" }</Link<Route>>
                    </>
                )
            }
        }
    }
}

pub enum Msg {
    Create(Animal),
}

pub enum RedirectTo {
    ItemPage { id: i64 },
    No,
}
