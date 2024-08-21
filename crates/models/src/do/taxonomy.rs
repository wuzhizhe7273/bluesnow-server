use chrono::{DateTime, Utc};
use sea_query::{enum_def, Expr, Query};
use sea_query_binder::SqlxBinder;
use sqlx::{query_as_with, AnyConnection};
use util::query::get_query_builder;
use util::{DataObject, FromAnyRow};
use uuid::Uuid;

#[enum_def]
#[derive(FromAnyRow, serde::Deserialize, Clone, DataObject)]
pub struct Taxonomy{
    tid:Uuid,
    name:String,
    path:String,
    cover:Option<String>,
    desc:String,
    created:DateTime<Utc>,
    changed:DateTime<Utc>
}

impl Taxonomy{
    pub async fn get_by_tid(conn:&mut AnyConnection,tid:Uuid)->result::Result<Option<Self>>{
        let (query,values)=Query::select()
            .columns([TaxonomyIden::Tid,TaxonomyIden::Name,TaxonomyIden::Path,TaxonomyIden::Cover,TaxonomyIden::Desc,TaxonomyIden::Created,TaxonomyIden::Changed])
            .from(TaxonomyIden::Table)
            .and_where(Expr::col(TaxonomyIden::Tid).eq(tid))
            .build_any_sqlx(&*get_query_builder(conn));
        let taxonomy=query_as_with::<_,Self,_>(&query,values).fetch_optional(conn).await?;
        Ok(taxonomy)
    }

    pub async fn get_related_nodes(conn:&mut AnyConnection,node_type:&str)->result::Result<()>{
        todo!()
    }
    pub async fn get_by_path_prefix(conn:&mut AnyConnection,path:&str)->result::Result<Vec<Taxonomy>>{
        let (query,values)= Query::select()
            .columns([TaxonomyIden::Tid,TaxonomyIden::Name,TaxonomyIden::Path,TaxonomyIden::Cover,TaxonomyIden::Desc,TaxonomyIden::Created,TaxonomyIden::Changed])
            .from(TaxonomyIden::Table)
            .and_where(Expr::col(TaxonomyIden::Path).like(format!("{}%",path)))
            .build_any_sqlx(&*get_query_builder(conn));
        let taxonomy=query_as_with::<_,Self,_>(&query,values).fetch_all(conn).await?;
        Ok(taxonomy)
    }
}