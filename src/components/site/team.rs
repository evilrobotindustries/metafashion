use yew::{function_component, html, Html};

#[function_component(Team)]
pub fn team() -> Html {
    let team = vec![
        TeamMember {
            name: "CAPTVART",
            role: "Owner / Lead Artist",
            class: "is-blue",
            image: "cb.png",
            twitter: "captvart",
            instagram: "captvart",
        },
        TeamMember {
            name: "Art by Jeremy",
            role: "Owner / Branding / Dev",
            class: "is-orange",
            image: "je.png",
            twitter: "Jeremyalanmob",
            instagram: "jeremy.eggert",
        },
        TeamMember {
            name: "Hilly",
            role: "Artist",
            class: "is-red",
            image: "h.png",
            twitter: "hillywrld",
            instagram: "hillywrld",
        },
    ];

    team.iter().map(|member| html! {
        <div class="column">
            <div class="card">
                <figure class="image">
                    <img class={ format!("is-rounded animate__animated {}", member.class )}
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

struct TeamMember {
    name: &'static str,
    role: &'static str,
    class: &'static str,
    image: &'static str,
    twitter: &'static str,
    instagram: &'static str,
}
