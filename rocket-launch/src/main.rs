use rocket::*;
use mongodb::Client;
use std::env;
use mongodb::options::ClientOptions;
use mongodb::options::ResolverConfig;

#[get("/<name>/<age>/<cool>")]
async fn hello_world(name: &str, age: u32, cool: bool) -> String {
    if cool { format!("Hello, world! This is the cool {} testing his first ever Rocket API. He/She is {} years old", name, age) }
	else { format!("Hello, world! This is {} testing his first ever Rocket API. He/She is {} years old", name, age) }
}


#[rocket::main]
async fn main() -> Result<(), rocket::Error> {

	let client_uri = env::var("MONGODB_URI").expect("You must set the MONGODB_URI environment var!");
	let options =
      ClientOptions::parse_with_resolver_config(&client_uri, ResolverConfig::cloudflare())
         .await.unwrap();

	let client = Client::with_options(options).unwrap();


	// for name in client.list_database_names(None, None).await.unwrap() {
	// 	println!("- {}", name);
	//  }
  
	//  Ok(())
	let _launch = rocket::build().mount("/", routes![hello_world]).launch().await;
	Ok(())
}
