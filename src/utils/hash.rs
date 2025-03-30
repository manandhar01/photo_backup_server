use bcrypt::hash;

pub fn hash_password(password: &str) -> String {
    hash(password, 12).expect("Failed to hash password")
}
