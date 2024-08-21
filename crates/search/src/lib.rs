use std::collections::HashMap;

pub trait DocIndex{

}

pub trait DocSchema{
    
}

pub struct Indices<'a>{
    indices:HashMap<&'a str,Box<dyn DocIndex>>
}
impl<'a> Indices<'a> {

}

#[cfg(test)]
mod test{
    use chrono::{DateTime, Utc};
    use serde::{Deserialize, Serialize};
    use serde_json::Value as Json;
    use uuid::Uuid;

    #[derive(Debug,Deserialize,Serialize)]
    pub struct Doc{
        id:Uuid,
        title:String,
        data:Json,
        created:DateTime<Utc>,
        changed:DateTime<Utc>
    }
    #[test]
    fn example(){
        
    }
}