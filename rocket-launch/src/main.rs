use rocket::*;
use mongodb::Client;
use std::env;
use mongodb::options::ClientOptions;
use mongodb::options::ResolverConfig;
use mongodb::bson::doc;
use bson::Document;
use rocket::futures::StreamExt;
use mongodb::Collection;

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

async fn generate_collection() -> Collection<Document> {
	let client_uri = env::var("MONGODB_URI").expect("You must set the MONGODB_URI environment var!");
	let options =
      ClientOptions::parse_with_resolver_config(&client_uri, ResolverConfig::cloudflare())
         .await.unwrap();

	let client = Client::with_options(options).unwrap();
	let collection = client.database("space_ships").collection::<Document>("spaceships");
	collection
}

#[get("/<name>/<age>/<cool>")]
async fn hello_world(name: &str, age: u32, cool: bool) -> String {
    if cool { format!("Hello, world! This is the cool {} testing his first ever Rocket API. He/She is {} years old", name, age) }
	else { format!("Hello, world! This is {} testing his first ever Rocket API. He/She is {} years old", name, age) }
}

#[get("/")]
async fn get_rockets() -> String {
	let collection = generate_collection().await;
	let mut cursor = collection.aggregate(None, None).await.unwrap();
	let mut res = String::new();

	while let Some(item) = cursor.next().await {
		let doc: Document = bson::from_document(item.unwrap()).unwrap();
		res.push_str(format!(" {}", doc.get("name").unwrap()).as_str());
	}
	res
}


#[rocket::main]
async fn main() -> Result<(), rocket::Error> {

	let _launch = rocket::build()
							.mount("/query", routes![hello_world])
							.mount("/", routes![get_rockets])
							.launch().await;
	Ok(())
}
