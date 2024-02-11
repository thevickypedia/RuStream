use rustream;

#[actix_rt::main]
async fn main() {
    rustream::start().await.unwrap();
}
