use rocket::Error;

#[rocket::main]
async fn main() -> Result<(), Error> {
    let rocket = backend::rocket_factory()?;
    rocket.launch().await
}