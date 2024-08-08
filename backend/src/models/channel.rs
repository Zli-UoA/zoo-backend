use async_graphql::SimpleObject;

#[derive(SimpleObject, Debug, PartialEq, Eq)]
pub struct Channel {
    pub id: String,
    pub channel_name: String,
    pub description: Option<String>,
    pub is_private: bool,
    pub created_user_id: String,
    pub created_at: String,
    pub updated_at: Option<String>,
    pub archive_at: Option<String>,
    pub deleted_at: Option<String>,
}
