use yew::{Component, Context, html, Html};
use crate::components::{loading::Loading, site::Site};

pub struct App{
    loaded: bool
}

pub enum AppMessage{
    Loaded
}

impl Component for App {
    type Message = AppMessage;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self{
            loaded: false
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            AppMessage::Loaded => {
                self.loaded = true;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let onclick = ctx.link().callback(|x| AppMessage::Loaded);
        html!{
            if !self.loaded{
                <Loading />
                <button {onclick}>{"Load" }</button>
            }
            else{
                <Site />
            }
        }
    }
}