// mod mnemonic_generator;

// fn main() {
//     // println!("{}", mnemonic_generator::generate_random_string().unwrap_or(String::from("Err")));
//     let string : Option<String> = mnemonic_generator::generate_random_string();
//     match string {
//         Some(s) => {
//             println!("{}", s)
//         },
//         None => {
//             println!("Err: String contains u");
//         }
//     }

// }

#[macro_use]
extern crate rocket;
use dashmap::DashMap;
use rand::Rng;
// use rocket::State;

#[get("/index.html")]
fn index() -> &'static str {
    "<html> <h1 style=\"text-align: center;\">Welcome to the Mnemonic Generator!</h1> <p style=\"text-align: center;\">This is a simple web app that generates a random mnemonic.</p> <p style=\"text-align: center;\">To generate a mnemonic, go to <a href=\"/generate\">/generate</a>.</p> </html>"
}

#[get("/favicon.ico")]
fn img() -> &'static str {
    "Lochya!"
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .manage(DashMap::<u32, String>::new())
        .mount("/", routes![index, img, shorten, redirect])
}

#[post("/short?<url>")]
fn shorten(url: String, state: &rocket::State<DashMap<u32, String>>) -> Result<String, rocket::response::status::BadRequest<&str>> {
    if url.is_empty() {
        Err(rocket::response::status::BadRequest(Some("URL is empty!")))
    } else {
        let key: u32 = rand::thread_rng().gen();
        state.insert(key, url);
        Ok(key.to_string())
    }
}

#[get("/<key>")]
fn redirect(key: u32, state: &rocket::State<DashMap<u32, String>>) -> Result<rocket::response::Redirect, rocket::response::status::NotFound<&str>> {
    state
        .get(&key)
        .map(|url| rocket::response::Redirect::to(url.clone()))
        .ok_or(rocket::response::status::NotFound("Invalid or expired link!"))
}