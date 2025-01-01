use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Tweet {
    #[serde(rename = "__typename")]
    pub typename: String,
    pub core: Core,
    pub legacy: Legacy,
    #[serde(rename = "rest_id")]
    pub rest_id: String,
    pub source: String,
    pub views: Views,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Core {
    pub user_results: UserResults,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UserResults {
    pub result: User,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct User {
    pub id: String,
    pub legacy: UserLegacy,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UserLegacy {
    pub followers_count: u32,
    pub name: String,
    pub screen_name: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Legacy {
    #[serde(rename = "bookmark_count")]
    pub bookmark_count: i32,
    pub bookmarked: bool,
    #[serde(rename = "conversation_id_str")]
    pub conversation_id_str: String,
    #[serde(rename = "created_at")]
    pub created_at: String,
    #[serde(rename = "display_text_range")]
    pub display_text_range: Vec<i32>,
    pub entities: LegacyEntities,
    #[serde(rename = "favorite_count")]
    pub favorite_count: i32,
    pub favorited: bool,
    #[serde(rename = "full_text")]
    pub full_text: String,
    #[serde(rename = "id_str")]
    pub id_str: String,
    #[serde(rename = "is_quote_status")]
    pub is_quote_status: bool,
    pub lang: String,
    #[serde(rename = "quote_count")]
    pub quote_count: i32,
    #[serde(rename = "reply_count")]
    pub reply_count: i32,
    #[serde(rename = "retweet_count")]
    pub retweet_count: i32,
    pub retweeted: bool,
    #[serde(rename = "user_id_str")]
    pub user_id_str: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct LegacyEntities {
    pub hashtags: Vec<Hashtag>,
    pub symbols: Vec<Symbol>,
    #[serde(rename = "user_mentions")]
    pub user_mentions: Vec<UserMentions>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Hashtag {
    pub indices: Vec<i32>,
    pub text: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UserMentions {
    pub id_str: String,
    pub screen_name: String,
    pub name: String,
    pub indices: Vec<i32>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Symbol {
    pub indices: Vec<i32>,
    pub text: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Views {
    pub count: String,
    pub state: String,
}
