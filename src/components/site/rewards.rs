use yew::{function_component, html};

#[function_component(Rewards)]
pub fn rewards() -> yew::Html {
    html! {
        <div class="columns is-variable is-8">
            <div class="column">
                <h4 class="title">{"MINTING REWARDS"}</h4>
                <div class="level">
                    <div class="level-item">
                        <ul>
                            <li>{"FREE MERCH TO WALLETS THAT MINT ON LAUNCH DAY"}</li>
                            <li>{"1/1 TOKEN REWARDS UP TO $50K IN ETH"}</li>
                            <li>{"SPECIAL ROLE IN DISCORD"}</li>
                            <li>{"MINT & HODL AIRDROP"}</li>
                        </ul>
                    </div>
                </div>
            </div>
            <div class="column">
                <h4 class="title">{"HOLDING REWARDS"}</h4>
                <div class="level">
                    <div class="level-item">
                        <ul>
                            <li>{"AIRDROPS"}</li>
                            <li>{"DAO VOTING POWER"}</li>
                            <li>{"METAVERSE ACCESS"}</li>
                            <li>{"$METAFASHION TOKEN REWARDS"}</li>
                            <li>{"EXCLUSIVE MERCH ACCESS"}</li>
                            <li>{"METAFASHION HQ PRIVATE CLUB"}</li>
                            <li>{"IRL MEET UP EVENT ACCESS"}</li>
                            <li>{"HODLER GIVEAWAYS (ETH, NFTS, TRIPS & MORE!)"}</li>
                        </ul>
                    </div>
                </div>
            </div>
        </div>
    }
}
