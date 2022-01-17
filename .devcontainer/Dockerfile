FROM rust:1.58.0

# install rust components
RUN rustup component add rustfmt clippy

# install nodejs
RUN curl -fsSL https://deb.nodesource.com/setup_16.x | bash -
RUN apt-get install -y nodejs

# hide "new version of npm available" notification from CLI
RUN npm config set update-notifier false

# install wasm-pack
RUN npm install -g wasm-pack@0.10.2