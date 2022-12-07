# URL Shortener

A url shortener application using Rust programming language.


## How to run
1. Install Rust
2. Clone this repository
3. Run `cargo run` in the root directory of the project
4. Open `localhost:8080` in your browser and start using the app.


## Run in Docker
1. Install Docker
2. Run `docker build -t url-shortener .` in the root directory of the project
3. Run `docker run -p 8080:8080 url-shortener`
4. Open `localhost:8080` in your browser and start using the app.

## For Further Testing
1. For further testing, you can test the app by running `python3 test_scripts/test.py` in the root directory of the project
2. Check the state in browser by going to `localhost:8080/state`