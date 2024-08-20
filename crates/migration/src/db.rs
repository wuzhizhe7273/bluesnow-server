use models::r#do::meta_info::{MetaInfo, MetaInfoIden};
use sea_query::Query;
use sqlx::AnyConnection;
use std::error::Error;
use sea_query_binder::SqlxBinder;
use util::{DataObject};
use util::query::get_query_builder;

#[derive(sqlx::FromRow)]
pub struct Name {
    pub name: String,
}

/// 获取数据库info信息
pub async fn get_info(conn: &mut AnyConnection) -> Result<Option<MetaInfo>, sqlx::Error> {
    let (query,values)=Query::select()
    .columns([MetaInfoIden::Version,MetaInfoIden::Database,MetaInfoIden::Initialized])
    .from(MetaInfoIden::Table)
        .build_any_sqlx(&*get_query_builder(conn))
    ;
    let info = sqlx::query_as_with::<_, MetaInfo,_>(&query,values)
        .fetch_optional(conn)
        .await?;
    Ok(info)
}

pub async fn check_info_exists(conn: &mut AnyConnection) -> Result<bool, sqlx::Error> {
    let res = match conn.backend_name() {
        "SQLite" => {
            sqlx::query_as::<_, Name>(
                "SELECT `name` FROM `sqlite_master` WHERE `type`='table' AND name='meta_info'",
            )
            .fetch_optional(conn)
            .await?
        }
        "MySQL" => {
            sqlx::query_as::<_,Name>("SELECT `table_name` FROM information_schema.tables WHERE table_schema = '数据库名称' AND table_name = '表名'").fetch_optional(conn).await?;
            todo!()
        }
        "PostgreSQL" => {
            sqlx::query_as::<_,Name>("SELECT `tablename` FROM pg_tables WHERE schemaname = 'public' AND tablename = 'mytable'").fetch_optional(conn).await?;
            todo!()
        }
        _ => panic!("Unsupported database"),
    };
    Ok(res.is_some())
}

pub async fn migrate(conn: &mut AnyConnection) {
    match conn.backend_name() {
        "SQLite" => {
            sqlx::migrate!("../../migrations/sqlite")
                .run(conn)
                .await
                .unwrap();
        }
        "MySQL" => {
            todo!()
        }
        "PostgreSQL" => {
            todo!()
        }
        _ => panic!("Unsupported Database"),
    };
}

pub async fn undo(conn: &mut AnyConnection, target: i64) {
    match conn.backend_name() {
        "SQLite" => {
            sqlx::migrate!("../../migrations/sqlite")
                .undo(conn, target)
                .await
                .unwrap();
        }
        "MySQL" => {
            todo!()
        }
        "PostgreSQL" => {
            todo!()
        }
        _ => panic!("Unsupported Database"),
    };
}

pub enum InitialStatus {
    TableNotExist,
    DataNotExist,
    AllReady,
}

pub async fn check_initialized(conn: &mut AnyConnection) -> Result<InitialStatus, sqlx::Error> {
    if check_info_exists(conn).await? {
        return Ok(InitialStatus::TableNotExist);
    }
    if get_info(conn).await?.is_none() {
        return Ok(InitialStatus::DataNotExist);
    }
    Ok(InitialStatus::AllReady)
}

pub async fn seed_data<M:DataObject>(conn: &mut AnyConnection,data:&str) -> Result<(), Box<dyn Error>> {
    let data: Vec<M> = serde_json::from_str(data).map_err(|e| Box::new(e))?;
    for item in data {
        item.save(conn).await?
    }
    Ok(())
}
pub fn seed_sata(_conn: &mut AnyConnection) -> Result<(), sqlx::Error> {
    todo!()
}
