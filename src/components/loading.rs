use yew::{function_component, html};

#[function_component(Loading)]
pub fn loading() -> yew::Html {
    html! {
        <section id="landing" class="hero is-fullheight">
            <section class="hero is-fullheight">
                <div class="hero-body">
                    <div class="container has-text-centered">
                        <div class="block">
                            <img src="/images/vial.webp" alt="MetaFashion" />
                        </div>
                        <div id="loading" class="container has-text-centered">
                            <div class="loading block">{"Loading"}</div>
                            <div class="progress-bar">
                                <div id="progress-bar-value" class="value"></div>
                            </div>
                            <div id="status" class="block">{"1%"}</div>
                        </div>
                    </div>
                </div>
            </section>
        </section>
    }
}
