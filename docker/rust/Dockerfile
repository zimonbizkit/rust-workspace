FROM rustlang/rust:nightly

RUN cargo install cargo-watch
RUN apt update
RUN apt install -y lynx
ENV USER=root

WORKDIR .

VOLUME ["/home/rust"]