FROM rust:1.51.0
RUN curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
RUN cargo install cargo-generate
RUN rustup component add rls rust-src rust-analysis clippy

RUN curl -fsSL https://deb.nodesource.com/setup_15.x | bash -
RUN apt-get install -y nodejs
# temporary to hide new-npm-version notification from CLI
RUN npm install -g npm@7.14.0

ENV USER=root