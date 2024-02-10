#[allow(non_snake_case)]
use RuStream::start;

#[actix_rt::main]
async fn main() {
    start().await.unwrap();
}
