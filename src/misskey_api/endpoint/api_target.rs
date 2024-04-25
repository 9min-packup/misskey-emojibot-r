pub enum ApiTarget {
    I,
    UsersShow,
    AdminShowModerationLogs,
}

impl ApiTarget {
    pub fn to_string(t : &ApiTarget) -> String {
        match &t {
            ApiTarget::I => String::from("i"),
            ApiTarget::UsersShow => String::from("users/show"),
            ApiTarget::AdminShowModerationLogs => String::from("admin/show-moderation-logs"),
        }
    }

}