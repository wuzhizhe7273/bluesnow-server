use meilisearch_sdk::client::Client;
use meilisearch_sdk::documents::DocumentsQuery;
use serde::{Deserialize, Serialize};
use tantivy::doc;

#[derive(Serialize, Deserialize, Clone)]
pub struct Doc {
    id: String,
    data: String,
}

pub struct MeiliIndices {
    client: Client,
}

impl MeiliIndices {
    fn new() -> Self {
        todo!()
    }
}

pub struct QueryResult<I: DocIndex> {
    pub total: Option<usize>,
    pub offset: Option<usize>,
    pub limit: Option<usize>,
    pub data: Vec<I>,
}

impl Indices for MeiliIndices {
    async fn get_docs_by_id<I: DocIndex>(&self, id: I::ID) -> Vec<I> {
        let index = self.client.index(I::index_name());
        let doc = DocumentsQuery::new(&index)
            .with_filter(&format!("{} = {}", I::primary_key(), id.to_string()))
            .with_limit(1)
            .execute::<I>()
            .await
            .unwrap()
            .results;
        doc
    }
    async fn add_or_update_docs<I: DocIndex>(&self, docs: Vec<I>) {
        let index = self
            .client
            .index(I::index_name())
            .add_or_update(&docs, Some(I::primary_key()))
            .await
            .unwrap();
    }
    async fn delete_docs_by_ids<I: DocIndex>(&self, ids: Vec<I::ID>) {
        let ids = ids.into_iter().map(|id| id.to_string()).collect::<Vec<_>>();
        let task = self
            .client
            .index(I::index_name())
            .delete_documents(&ids)
            .await
            .unwrap();
    }
    async fn create_index<I: DocIndex>(&self) {
        self.client
            .create_index(I::index_name(), Some(I::primary_key()))
            .await
            .unwrap();
    }
    async fn query<I: DocIndex>(&self, query: &str) -> QueryResult<I> {
        let result = self
            .client
            .index(I::index_name())
            .search()
            .with_query(query)
            .execute::<I>()
            .await
            .unwrap();
        let result = QueryResult {
            total: result.estimated_total_hits,
            offset: result.offset,
            limit: result.limit,
            data: result.hits.into_iter().map(|item| item.result).collect(),
        };
        result
    }
}

pub trait Indices {
    async fn get_docs_by_id<I: DocIndex>(&self, id: I::ID) -> Vec<I>;
    async fn delete_docs_by_ids<I: DocIndex>(&self, id: Vec<I::ID>);
    async fn create_index<I: DocIndex>(&self);
    async fn add_or_update_docs<I: DocIndex>(&self, docs: Vec<I>);
    async fn query<I: DocIndex>(&self, query: &str) -> QueryResult<I>;
}

pub trait DocIndex: for<'a> Deserialize<'a> + Serialize + Send + Sync + 'static + Clone {
    type ID: ToString;
    fn primary_key() -> &'static str;
    fn index_name() -> &'static str;

    fn get_fields() -> Vec<&'static str>;
}

impl DocIndex for Doc {
    type ID = String;
    fn primary_key() -> &'static str {
        "id"
    }
    fn index_name() -> &'static str {
        "doc"
    }

    fn get_fields() -> Vec<&'static str> {
        vec!["id", "data"]
    }
}
