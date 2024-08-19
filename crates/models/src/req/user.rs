#[derive(Debug,serde::Deserialize,garde::Validate)]
pub struct RegisterRequest{
    #[garde(length(max=64,min=1))]
    pub username:String,
    #[garde(email)]
    pub email:String,
    #[garde(skip)]
    pub password:String,
}

#[derive(Debug,serde::Deserialize,garde::Validate)]
pub struct LoginRequest{
    #[garde(length(max=64,min=1))]
    pub username:String,
    #[garde(skip)]
    pub password:String,
}