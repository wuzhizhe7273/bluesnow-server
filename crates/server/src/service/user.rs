use models::req::user::RegisterRequest;
use crate::context::ServerContext;

pub fn register(context:ServerContext,req:RegisterRequest)->anyhow::Result<()>{
    let pwd=util::hash::argon_hash(&req.password)?;
    Ok(())
}