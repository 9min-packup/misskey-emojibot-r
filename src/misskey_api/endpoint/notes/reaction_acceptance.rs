#[allow(dead_code)]
pub enum ReactionAcceptance {
    ALL,
    LikeOnly,
    LikeOnlyForRemote,
    NonSensitiveOnly,
    NonSensitiveOnlyForLocalLikeOnlyForRemote,
}

impl ReactionAcceptance {
    pub fn to_string(t : &ReactionAcceptance) -> Option<String> {
        match &t {
            ReactionAcceptance::ALL => None,
            ReactionAcceptance::LikeOnly => Some(String::from("likeOnly")),
            ReactionAcceptance::LikeOnlyForRemote => Some(String::from("likeOnlyForRemote")),
            ReactionAcceptance::NonSensitiveOnly => Some(String::from("nonSensitiveOnly")),
            ReactionAcceptance::NonSensitiveOnlyForLocalLikeOnlyForRemote => Some(String::from("nonSensitiveOnlyForLocalLikeOnlyForRemote")),
        }
    }
}
