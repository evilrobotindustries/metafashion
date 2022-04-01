use gloo_timers::future::sleep;
use std::time::Duration;
use gloo_console::debug;
use yew::{Component, Context, html, Html};
use crate::components::{loading::Loading, site::Site};
use wasm_bindgen::JsCast;
use web_sys::{FontFace, HtmlElement};

const IMAGES: [&str;7] = [
    "/images/logo.svg",
    "/images/header.jpg",
    "/images/star.svg",
    "/images/arrow.svg",
    "/images/cb.png",
    "/images/je.png",
    "/images/h.png",
];
const FONT : (&str, &str) = ("Druk Wide Medium", "url('/fonts/7e389c5e310dc537b083e0e25ea6eab5.woff2') format('woff2')");

pub struct App{
    loaded: bool
}

impl App {
    async fn pre_load_assets(){
        debug!("pre-loading assets...");

        sleep(Duration::from_secs(1)).await;

        let window = web_sys::window().expect("global window does not exists");
        let document = window.document().expect("expecting a document on window");
        let status = document.get_element_by_id("status").expect("could not find status element")
            .dyn_into::<HtmlElement>().expect("could not find status element");
        let progress_indicator = document.get_element_by_id("progress-bar-value")
            .expect("could not find progress value element")
            .dyn_into::<HtmlElement>().expect("could not find progress value element");

        let total = IMAGES.len() as f32 + 1.0;
        for (i, image) in IMAGES.iter().enumerate(){
            // Create an img element to get browser to cache image
            let element = document.create_element("img")
                .unwrap()
                .dyn_into::<web_sys::HtmlImageElement>()
                .unwrap();
            element.set_src(image);

            set_status(&status, &progress_indicator, ((i as f32 + 1.0) / total) * 100.0);
        };

        // Load the font
        if let Err(error) = FontFace::new_with_str(FONT.0, FONT.1).expect("could not load the font").load(){
            debug!(format!("unable to load the {} font: {:?}", FONT.0, error))
        }

        // Finally update completion status
        set_status(&status, &progress_indicator, 100f32);
        sleep(Duration::from_millis(350)).await;

        fn set_status(status: &HtmlElement, progress_indicator: &HtmlElement, percent: f32){
            if percent >= 100f32 {
                if let Err(error) = progress_indicator.class_list().add_1("is-complete") {
                    debug!(format!("unable to update the progress: {:?}", error))
                }
            }
            let percent = format!("{}%", percent);
            if let Err(error) = progress_indicator.style().set_property("width", &percent) {
                debug!(format!("unable to update the progress: {:?}", error))
            }
            status.set_inner_text(&percent);
        }

        debug!("assets loaded");
    }
}

pub enum AppMessage{
    Loaded
}

impl Component for App {
    type Message = AppMessage;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        // Start pre-loading assets
        ctx.link().send_future(async move {
            App::pre_load_assets().await;
            AppMessage::Loaded
        });
        Self { loaded: false }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            AppMessage::Loaded => {
                self.loaded = true;
                true
            }
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html!{
            if !self.loaded{ <Loading /> }
            else { <Site /> }
        }
    }
}