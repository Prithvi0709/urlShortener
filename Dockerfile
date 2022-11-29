FROM rust as build


# Pull our Repo
# RUN git clone https://github.com/Prithvi0709/urlShortener.git
# RUN git checkout master/
# Build our Project

COPY *.toml /urlShortener/
COPY src /urlShortener/src
COPY static_content /urlShortener/static_content
COPY wordlists /urlShortener/wordlists


RUN ls urlShortener
RUN cd urlShortener && cargo build --release
RUN cp /urlShortener/target/release/urlShortener /urlShortener/src

EXPOSE 8080

CMD ["bash" , "-c" , "cd /urlShortener/ && cargo run --release"]