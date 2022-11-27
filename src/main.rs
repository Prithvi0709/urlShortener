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
        .manage(dashmap::DashMap::<String, String>::new())
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
        state: &rocket::State<dashmap::DashMap<String, String>>) -> Result<String, rocket::response::status::BadRequest<&str>> {

    if url.is_empty() {
        Err(rocket::response::status::BadRequest(Some("URL is empty!")))
    
    } else if url_validation::period_in_url(&url) == false {
        Err(rocket::response::status::BadRequest(Some("URL is invalid!")))
    
    } else {
        
        let url = url_validation::add_protocol(&url);
        println!("URL after adding protocol: {}", url);
        // Key converted from u32 to string

        if translation_type == "1" {
            use rand::Rng;
            let key: u32 = rand::thread_rng().gen();
            state.insert(key.to_string(), url);
            println!("{}",key);
            Ok(key.to_string())
        }

        else if translation_type == "2" {
            // TODO: Implement mnemonic return.
            // Here the key is a string. 
            // TODO Implement a new Dashmap with <String, String> type
            let person: [String; 6] = ["Santa ".to_string(),"The Elf ".to_string(),"Mr Snowman ".to_string(),
                                        "The Gingerbreadman ".to_string(),"Scrooge ".to_string(),"Rudolph ".to_string()];
            let connect: [String; 11] = ["was ".to_string(),"is ".to_string(),"likes ".to_string(),"hates ".to_string(),"prefers ".to_string(),
                                         "has been ".to_string(),"will be ".to_string(),"adores ".to_string(),"enjoys ".to_string(),"loves ".to_string(),"dislikes ".to_string()];
            let action: [String; 17] = ["cooking".to_string(),"singing".to_string(),"dancing".to_string(),"sleeping".to_string(),"celebrating".to_string(),
                                        "surprising".to_string(),"writing".to_string(),"hoping".to_string(),"lying".to_string(),"listening".to_string(),"offering".to_string(),
                                        "speaking".to_string(),"running".to_string(),"programming".to_string(),"snoring".to_string(),"entertaining".to_string(),"tickling".to_string()];

            use rand::Rng;

            let mut key_gen = "".to_string();
            key_gen.push_str(&person[rand::thread_rng().gen_range(0..6)]);
            key_gen.push_str(&connect[rand::thread_rng().gen_range(0..11)]);
            key_gen.push_str(&action[rand::thread_rng().gen_range(0..17)]);
            

            // let key = key_gen;
            println!("{}",key_gen);
            state.insert(key_gen.clone(), url);
            Ok(key_gen)
        }

        else if translation_type == "3" {
            // TODO: Implement emoji return.

            let theme: [String; 10] = ["🙂".to_string(),"🔴".to_string(),"⚠️".to_string(),
                                        "📌".to_string(),"😘".to_string(),"💙".to_string(),
                                        "👽".to_string(),"👻".to_string(),"🐉".to_string(),"❤️‍🔥".to_string()];
            
            let mut key_gen = "".to_string();

            use rand::Rng;
            //🐼🦄
            for _ in 0..10{
                let num2: usize = rand::thread_rng().gen_range(0..10);
                key_gen.push_str(&theme[num2]);
            }

            // let key = key_gen;
            println!("{}",key_gen);
            state.insert(key_gen.clone(), url);
            Ok(key_gen)
            // Err(rocket::response::status::BadRequest(Some("Emoji not implemented yet!")))
        }

        else {
            Err(rocket::response::status::BadRequest(Some("Invalid translation type!")))
        }
    }
}


// Add Api for login and register, so that user can add a custom url

#[get("/<key>")]
fn redirect(key: String, state: &rocket::State<dashmap::DashMap<String, String>>) -> Result<rocket::response::Redirect, rocket::response::status::NotFound<&str>> {
    // TODO: Implement click tracking here.
    state
        .get(&key)
        .map(|url| rocket::response::Redirect::to(url.clone()))
        .ok_or(rocket::response::status::NotFound("Invalid or expired link!"))

}