#[allow(dead_code)]
pub enum ReactionAcceptance {
    All,
    LikeOnly,
    LikeOnlyForRemote,
    NonSensitiveOnly,
    NonSensitiveOnlyForLocalLikeOnlyForRemote,
}

impl ReactionAcceptance {
    #[allow(dead_code)]
    pub fn to_string(t : &ReactionAcceptance) -> Option<String> {
        match &t {
            ReactionAcceptance::All => None,
            ReactionAcceptance::LikeOnly => Some(String::from("likeOnly")),
            ReactionAcceptance::LikeOnlyForRemote => Some(String::from("likeOnlyForRemote")),
            ReactionAcceptance::NonSensitiveOnly => Some(String::from("nonSensitiveOnly")),
            ReactionAcceptance::NonSensitiveOnlyForLocalLikeOnlyForRemote => Some(String::from("nonSensitiveOnlyForLocalLikeOnlyForRemote")),
        }
    }

    #[allow(dead_code)]
    pub fn from_str(t : &str) -> ReactionAcceptance {
        match &t {
            &"null" => ReactionAcceptance::All,
            &"all" => ReactionAcceptance::All,
            &"likeOnly" => ReactionAcceptance::LikeOnly,
            &"likeOnlyForRemote" => ReactionAcceptance::LikeOnlyForRemote,
            &"nonSensitiveOnly" => ReactionAcceptance::NonSensitiveOnly,
            &"nonSensitiveOnlyForLocalLikeOnlyForRemote" => ReactionAcceptance::NonSensitiveOnlyForLocalLikeOnlyForRemote,
            _ => ReactionAcceptance::All,
        }
    }
}
