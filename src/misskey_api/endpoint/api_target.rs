pub enum ApiTarget {
    I,
    UsersShow,
    NotesShow,
    NotesCreate,
    AdminShowModerationLogs,
}

impl ApiTarget {
    pub fn to_string(t : &ApiTarget) -> String {
        match &t {
            ApiTarget::I => String::from("i"),
            ApiTarget::UsersShow => String::from("users/show"),
            ApiTarget::NotesShow => String::from("notes/show"),
            ApiTarget::NotesCreate => String::from("notes/create"),
            ApiTarget::AdminShowModerationLogs => String::from("admin/show-moderation-logs"),
        }
    }

}