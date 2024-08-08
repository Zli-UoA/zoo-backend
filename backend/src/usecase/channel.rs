use crate::context::Context;
use crate::generate::entities::channel::Model;
use crate::models::channel::Channel;

pub fn get_all_channel(_ctx: &Context) -> Result<Vec<Channel>, ()> {
    todo!("generate::entities::channel::Model to models::channel::Channel")
}

#[cfg(test)]
mod test {
    use super::get_all_channel;
    use crate::context::Context;
    use crate::generate::entities::channel;
    use crate::models::channel::Channel;
    use sea_orm::prelude::*;
    use sea_orm::MockDatabase;

    #[test]
    fn すべてのチャンネルを取得する() {
        // Arrange
        let db: &DatabaseConnection = &MockDatabase::new(sea_orm::DatabaseBackend::Postgres)
            .append_query_results([vec![channel::Model {
                id: "0".to_string(),
                channel_name: "hoge".to_string(),
                description: Some("huga".to_string()),
                is_private: false,
                created_user_id: "0".to_string(),
                created_at: DateTime::parse_from_str("2024-08-08 00:00:00", "%Y-%m-%d %H:%M:%S")
                    .unwrap(),
                updated_at: None,
                archive_at: None,
                deleted_at: None,
            }]])
            .into_connection();

        // Action
        let result = get_all_channel(&Context::new()).unwrap();

        // Assert
        assert_eq!(
            result,
            vec![Channel {
                id: "0".to_string(),
                channel_name: "hoge".to_string(),
                description: Some("huga".to_string()),
                is_private: false,
                created_user_id: "0".to_string(),
                created_at: "2024-08-08 00:00:00".to_string(),
                updated_at: None,
                archive_at: None,
                deleted_at: None,
            }],
        )
    }
}
