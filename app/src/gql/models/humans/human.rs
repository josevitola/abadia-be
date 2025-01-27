use async_graphql::{ComplexObject, Context, SimpleObject};
use chrono::{DateTime, Utc};

use crate::{
    gql::models::{countries::Country, texts::Text},
    utils::db::get_bridge_ids,
};

use super::AppContext;

#[derive(sqlx::FromRow, Hash, Clone, SimpleObject)]
#[graphql(complex)]
pub struct Human {
    pub id: String,
    pub last_name: String,
    pub first_name: Option<String>,
    pub country1_id: Option<String>,
    pub country2_id: Option<String>,
    pub birthyear: Option<i32>,
    pub deathyear: Option<i32>,
    pub pseudonym: Option<String>,
    pub dcr: DateTime<Utc>,
}

#[ComplexObject]
impl Human {
    async fn country1(&self, ctx: &Context<'_>) -> Result<Option<Country>, async_graphql::Error> {
        let country_loader = &ctx.data::<AppContext>()?.loaders.countries;

        if self.country1_id.is_some() {
            country_loader
                .load_one(self.country1_id.clone().unwrap())
                .await
        } else {
            Ok(None)
        }
    }

    async fn country2(&self, ctx: &Context<'_>) -> Result<Option<Country>, async_graphql::Error> {
        let country_loader = &ctx.data::<AppContext>()?.loaders.countries;

        if self.country2_id.is_some() {
            country_loader
                .load_one(self.country2_id.clone().unwrap())
                .await
        } else {
            Ok(None)
        }
    }

    async fn name(&self) -> String {
        if let Some(pseudonym) = &self.pseudonym {
            pseudonym.to_string()
        } else {
            let last_name = &self.last_name;
            let first_name_opt = &self.first_name;

            if first_name_opt.is_none() {
                last_name.to_string()
            } else {
                let first_name = first_name_opt.as_ref().unwrap();
                format!("{first_name} {last_name}")
            }
        }
    }

    async fn texts(&self, ctx: &Context<'_>) -> Result<Vec<Text>, async_graphql::Error> {
        let context = ctx.data::<AppContext>()?;
        let pool = &context.pool;

        let query = sqlx::query("SELECT text_id AS bridge FROM text_authors WHERE author_id = $1")
            .bind(&self.id);

        let text_ids: Vec<String> = get_bridge_ids(query, pool).await?;
        if text_ids.is_empty() {
            ()
        }

        let text_loader = &context.loaders.texts;

        Ok(text_loader
            .load_many(text_ids)
            .await?
            .values()
            .cloned()
            .collect())
    }
}
