use tokio::task::spawn_blocking;
use result::invalid_input_error;
use crate::hash::{argon_hash, argon_verify};

pub async fn hash(password: &str) -> result::Result<String> {
    let password = password.to_string();
    let jh = spawn_blocking(move || argon_hash(password));
    let password = jh.await??;
    // let password=argon_hash(&password)?;
    Ok(password)
}

pub async fn verify(password: &str, hashed_pass: &str) -> result::Result<()> {
    let password=password.to_string();
    let hashed_pass=hashed_pass.to_string();
    let jh = spawn_blocking(move || argon_verify(password, hashed_pass));
    if let Err(e) = jh.await? {
        #[cfg(feature = "log")]
        tracing::debug!("The password is not correct: {e}");
        Err(invalid_input_error(
            "password",
            "The password is not correct.",
        ))
    } else {
        Ok(())
    }
}
