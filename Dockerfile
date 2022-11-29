FROM rust as build


# Pull our Repo
RUN git clone https://github.com/Prithvi0709/urlShortener.git

# Build our Project
RUN cd urlShortener && cargo build --release
RUN cd ../


CMD ["cd urlShortener && cargo run"]
