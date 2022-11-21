FROM rust as build


# Pull our Repo
RUN git clone https://github.com/Prithvi0709/urlShortener.git
RUN git checkout samad

# Install this thing npm init svelte@next svelte (Possible version issue here , watch for it, nodejs version should ve < 16)

# Build our Project
RUN cd urlShortener && cargo build --release
RUN cd ../


CMD ["cd urlShortener && cargo run"]