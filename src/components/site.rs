use yew::{classes, Callback, function_component, html, Html};
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/assets/bulma-collapsible.min.js")]
extern "C" {

    #[wasm_bindgen(js_namespace = console)]
    fn log(s: String);

    type default;

    #[wasm_bindgen(static_method_of = default)]
    fn attach();
}

#[function_component(Site)]
pub fn site() -> Html {
    let load = Callback::from(|_| {
        default::attach();
    });

    html! {
        <div id="site" onload={load}>
            // Navigation
            <Navigation />
            // Hero
            <section class="hero is-fullheight">
                <Hero />
            </section>
            // Rewards
            <section id="rewards" class="section">
                <Rewards />
            </section>
            // Roadmap
            <section id="roadmap" class="section">
                <h1 class="title">{"Roadmap"}</h1>
                <Roadmap />
            </section>
            // Team
            <section id="team" class="section team">
                <h1 class="title">{"Team"}</h1>
                <div class="columns is-variable is-8">
                    <Team />
                </div>
            </section>
            // FAQ
            <section id="faq" class="section has-text-centered">
                <h1 class="title">{"FAQ"}</h1>
                <FAQ />
            </section>
            // Footer
            <Footer />
        </div>
    }
}

#[function_component(Navigation)]
fn nav() -> Html {
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

#[function_component(Hero)]
fn hero() -> Html {
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

#[function_component(Rewards)]
fn rewards() -> Html {
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

#[function_component(Roadmap)]
fn roadmap() -> Html {

    struct Phase{
        name: &'static str,
        items: Vec<Item>
    }
    struct Item {
        title: &'static str,
        content: &'static str,
    }

    let phases = vec![
        Phase {
            name: "Phase 1",
            items: vec![
                Item{
                    title: "Ready to Launch",
                    content: "LAUNCH DISCORD AND WEBSITE"
                },
                Item{
                    title: "Rewards",
                    content: "REWARD THE GROWING COMMUNITY WITH GIVEAWAYS INCLUDING ETH, NFTS AND SPECIAL ROLES
                            WITHIN THE COMMUNITY WITH VIP LIST BENEFITS"
                },
                Item{
                    title: "Launch",
                    content: "LAUNCH THE COLLECTION OF 10K METAFASHION AVATARS INTO THE METAVERSE"
                }
            ]
        },
        Phase {
            name: "Phase 2",
            items: vec![
                Item {
                    title:"Foundation",
                    content:"ENABLE OUR DAO SYSTEM THAT WILL ALLOW HOLDERS OF THE PROJECT TO HAVE VOTING POWER ON ALL MAJOR DECISIONS COMING TO THE PROJECT VIA SNAPSHOT"
                },
                Item {
                    title:"Rewards",
                    content:"CURRENT HODLERS WILL BE REWARDED WITH EXCLUSIVE AIRDROPS INCLUDING ADDITIONAL NFTS & ETH"
                },
                Item {
                    title:"Rewards",
                    content:"MINTING REWARDS WILL BE DEPLOYED"
                },
                Item {
                    title:"Merchandise",
                    content:"OFFICIAL MERCH STORE WILL BE RELEASED TO ALL CURRENT HODLERS WITH ACCESS TO HIGH QUALITY MERCH RELATED TO THE METAFASHION BRAND AS WELL AS IRL HIGH FASHION REPRODUCTIONS OF YOUR AVATARS GEAR"
                },
            ]
        },
        Phase {
            name: "Phase 3",
            items: vec![
                Item {
                    title:"Giving Back",
                    content:"ACTIVATE AND FUND THE COMMUNITY AND CHARITY WALLET. (DAO SYSTEM WILL BE USED TO DECIDE UPON WHICH CHARITIES WILL BE CONSIDERED)"
                },
                Item {
                    title:"Meet Up",
                    content:"FIRST IRL MEET UP"
                },
                Item {
                    title:"Merchandise",
                    content:"MERCH STORE IS RELEASED TO THE PUBLIC (LIMITED QUANTITY)"
                },
            ]
        },
        Phase {
            name: "Phase 4",
            items: vec![
                Item {
                    title:"MetaFashion HQ",
                    content:"THE METAVERSE ACTION BEGINS! DAO VOTING STARTS CONSTRUCTION OF METAFASHION HQ IN THE METAVERSE"
                },
                Item {
                    title:"Tokenomics",
                    content:"HODLERS WILL BEGIN TO RECEIVE $METAFASHION TOKEN FOR AMOUNT OF HODLING TIME"
                },
                Item {
                    title:"Airdrop",
                    content:"AIRDROP TO ALL HODLERS (TIER BASED & METAVERSE READY)"
                },
                Item {
                    title:"Metaverse",
                    content:"METAVERSE IS LAUNCHED AND CONSTITUTION OF METAFASHION HQ CURATED IN A METAVERSE MEET UP"
                },
            ]
        },
        Phase {
            name: "Phase 5",
            items: vec![
                Item {
                    title:"Next Up",
                    content:"METAFASHION’S NEXT DROP"
                },
                Item {
                    title:"Play 2 Earn",
                    content:"BUILDING A FOUNDATION FOR PLAY 2 EARN"
                },
            ]
        },
        Phase {
            name: "Moon",
            items: vec![]
        }
    ];

    let phases = phases.iter().enumerate().map(|(i, phase)| {
        let id = format!("phase{}", i + 1);
        let is_active = i == 0;
        html! {
        <div class="timeline-phase">
            if !phase.items.is_empty() {
                <header class="timeline-header">
                    <a href={ format!("#{}", id) } data-action="collapse" data-target={ format!("#{}", id) }>
                        <span class="tag is-medium">
                            { phase.name }
                            <i class="fa-solid fa-angle-down"></i>
                        </span>
                    </a>
                </header>
                <div class="timeline-item is-spacer"></div>
                <div id={ id } class={classes!("is-collapsible", is_active.then(|| Some("is-active")))}>
                    {
                        phase.items.iter().map(|item| html! {
                            <div class="timeline-item">
                                <div class="timeline-marker is-24x24 is-image"></div>
                                <div class="timeline-content">
                                    <p class="heading">{ item.title }</p>
                                    <p>{ item.content }</p>
                                </div>
                            </div>
                        }).collect::<Html>()
                    }
                </div>
            }
            else {
                <div class="timeline-header">
                    <span class="tag is-medium">{ phase.name }</span>
                </div>
            }
        </div>
    }
    }).collect::<Html>();

    html! {
        <div class="timeline is-centered">
            { phases }
        </div>
    }
}

#[function_component(Team)]
fn team() -> Html {
    struct TeamMember{
        name: &'static str,
        role: &'static str,
        class: &'static str,
        image: &'static str,
        twitter: &'static str,
        instagram: &'static str
    }

    let team = vec![
        TeamMember{
            name: "CAPTVART",
            role: "Owner / Lead Artist",
            class: "is-blue",
            image: "cb.png",
            twitter: "captvart",
            instagram: "captvart"
        },
        TeamMember{
            name: "Art by Jeremy",
            role: "Owner / Branding / Dev",
            class: "is-orange",
            image: "je.png",
            twitter: "Jeremyalanmob",
            instagram: "jeremy.eggert"
        },
        TeamMember{
            name: "Hilly",
            role: "Artist",
            class: "is-red",
            image: "h.png",
            twitter: "hillywrld",
            instagram: "hillywrld"
        }
    ];

    team.iter().map(|member| html! {
        <div class="column">
            <div class="card">
                <figure class="image">
                    <img class={ format!("is-rounded {}", member.class )}
                         src={ format!("/images/{}", member.image) }
                         alt={ member.name } />
                </figure>
                <div class="card-content has-text-centered">
                    <div class="content">
                        <p class="title is-5">{ member.name }</p>
                        <p class="subtitle is-5">{ member.role }</p>
                    </div>
                    <div class="media">
                        <div class="media-content ">
                            <p class="subtitle is-6">
                            <span class="icon is-large">
                                <a href={ format!("https://twitter.com/{}", member.twitter ) }>
                                    <span class="icon is-rounded">
                                        <i class="fab fa-twitter fa-2xl"></i>
                                    </span>
                                </a>
                            </span>
                            <span class="icon is-large">
                                <a href={ format!("https://instagram.com/{}", member.instagram ) }>
                                    <span class="icon is-rounded">
                                        <i class="fab fa-instagram fa-2xl"></i>
                                    </span>
                                </a>
                            </span>
                            </p>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }).collect::<Html>()
}

#[function_component(FAQ)]
fn faq() -> Html {
    struct FrequentlyAskedQuestion {
        question: &'static str,
        answer: Html,
    }

    let faqs: Vec<FrequentlyAskedQuestion> = vec![
        FrequentlyAskedQuestion{
            question: "When is Mint Day?",
            answer: html!{
                <p>{"Q2 2022 on Ethereum: Official Date’s will be announced later"}</p>
            }
        },
        FrequentlyAskedQuestion{
            question: "How Much Will it Cost?",
            answer: html! {
                <>
                    <p>{"VIP LIST will be .075Ξ + Gas"}</p>
                    <p>{"Public Sale will be .085Ξ + Gas"}</p>
                </>
            }
        },
        FrequentlyAskedQuestion{
            question: "What is the Total Supply?",
            answer: html! {
                <>
                    <p class="block">{"There will be 10,000 MetaFashion NFT’s"}</p>
                    <p>{"Some of the supply will be designated for contests and giveaways within our Community and to award those that help us grow"}</p>
                </>
            }
        },
        FrequentlyAskedQuestion{
            question: "What is the Max Mint?",
            answer: html! {
                <>
                    <p>{"VIP LIST will have a limit of 3 Mints, one transaction Max"}</p>
                    <p>{"PUBLIC SALE will have a limit of 5 Mints per Transaction, for up to 3 transactions"}</p>
                </>
            }
        },
        FrequentlyAskedQuestion{
            question: "When will giveaway winners receive their Prizes?",
            answer: html! {
                <>
                    <p>{"Winners will receive their prizes once we sell out of all Meta Fashion NFT’s"}</p>
                </>
            }
        },
    ];

    faqs.iter().enumerate().map(|(i, faq)| {
        let id = format!("faq{}", i + 1);
        html! {
        <div class="block">
            <header class="question">
                <a href={ format!("#{}", id) } data-action="collapse" data-target={ format!("#{}", id) }>
                    { format!("{}   ", faq.question) }<i class="fa-solid fa-angle-down"></i>
                </a>
            </header>
            <div id={ id } class="block answer is-collapsible">
                { faq.answer.clone() }
            </div>
        </div>
    }
    }).collect::<Html>()
}

#[function_component(Footer)]
fn footer() -> Html {
    html! {
        <footer class="footer">
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
        </footer>
    }
}




