use uuid::Uuid;

#[derive(serde::Serialize)]
pub struct RegisterResponse{
    uid:Uuid
}
impl RegisterResponse {
    pub fn new(uid:Uuid)->Self{
        Self{
            uid
        }
    }
}

#[derive(serde::Serialize)]
pub struct LoginResponse {
    token: String
}
impl LoginResponse {
    pub fn new(token:String)->Self{
        Self{
            token
        }
    }
}