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