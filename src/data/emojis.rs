use rand::seq::SliceRandom; // 0.7.2

#[derive(Clone)]
pub struct Emoji {
    pub name: String,
    pub id: String,
    pub animated: bool,
}

pub fn get_emojis() -> Emoji {
    let emojis = [
        Emoji {
            name: "jibril".to_owned(),
            id: "914851644674572308".to_owned(),
            animated: false,
        },
        Emoji {
            name: "SCblushy".to_owned(),
            id: "834155037852696656".to_owned(),
            animated: false,
        },
        Emoji {
            name: "OmegaGoodJobA".to_owned(),
            id: "499602590921916421".to_owned(),
            animated: true,
        },
        Emoji {
            name: "ShiroUnamused".to_owned(),
            id: "318672492799721472".to_owned(),
            animated: false,
        },
        Emoji {
            name: "shiroheart".to_owned(),
            id: "738235624565506050".to_owned(),
            animated: false,
        },
        Emoji {
            name: "SchwiHeart".to_owned(),
            id: "912843049879625758".to_owned(),
            animated: false,
        },
        Emoji {
            name: "shiroPat".to_owned(),
            id: "668152473965559819".to_owned(),
            animated: false,
        },
        Emoji {
            name: "shiroEmbarassed".to_owned(),
            id: "788411174215090216".to_owned(),
            animated: false,
        },
        Emoji {
            name: "shiroMischevious".to_owned(),
            id: "774777625973358633".to_owned(),
            animated: false,
        },
        Emoji {
            name: "shiroAnnoyed".to_owned(),
            id: "674249440340738049".to_owned(),
            animated: false,
        },
        Emoji {
            name: "shiroNoms".to_owned(),
            id: "682499753011052555".to_owned(),
            animated: false,
        },
        Emoji {
            name: "shiroSleeb".to_owned(),
            id: "674949450547986441".to_owned(),
            animated: false,
        },
        Emoji {
            name: "shiroScared".to_owned(),
            id: "795782234425655367".to_owned(),
            animated: false,
        },
        Emoji {
            name: "shiroLoveletter".to_owned(),
            id: "674398556106260500".to_owned(),
            animated: false,
        },
        Emoji {
            name: "JibrilWant2".to_owned(),
            id: "464435940283121675".to_owned(),
            animated: true,
        },
    ];

    return emojis.choose(&mut rand::thread_rng()).unwrap().clone();
}
