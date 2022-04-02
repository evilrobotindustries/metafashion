use crate::components::ElementList;
use gloo_console::error;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{Document, Element, HtmlAnchorElement, HtmlElement, KeyboardEvent, Window};
use yew::{function_component, html, use_effect};

mod faq;
mod footer;
mod hero;
mod nav;
mod rewards;
mod roadmap;
mod team;
mod vip;

#[function_component(Site)]
pub fn site() -> yew::Html {
    {
        use_effect(move || {
            on_load();
            || ()
        });
    }

    html! {
        <div id="site" class="has-navbar-fixed-top">
            // Navigation
            <nav::Navigation />
            // Hero
            <section class="hero is-fullheight">
                <hero::Hero />
            </section>
            // Rewards
            <section id="rewards" class="section">
                <rewards::Rewards />
            </section>
            // Roadmap
            <section id="roadmap" class="section">
                <h1 class="title">{"Roadmap"}</h1>
                <roadmap::Roadmap />
            </section>
            // Team
            <section id="team" class="section team">
                <h1 class="title">{"Team"}</h1>
                <div class="columns is-variable is-8">
                    <team::Team />
                </div>
            </section>
            // FAQ
            <section id="faq" class="section has-text-centered">
                <h1 class="title">{"FAQ"}</h1>
                <faq::FAQ />
            </section>
            // VIP
            <div id="vip" class="modal">
                <vip::VIP />
            </div>
            // Footer
            <footer class="footer">
                <footer::Footer />
            </footer>
        </div>
    }
}

fn on_load() {
    let window = web_sys::window().expect("global window does not exists");
    let document = window.document().expect("expecting a document on window");

    // Add navigation listeners
    add_navigation_listeners(&document, &window);

    add_modals(&document);

    // Attach all collapsible elements
    default::attach();
}

fn add_navigation_listeners(document: &Document, window: &Window) {
    // Check if there are any navbar burgers
    if let Ok(burgers) = document.query_selector_all(".navbar-burger") {
        let nav = document
            .get_elements_by_tag_name("nav")
            .item(0)
            .expect("could not find nav element");

        // Add a click event on each of them
        for burger in burgers.to_list::<Element>() {
            // Get the target from the "data-target" attribute
            let target = burger
                .get_attribute("data-target")
                .expect("could not find data-target attribute on burger");
            let target = document
                .get_element_by_id(&target)
                .expect("could not find target element");

            // Add click event listener to burger
            let nav = nav.clone();
            let burger_clone = burger.clone();
            let target_clone = target.clone();
            let listener = Closure::wrap(Box::new(move |_error: JsValue| {
                // Toggle the "is-active" class on the "navbar-burger" and the "navbar-menu"
                if let Err(e) = nav.class_list().toggle("is-active") {
                    error!(format!("unable to toggle is-active for nav: {:?}", e))
                }
                if let Err(e) = burger_clone.class_list().toggle("is-active") {
                    error!(format!("unable to toggle is-active for burger: {:?}", e))
                }
                if let Err(e) = target_clone.class_list().toggle("is-active") {
                    error!(format!(
                        "unable to toggle is-active for burger target: {:?}",
                        e
                    ))
                }
            }) as Box<dyn Fn(JsValue)>);
            if let Err(e) =
                burger.add_event_listener_with_callback("click", listener.as_ref().unchecked_ref())
            {
                error!("unable to add click event listener to burger: {:?}", e)
            }
            listener.forget();

            // Add listener to navbar items to close menu when clicked
            match target.query_selector_all(".navbar-item") {
                Ok(navbar_items) => {
                    let navbar_items = navbar_items.to_list::<Element>();
                    for item in &navbar_items {
                        let item_clone = item.clone();
                        let burger_clone = burger.clone();
                        let target_clone = target.clone();
                        let navbar_items = navbar_items.clone();
                        // Close menu when item clicked
                        let listener = Closure::wrap(Box::new(move |_error: JsValue| {
                            for item in &navbar_items {
                                if let Err(e) = item.class_list().remove_1("is-active") {
                                    error!("unable to remove is-active from navbar-item: {:?}", e)
                                }
                            }

                            if let Err(e) = item_clone.class_list().toggle("is-active") {
                                error!(format!(
                                    "unable to toggle is-active for navbar item: {:?}",
                                    e
                                ))
                            }
                            if let Err(e) = burger_clone.class_list().toggle("is-active") {
                                error!(format!("unable to toggle is-active for burger: {:?}", e))
                            }
                            if let Err(e) = target_clone.class_list().toggle("is-active") {
                                error!(format!("unable to toggle is-active for target: {:?}", e))
                            }
                        })
                            as Box<dyn Fn(JsValue)>);
                        if let Err(e) = item.add_event_listener_with_callback(
                            "click",
                            listener.as_ref().unchecked_ref(),
                        ) {
                            error!("unable to add click event listener to navbar item: {:?}", e)
                        }
                        listener.forget();
                    }
                }
                Err(error) => error!(error),
            }
        }

        // Set navbar items as active as user scrolls through sections
        let sections = document
            .query_selector_all("section")
            .expect("unable to find any sections")
            .to_list::<HtmlElement>();
        let navbar_items = document
            .query_selector_all(".navbar-item")
            .expect("unable to find any navbar items")
            .to_list::<HtmlAnchorElement>();
        const OFFSET: i32 = 70;

        let window_clone = window.clone();
        let listener = Closure::wrap(Box::new(move |_error: JsValue| {
            let mut current = "".to_string();

            // Add class to nav once user scrolls
            if let Ok(y) = window_clone.scroll_y() {
                if y > 10.0 {
                    if let Err(e) = nav.class_list().add_1("scroll") {
                        error!("unable to add scroll class to nav: {:?}", e)
                    }
                } else {
                    if let Err(e) = nav.class_list().remove_1("scroll") {
                        error!("unable to remove scroll class from nav: {:?}", e)
                    }
                }

                for section in &sections {
                    if y as i32 >= section.offset_top() - OFFSET {
                        if let Some(attribute) = section.get_attribute("id") {
                            current = format!("#{}", attribute);
                        }
                    }
                }

                for item in &navbar_items {
                    if item.hash() == current {
                        if let Err(e) = item.class_list().add_1("is-active") {
                            error!("unable to add is-active class to nav item: {:?}", e)
                        }
                    } else {
                        if let Err(e) = item.class_list().remove_1("is-active") {
                            error!("unable to remove is-active class from nav item: {:?}", e)
                        }
                    }
                }
            }
        }) as Box<dyn Fn(JsValue)>);
        window.set_onscroll(Some(listener.as_ref().unchecked_ref()));
        listener.forget();
    }
}

fn add_modals(document: &Document) {
    // Add a click event on buttons to open a specific modal
    if let Ok(modal_buttons) = document.query_selector_all(".modal-button") {
        for button in modal_buttons.to_list::<Element>() {
            let target = button
                .get_attribute("data-target")
                .expect("could not find data-target attribute on modal button");
            let target = document
                .get_element_by_id(&target)
                .expect("could not find target element");

            // Add event listener
            let target_clone = target.clone();
            let listener = Closure::wrap(Box::new(move |_error: JsValue| {
                if let Err(e) = target_clone.class_list().add_1("is-active") {
                    error!("unable to add is-active class to modal: {:?}", e)
                }
            }) as Box<dyn Fn(JsValue)>);
            if let Err(e) =
                button.add_event_listener_with_callback("click", listener.as_ref().unchecked_ref())
            {
                error!(
                    "unable to add click event listener to modal button: {:?}",
                    e
                )
            }
            listener.forget();
        }
    }

    // Add a click event on various child elements to close the parent modal
    if let Ok(modal_buttons) = document.query_selector_all(
        ".modal-background, .modal-close, .modal-card-head .delete, .modal-card-foot .button",
    ) {
        for close in modal_buttons.to_list::<Element>() {
            if let Some(target) = close
                .closest(".modal")
                .expect("could not find closest modal")
            {
                // Add event listener
                let target_clone = target.clone();
                let listener = Closure::wrap(Box::new(move |_error: JsValue| {
                    if let Err(e) = target_clone.class_list().remove_1("is-active") {
                        error!("unable to remove is-active class from modal: {:?}", e)
                    }
                }) as Box<dyn Fn(JsValue)>);
                if let Err(e) = close
                    .add_event_listener_with_callback("click", listener.as_ref().unchecked_ref())
                {
                    error!(
                        "unable to add click event listener to close button: {:?}",
                        e
                    )
                }
                listener.forget();
            }
        }
    }

    // Add a keyboard event to close all modals
    let document_clone = document.clone();
    let listener = Closure::wrap(Box::new(move |e: KeyboardEvent| {
        // Check for escape key
        if e.key_code() == 27 {
            if let Ok(modals) = document_clone.query_selector_all(".modal") {
                for modal in modals.to_list::<Element>() {
                    if let Err(e) = modal.class_list().remove_1("is-active") {
                        error!("unable to remove is-active class from modal: {:?}", e)
                    }
                }
            }
        }
    }) as Box<dyn Fn(KeyboardEvent)>);
    if let Err(e) =
        document.add_event_listener_with_callback("keydown", listener.as_ref().unchecked_ref())
    {
        error!("unable to add keydown event listener to document: {:?}", e)
    }
    listener.forget();
}

#[wasm_bindgen(module = "/assets/bulma-collapsible.min.js")]
extern "C" {
    #[allow(non_camel_case_types)]
    type default;

    #[wasm_bindgen(static_method_of = default)]
    fn attach();
}
