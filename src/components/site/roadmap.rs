use yew::{classes, function_component, html, Html};

#[function_component(Roadmap)]
pub fn roadmap() -> Html {
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

struct Phase {
    name: &'static str,
    items: Vec<Item>,
}
struct Item {
    title: &'static str,
    content: &'static str,
}
