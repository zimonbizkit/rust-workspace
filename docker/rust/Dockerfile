FROM rustlang/rust:nightly

RUN cargo install cargo-watch
RUN apt update
RUN apt install -y lynx
ENV USER=root
ENV RUST_BACKTRACE=1

WORKDIR .

VOLUME ["/home/rust"]