/*
    Request Handler for the server.
    Author(s): Prithvi <prithvi_raj@gwu.edu> , Sam Ul Haque <sam.ulhaque@gwu.edu>
    Rust Group for ASP Fall 2022
*/

#[macro_use]
extern crate rocket; ///     |||||||||||||||||||||||||| ROCKET ||||||||||||||||||
mod url_validation;
use rand::Rng; // Bring trait into scope.


// #[derive(Debug)]
struct TrackerStruct {
    url: String,
    count: u32,
}



/// ||||||||||||||||||||||||              TRAITS THING  ||||||||||||||||||||||||||||||||||||
use std::fmt;

impl fmt::Display for TrackerStruct
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "URL: {}, Count: {}", self.url, self.count)
    }
}


#[launch]
fn rocket() -> _ {

    rocket::build()
        .manage(dashmap::DashMap::<String, TrackerStruct>::new()) // |||||||||||||||||||||||    DASHMAP |||||||||||||||||||||||
        .mount("/", routes![
            shorten,
            redirect,
            track,
        ])
        .mount(
            "/",
            rocket::fs::FileServer::from("static_content/")
        )
}



#[get("/shorten?<url>&<translation_type>")]
fn shorten<>( 
        url: String,
        translation_type : String,
        state: & rocket::State<dashmap::DashMap<String, TrackerStruct>>,
    ) -> Result<String, rocket::response::status::BadRequest<& str>> { // |||||||||||||||||||||||||||||||||||||||||||||| MONADS |||||||||||||||\\\\\\\

    if url.is_empty() {
        Err(rocket::response::status::BadRequest(Some("URL is empty!")))
    
    } else if url_validation::period_in_url(&url) == false {
        Err(rocket::response::status::BadRequest(Some("URL is invalid!"))) // ||||||||||| ERROR ||||||||||||||||||||||||
    
    } else {
        
        let url = url_validation::add_protocol(&url);

        if translation_type == "1" {
            use rand::Rng;
            let key: u32 = rand::thread_rng().gen();
            
            
            let datum = TrackerStruct {
                url: url.clone(),
                count: 0,
            };

            state.insert(key.to_string(), datum  );
            
            Ok(key.to_string()) // ||||||||||||||||||||||||||||    SOME (T) THINGY ||||||||||||||||||||||||||||||||||||
        }

        else if translation_type == "2" {

            let mut words: std::collections::HashMap<String, Vec<String>> = std::collections::HashMap::new();

            for file_name in ["conjunctions", "nouns", "verbs"].iter() {
                let c_dir = std::env::current_dir().unwrap().to_str().unwrap().to_string();
                let path_string = format!("{}/wordlists/themes/holidays/{}.txt", c_dir, file_name); // Improve this, use generic methods instead of hacks.
                let file_content = std::fs::read_to_string(&path_string).expect("Path {path_string} is invalid, no such file/directory exists.");
                let word_list_vec: Vec<String> = file_content.split("\n").map(String::from).collect();
                words.insert( file_name.to_string() , word_list_vec );
            }
            

            let mut _key: String = String::new();
            for file_name in ["nouns", "conjunctions", "verbs"].iter() {
                let k = words.get(&file_name.to_string()).unwrap();
                _key.push_str(&k[rand::thread_rng().gen_range(0..k.len())]);

            }

            let datum = TrackerStruct {
                url: url.to_string().clone(),
                count: 0,
            };
            
            state.insert(_key.clone(), datum  );
            

            
            Ok(_key)
        }

        else if translation_type == "3" {

            // ||||||||||||||||||||||||||  EMOJI USE |||||||||||||||||||||||||||||||||||||||||||||||||||
            let c_dir = std::env::current_dir().unwrap().to_str().unwrap().to_string();
            let path_string = format!("{}/wordlists/emojis/emojis.txt", c_dir); // Improve this, use generic methods instead of hacks.
            let file_content = std::fs::read_to_string(&path_string).expect("Path {path_string} is invalid, no such file/directory exists.");
            let emojis: Vec<String> = file_content.split("\n").map(String::from).collect();

            // Choose a random emojis
            let mut _key: String = String::new();
            for _ in 0..7 {
                _key.push_str(&emojis[rand::thread_rng().gen_range(0..emojis.len())]);
            }

            let datum = TrackerStruct {
                url: url.to_string().clone(),
                count: 0,
            };

            
            state.insert(_key.clone(), datum );

            Ok(_key)
        }

        else {
            Err(rocket::response::status::BadRequest(Some("Invalid translation type!")))
        }
    }
}


// Add Api for login and register, so that user can add a custom url

#[get("/<key>")]
fn redirect<>(
    key: String,
    state: & rocket::State<dashmap::DashMap<String, TrackerStruct>>,
) -> Result<rocket::response::Redirect, rocket::response::status::NotFound<String>> {
    // TODO: Implement click tracking here.
    println!("Entering Redirect");
    
    // Increase the count forthe key
    let datum = state.get_mut(&key.clone());

    match datum {
        Some(mut datum) => {
            datum.count += 1;
            Ok(rocket::response::Redirect::to(datum.url.to_string()))
        },
        None => Err(rocket::response::status::NotFound( "Reload the page.".to_string() ) )//format!( "Key is: {} and state is: {:?}" ,  key.clone() , state)  )),
    }
    
}


#[get("/track?<hkey>")]
fn track<>(
    hkey: String,
    state: & rocket::State< dashmap::DashMap::<String, TrackerStruct> >,
) -> Result<String, rocket::response::status::BadRequest<& str>> {


    let stats = state.get(&hkey);
    if stats.is_none() {
        Err(rocket::response::status::BadRequest(Some("Invalid or expired link!")))
    }
    else {
        let stats = stats.unwrap();
        let stats = stats.value();
        let stats = format!("{}prithviandsamadandkamal{}", stats.url, stats.count);
        Ok(stats)
    }
}