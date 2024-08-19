use crate::context::ServerContext;
use crate::repo;
use models::req::user::RegisterRequest;
use models::resp::user::RegisterResponse;

pub async fn register(
    context: ServerContext,
    req: RegisterRequest,
) -> result::Result<RegisterResponse> {
    let pwd = util::pwd::hash(&req.password).await?;
    let tx = &mut context.db.begin().await?;
    let uid = repo::user::register(tx, &req.username, &req.email, &pwd).await?;
    let resp = RegisterResponse::new(uid);
    // let resp=RegisterResponse::new(Uuid::now_v7());
    Ok(resp)
}

// pub async fn login(context:ServerContext,req:LoginRequest)->result::Result<LoginResponse>{
//     let conn=&context.db;
//     let user=repo::user::get_by_username(conn,&req.username).await?.to_result()?;
//     util::pwd::verify(&req.password,&user.password).await?;
//     todo!()
// }
