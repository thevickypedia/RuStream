use rustream::start;

#[actix_rt::main]
async fn main() {
    start().await.unwrap();
}
