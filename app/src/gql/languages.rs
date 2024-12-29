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

    async fn languages_by_name(&self, keyword: String) -> Vec<Language> {
        let list = vec![
            Language { iso693_3: "spa", name: "Español".into() },
            Language { iso693_3: "ang", name: "Old English (ca.450–1100)".into() },
            Language { iso693_3: "enm", name: "Middle English (1100–1500)".into() },
            Language { iso693_3: "eng", name: "English".into() },
        ];

        list.into_iter().filter(|language| language.name.to_lowercase().contains(&keyword.to_lowercase())).collect()
    }
}