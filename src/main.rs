#[actix_rt::main]
async fn main() {
    match rustream::start().await {
        Ok(_) => {
            println!("\nRuStream session has ended")
        }
        Err(err) => {
            eprintln!("\nError starting RuStream: {}", err)
        }
    }
}
