use rocket::*;

#[get("/<name>/<age>/<cool>")]
async fn hello_world(name: &str, age: u32, cool: bool) -> String {
    if cool { format!("Hello, world! This is the cool {} testing his first ever Rocket API. He/She is {} years old", name, age) }
	else { format!("Hello, world! This is {} testing his first ever Rocket API. He/She is {} years old", name, age) }
}


#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
	let _rocket = rocket::build()
						.mount("/", routes![hello_world])
						.launch()
						.await?;
	Ok(())
}