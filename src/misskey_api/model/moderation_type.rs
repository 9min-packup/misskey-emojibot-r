
#[derive(Debug)]
pub enum ModerationType {
    AddCustomEmoji,
    UpdateCustomEmoji,
    DeleteCustomEmoji,
    CreateAvatarDecoration,
    
    UpdateAvatarDecoration,
    DeleteAvatarDecoration,
    Other,
}

impl ModerationType {
    pub fn to_string(t : &ModerationType) -> String {
        match &t {
            ModerationType::AddCustomEmoji => String::from("addCustomEmoji"),
            ModerationType::UpdateCustomEmoji => String::from("updateCustomEmoji"),
            ModerationType::DeleteCustomEmoji => String::from("deleteCustomEmoji"),
            ModerationType::CreateAvatarDecoration => String::from("createAvatarDecoration"),
            ModerationType::UpdateAvatarDecoration => String::from("updateAvatarDecoration"),
            ModerationType::DeleteAvatarDecoration => String::from("deleteAvatarDecoration"),
            ModerationType::Other => String::from("other")
        }
    }

    pub fn from_string(s : &str) -> ModerationType {
        match &s {
            &"addCustomEmoji" => ModerationType::AddCustomEmoji,
            &"updateCustomEmoji" => ModerationType::UpdateCustomEmoji,
            &"deleteCustomEmoji" => ModerationType::DeleteCustomEmoji,
            &"createAvatarDecoration" => ModerationType::CreateAvatarDecoration,
            &"updateAvatarDecoration" => ModerationType::UpdateAvatarDecoration,
            &"deleteAvatarDecoration" => ModerationType::DeleteAvatarDecoration,
            _ => ModerationType::Other,
        }
    }
}
