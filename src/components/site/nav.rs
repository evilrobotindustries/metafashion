use yew::{function_component, html};

#[function_component(Navigation)]
pub fn nav() -> yew::Html {
    html! {
        <nav class="navbar is-transparent is-spaced is-fixed-top" role="navigation" aria-label="main navigation">
            <div class="navbar-brand">
                <a class="navbar-item" href="#">
                    <img src="/images/logo.svg"
                         alt="MetaFashionNFT is a collection of 10K generative 3D art pieces about to drop on the Ethereum blockchain!" />
                </a>

                <a role="button" class="navbar-burger" aria-label="menu" aria-expanded="false" data-target="navMenu">
                    <span aria-hidden="true"></span>
                    <span aria-hidden="true"></span>
                    <span aria-hidden="true"></span>
                </a>
            </div>

            <div id="navMenu" class="navbar-menu">
                <div class="navbar-start">
                    <a class="navbar-item" href="#rewards">{"Rewards"}</a>
                    <a class="navbar-item" href="#roadmap">{"Roadmap"}</a>
                    <a class="navbar-item" href="#team">{"Team"}</a>
                    <a class="navbar-item" href="#faq">{"FAQ"}</a>
                </div>
                <div class="navbar-end">
                    <div class="navbar-item">
                        <a href="https://twitter.com/MetaFashionNFT">
                            <span class="icon is-large">
                                <i class="fab fa-twitter fa-lg"></i>
                            </span>
                        </a>
                        <a href="https://discord.com/invite/baryFXMV">
                            <span class="icon is-large">
                                <i class="fab fa-discord fa-lg"></i>
                            </span>
                        </a>
                        <a href="https://instagram.com/metafashion.NFT">
                            <span class="icon is-large">
                                <i class="fab fa-instagram fa-lg"></i>
                            </span>
                        </a>
                    </div>
                    <div class="navbar-item is-spacer">{"|"}</div>
                    <div class="navbar-item">
                        <a class="button modal-button vip" data-target="vip">{"VIP List"}</a>
                    </div>
                </div>
            </div>
        </nav>
    }
}
