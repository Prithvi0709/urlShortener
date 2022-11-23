
#[macro_use]
extern crate rocket;
use dashmap::DashMap;
use rand::Rng;


// #[get("/")]
// fn index() ->  &'static str {
//     return "Welcome to Our thing!!"
// }

#[get("/favicon.ico")]
fn img() -> &'static str {
    "Lochya!"
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .manage(DashMap::<u32, String>::new())
        .mount("/", routes![shorten, redirect])

        .mount(
            "/",
            if cfg!(debug_assertions) {
                // debug mode, therefore serve relative to crate root
                rocket::fs::FileServer::from(rocket::fs::relative!("html/"))
            } else {
                // dockerized, therefore serve from absolute path
                rocket::fs::FileServer::from("html/")
            },
        )
}

#[get("/shorten?<url>&<translation_type>")]
fn shorten(url: String, translation_type : String, state: &rocket::State<DashMap<u32, String>>) -> Result<String, rocket::response::status::BadRequest<&str>> {
    // TODO: Three translation types:
    // 1. Random number (default)
    // 2. Memonic TODO
    // 3. Emoji   TODO
    println!( "URL: {}", url);
    if url.is_empty() {
        Err(rocket::response::status::BadRequest(Some("URL is empty!")))
    } else {
        // Impliment type trigred url key change here.
        let key: u32 = rand::thread_rng().gen();
        state.insert(key, url);
        // print out the key to the rocket console
        println!("Key: {}", translation_type);
        Ok(key.to_string())
    }
}


// Add Api for login and register, so that user can add a custom url

#[get("/<key>")]
fn redirect(key: u32, state: &rocket::State<DashMap<u32, String>>) -> Result<rocket::response::Redirect, rocket::response::status::NotFound<&str>> {
    state
        .get(&key)
        .map(|url| rocket::response::Redirect::to(url.clone()))
        .ok_or(rocket::response::status::NotFound("Invalid or expired link!"))

}