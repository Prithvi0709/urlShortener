/*
    Request Handler for the server.
    Author(s): Prithvi <prithvi_raj@gwu.edu> , Sam Ul Haque <sam.ulhaque@gwu.edu>
    Rust Group for ASP Fall 2022
*/

#[macro_use]
extern crate rocket;
mod url_validation;
use rand::Rng; // Bring trait into scope.

struct TrackerStruct {
    url: String,
    count: u32,
}

#[launch]
fn rocket() -> _ {

    rocket::build()
        .manage(dashmap::DashMap::<String, TrackerStruct>::new())
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
    ) -> Result<String, rocket::response::status::BadRequest<& str>> {

    if url.is_empty() {
        Err(rocket::response::status::BadRequest(Some("URL is empty!")))
    
    } else if url_validation::period_in_url(&url) == false {
        Err(rocket::response::status::BadRequest(Some("URL is invalid!")))
    
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
            
            Ok(key.to_string())
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
) -> Result<rocket::response::Redirect, rocket::response::status::NotFound<& str>> {
    // TODO: Implement click tracking here.
    println!("Entering Redirect");
    
    // Increase the count forthe key
    let datum = state.get_mut(&key);

    match datum {
        Some(mut datum) => {
            datum.count += 1;
            Ok(rocket::response::Redirect::to(datum.url.to_string()))
        },
        None => Err(rocket::response::status::NotFound("Key not found!")),
    }
    // datum.count += 1;

    // datum.url
    
    // println!("Done adding tracker stats");
    // state
    //     .get(&key)
    //     .map(|tstruct| rocket::response::Redirect::to(tstruct.url.clone()))
    //     .ok_or(rocket::response::status::NotFound("Invalid or expired link!"))

    
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