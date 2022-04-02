use yew::{function_component, html};

#[function_component(Footer)]
pub fn footer() -> yew::Html {
    html! {
        <>
            <div class="content has-text-centered">
                <img src="/images/logo.svg"
                     alt="MetaFashionNFT is a collection of 10K generative 3D art pieces about to drop on the Ethereum blockchain!" />
            </div>
            <div class="content has-text-centered">
                <a href="https://twitter.com/MetaFashionNFT">
                    <i class="fab fa-twitter fa-2xl"></i>
                </a>
                <a href="https://discord.com/invite/baryFXMV">
                    <i class="fab fa-discord fa-2xl"></i>
                </a>
                <a href="https://instagram.com/metafashion.NFT">
                    <i class="fab fa-instagram fa-2xl"></i>
                </a>
            </div>
            <div class="content has-text-centered">
                <p>{"© 2022 METAFASHION"}</p>
                <p>{"All Rights Reserved"}</p>
            </div>
        </>
    }
}
