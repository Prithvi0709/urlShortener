mod mnemonic_generator;

fn main() {
    println!("{}", mnemonic_generator::generate_random_string().unwrap_or_default());
}
