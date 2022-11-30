use rand::distributions::{Alphanumeric, DistString};

pub fn generate_random_string() -> Option<String> {
    let string = Alphanumeric.sample_string(&mut rand::thread_rng(), 16);
    // Implement logic to make it sound like a mnemonic.
    // Also check if the new string is already in the database.
    if string.contains('u') {
        return None;
    }

    Some(string)
}


/*
    for i in  {1..100}; do curl -I 0.0.0.0:8080; done;
    for i in  {1..100}; do curl -I https://86b6-161-253-74-219.ngrok.io
 */