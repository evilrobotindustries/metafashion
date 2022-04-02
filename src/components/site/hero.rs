use yew::{function_component, html};

#[function_component(Hero)]
pub fn hero() -> yew::Html {
    html! {
        <div class="hero-body">
            <div id="hero-image" class="glitch">
                <div class="glitch-image"></div>
                <div class="glitch-image"></div>
                <div class="glitch-image"></div>
                <div class="glitch-image"></div>
                <div class="glitch-image"></div>
            </div>
            <div id="hero-content" class="block">
                <p class="title">{"The takeover is now"}</p>
                <p class="subtitle has-text-centered">
                    <a href="#rewards">
                        <span class="icon is-large">
                            <img class="arrow" src="/images/arrow.svg" alt="Scroll for more"/>
                        </span>
                    </a>
                </p>
            </div>
        </div>
    }
}
