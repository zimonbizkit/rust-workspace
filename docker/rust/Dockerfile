FROM rustlang/rust:nightly

RUN cargo install cargo-watch
RUN apt update
RUN apt install -y lynx && apt install -y valgrind
RUN wget -qO - "https://github.com/exercism/cli/releases/download/v3.0.11/exercism-linux-64bit.tgz" | tar zxvf - && mv exercism /usr/local/bin
RUN exercism configure --token=YOURTOKEN --workspace=/home/user/rust/projects/exercism
RUN mkdir -p /root/exercism/rust
ENV USER=root
ENV RUST_BACKTRACE=1

WORKDIR .

VOLUME ["/home/rust"]