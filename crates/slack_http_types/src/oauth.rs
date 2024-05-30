pub enum OauthToken {
    Bot(String),
    User(String),
}

impl OauthToken {
    pub fn value(&self) -> &str {
        match self {
            OauthToken::Bot(token) => token.as_str(),
            OauthToken::User(token) => token.as_str(),
        }
    }
}
