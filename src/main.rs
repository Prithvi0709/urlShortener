/*
    Request Handler for the server.
    Author(s): Prithvi <prithvi_raj@gwu.edu> , Sam Ul Haque <sam.ulhaque@gwu.edu>
    Rust Group for ASP Fall 2022
*/

#[macro_use]
extern crate rocket;
mod url_validation;


#[launch]
fn rocket() -> _ {
    rocket::build()
        .manage(dashmap::DashMap::<u32, String>::new())
        .mount("/", routes![
            shorten,
            redirect
        ])
        .mount(
            "/",
            rocket::fs::FileServer::from("static_content/")
        )
}

#[get("/shorten?<url>&<translation_type>")]
fn shorten( 
        url: String,
        translation_type : String,
        state: &rocket::State<dashmap::DashMap<u32, String>>) -> Result<String, rocket::response::status::BadRequest<&str>> {

    if url.is_empty() {
        Err(rocket::response::status::BadRequest(Some("URL is empty!")))
    
    } else if url_validation::period_in_url(&url) == false {
        Err(rocket::response::status::BadRequest(Some("URL is invalid!")))
    
    } else {
        
        let url = url_validation::add_protocol(&url);
        println!("URL after adding protocol: {}", url);
        
        if translation_type == "1" {
            use rand::Rng;
            let key: u32 = rand::thread_rng().gen();
            state.insert(key, url);
            Ok(key.to_string())
        }

        else if translation_type == "2" {
            // TODO: Implement mnemonic return.
            // Here the key is a string. 
            // TODO Implement a new Dashmap with <String, String> type
            let theme: [String; 10] = ["Christmas".to_string(),"Santa".to_string(),"Feast".to_string(),
                                        "Festive".to_string(),"Elves".to_string(),"Ornaments".to_string(),
                                        "Presents".to_string(),"Snow".to_string(),"Holiday".to_string(),"Spirit".to_string()];
            
            let mut key = "".to_string();

            use rand::Rng;
            let num1: u32 = rand::thread_rng().gen_range(2..5);
            
            for i in 0..num1{
                let num2: usize = rand::thread_rng().gen_range(0..10);
                key.push_str(&theme[num2]);
            }

            state.insert(key, url);
            Ok(key)
        }

        else if translation_type == "3" {
            // TODO: Implement emoji return.
            Err(rocket::response::status::BadRequest(Some("Emoji not implemented yet!")))
        }

        else {
            Err(rocket::response::status::BadRequest(Some("Invalid translation type!")))
        }
    }
}


// Add Api for login and register, so that user can add a custom url

#[get("/<key>")]
fn redirect(key: u32, state: &rocket::State<dashmap::DashMap<u32, String>>) -> Result<rocket::response::Redirect, rocket::response::status::NotFound<&str>> {
    // TODO: Implement click tracking here.
    state
        .get(&key)
        .map(|url| rocket::response::Redirect::to(url.clone()))
        .ok_or(rocket::response::status::NotFound("Invalid or expired link!"))

}