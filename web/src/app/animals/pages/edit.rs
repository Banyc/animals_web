use crate::app::animals::services::persistence::PERSISTENCE_INSTANCE;

use super::route::Route;

use internal::animals::models::animal::Animal;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_router::prelude::*;

pub struct Edit {
    redirect: RedirectTo,
}

impl Component for Edit {
    type Message = Msg;

    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            redirect: RedirectTo::No,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Update { animal } => {
                PERSISTENCE_INSTANCE.update(animal);
                self.redirect = RedirectTo::ItemPage;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        match self.redirect {
            RedirectTo::ItemPage => {
                html!(
                    <Redirect<Route> to={ Route::Item { id: ctx.props().id } } />
                )
            }
            RedirectTo::No => {
                let animal = PERSISTENCE_INSTANCE.get(ctx.props().id);
                let face_ref = NodeRef::default();
                let face_ref_clone = face_ref.clone();
                let id = ctx.props().id;
                let confirm_on_click = ctx.link().callback(move |_| Msg::Update {
                    animal: Animal {
                        id,
                        face: face_ref_clone.cast::<HtmlInputElement>().unwrap().value(),
                    },
                });
                match animal {
                    Some(animal) => {
                        html!(
                            <>
                                <h1>{ "Edit animal" }</h1>
                                <p>{ "face: " }</p>
                                <input ref={ face_ref } value={ animal.face } />
                                <br/>
                                <button onclick={ confirm_on_click } >{ "Confirm" }</button>
                                <br/>
                                <Link<Route> to={ Route::List }>{ "Back to list" }</Link<Route>>
                            </>
                        )
                    }
                    None => {
                        html! (
                            <Redirect<Route> to={ Route::List } />
                        )
                    }
                }
            }
        }
    }
}

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    pub id: i64,
}

pub enum Msg {
    Update { animal: Animal },
}

enum RedirectTo {
    ItemPage,
    No,
}
