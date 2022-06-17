use super::tag::*;
use crate::state::AppStateRaw;

#[async_trait]
pub trait ITag: std::ops::Deref<Target = AppStateRaw> {
    async fn tag_query(&self, name: &str) -> sqlx::Result<Vec<Tag>>;
    async fn get_all_tags(&self, name: &str) -> sqlx::Result<Vec<Tag>>;
}

#[cfg(any(feature = "postgres"))]
#[async_trait]
impl ITag for &AppStateRaw {
    async fn tag_query(&self, name: &str) -> sqlx::Result<Vec<Tag>> {       
        let sql = format!(
            "SELECT *
            FROM tags
            where {} like '{}%';",
            "name", name
        );

        sqlx::query_as(&sql).fetch_all(&self.sql).await
    }

    async fn get_all_tags(&self, name: &str) -> sqlx::Result<Vec<Tag>> {
        sqlx::query_as!(
            Tag, r#"
            select * from tags where name > $1 order by post_count desc, name limit $2
            "#,
            name, self.config.tags_per_page
        )
        .fetch_all(&self.sql)
        .await
    }
}