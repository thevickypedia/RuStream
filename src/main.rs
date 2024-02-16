#[actix_rt::main]
async fn main() {
    match rustream::start().await {
        Ok(_) => {
            println!("RuStream session terminated")
        }
        Err(err) => {
            eprintln!("Error starting rustream: {}", err)
        }
    }
}
