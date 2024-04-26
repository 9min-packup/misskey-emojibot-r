#[allow(dead_code)]
pub enum Visibility {
    Public,
    Home,
    Followers,
    Specified,
}

impl Visibility {
    #[allow(dead_code)]
    pub fn to_string(t : &Visibility) -> String {
        match &t {
            Visibility::Public => String::from("public"),
            Visibility::Home => String::from("home"),
            Visibility::Followers => String::from("followers"),
            Visibility::Specified => String::from("specified"),
        }
    }

    #[allow(dead_code)]
    pub fn from_str(t : &str) -> Visibility {
        match &t {
            &"public" => Visibility::Public,
            &"home" => Visibility::Home,
            &"followers" => Visibility::Followers,
            &"specified" => Visibility::Specified,
            _ => Visibility::Public,
        }
    }
}