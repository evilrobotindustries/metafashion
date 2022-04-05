use yew::{function_component, html, Html};

#[function_component(FAQ)]
pub fn faq() -> Html {
    let faqs: Vec<FrequentlyAskedQuestion> = vec![
        FrequentlyAskedQuestion {
            question: "When is Mint Day?",
            answer: html! {
                <p>{"Q2 2022 on Ethereum: Official Date’s will be announced later"}</p>
            },
        },
        FrequentlyAskedQuestion {
            question: "How Much Will it Cost?",
            answer: html! {
                <>
                    <p>{"VIP LIST will be .075Ξ + Gas"}</p>
                    <p>{"Public Sale will be .085Ξ + Gas"}</p>
                </>
            },
        },
        FrequentlyAskedQuestion {
            question: "What is the Total Supply?",
            answer: html! {
                <>
                    <p class="block">{"There will be 10,000 MetaFashion NFT’s"}</p>
                    <p>{"Some of the supply will be designated for contests and giveaways within our Community and to award those that help us grow"}</p>
                </>
            },
        },
        FrequentlyAskedQuestion {
            question: "What is the Max Mint?",
            answer: html! {
                <>
                    <p>{"VIP LIST will have a limit of 3 Mints, one transaction Max"}</p>
                    <p>{"PUBLIC SALE will have a limit of 5 Mints per Transaction, for up to 3 transactions"}</p>
                </>
            },
        },
        FrequentlyAskedQuestion {
            question: "When will giveaway winners receive their Prizes?",
            answer: html! {
                <>
                    <p>{"Winners will receive their prizes once we sell out of all Meta Fashion NFT’s"}</p>
                </>
            },
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
                <p class="animate__animated animate__fadeIn">{ faq.answer.clone() }</p>
            </div>
        </div>
    }
    }).collect::<Html>()
}

struct FrequentlyAskedQuestion {
    question: &'static str,
    answer: Html,
}
