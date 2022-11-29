FROM rust as build


# Pull our Repo
RUN git clone https://github.com/Prithvi0709/urlShortener.git

# Build our Project
RUN cd urlShortener && cargo build --release
RUN cp /urlShortener/target/release/urlShortener /urlShortener/src

EXPOSE 8080

CMD ["bash" , "-c" , "cd /urlShortener/ && cargo run --release"]