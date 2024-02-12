#[actix_rt::main]
async fn main() {
    match rustream::start().await {
        Ok(_) => {
            println!("Successfully served session")
        }
        Err(err) => {
            eprintln!("Error starting rustream: {}", err)
        }
    }
}
