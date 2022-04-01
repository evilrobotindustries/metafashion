use yew::{function_component, html};

#[function_component(VIP)]
pub fn vip() -> yew::Html {
    html! {
        <>
            <div class="modal-background"></div>
            <div class="modal-content">
                <div class="container">
                    <section class="section" >
                        <h1 class="title">{"VIP Signup"}</h1>
                        <div class="block">
                            <p>{"> Connect your wallet to add your address to the MetaFashion VIP list"}</p>
                        </div>
                        <div class="block">
                            <a class="button modal-button vip" data-target="vip">{"Connect Wallet"}</a>
                        </div>
                    </section>
                </div>
            </div>
            <button class="modal-close is-large" aria-label="close"></button>
        </>
    }
}
