use argon2::{
    password_hash::{rand_core::OsRng,SaltString},
    Argon2, PasswordHash, PasswordHasher, PasswordVerifier,
};
pub fn argon_hash(content: impl AsRef<str>) -> Result<String, argon2::password_hash::Error> {
    let salt = SaltString::generate(&mut OsRng);
    let argon = Argon2::default();
    Ok(argon
        .hash_password(content.as_ref().as_bytes(), &salt)?
        .to_string())
}

pub fn argon_verify(
    content: impl AsRef<str>,
    hash: impl AsRef<str>,
) -> Result<(), argon2::password_hash::Error> {
    let parsed_hash = PasswordHash::new(hash.as_ref())?;
    Argon2::default().verify_password(content.as_ref().as_bytes(), &parsed_hash)
}

#[cfg(test)]
mod test_hash{
    use crate::hash::{argon_hash, argon_verify};

    #[test]
    fn test_argon(){
        let pwd="123456";
        let hash_pwd=argon_hash(pwd).unwrap();
        argon_verify(pwd,&hash_pwd).unwrap()
    }
}