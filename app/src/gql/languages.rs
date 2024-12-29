use async_graphql::{Object, SimpleObject};

#[derive(SimpleObject)]
pub(super) struct Language {
    pub iso693_3: &'static str,
    pub name: String
}

#[derive(Default)]
pub(super) struct LanguageQuery;

#[Object]
impl LanguageQuery {
    async fn languages(&self) -> Vec<Language> {
        vec![
            Language { iso693_3: "spa", name: "Español".into() },
            Language { iso693_3: "ang", name: "Old English (ca.450–1100)".into() },
            Language { iso693_3: "enm", name: "Middle English (1100–1500)".into() },
            Language { iso693_3: "eng", name: "English".into() },
        ]
    }
}