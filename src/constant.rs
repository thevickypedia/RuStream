use fernet::Fernet;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref FERNET: Fernet = Fernet::new(&generate_key()).unwrap();
}

fn generate_key() -> String {
    Fernet::generate_key()
}
