use serde::Deserialize;
use sqlx::AnyConnection;

pub trait DataObject:for <'a> Deserialize<'a>{
    #[allow(async_fn_in_trait)]
    async fn save(self,conn:&mut AnyConnection)->result::Result<Self>;
}