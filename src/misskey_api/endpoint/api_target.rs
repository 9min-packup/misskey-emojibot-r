pub enum ApiTarget {
    I,
    USER_SHOW,
}

impl ApiTarget {
    pub fn to_string(&self) -> String {
        match self {
            ApiTarget::I => String::from("i"),
            ApiTarget::USER_SHOW => String::from("user/show"),
        }
    }
}