ARG rust_image=rust:1.52-buster
FROM ${rust_image} AS build

ENV PKG_CONFIG_ALLOW_CROSS=1

WORKDIR /usr/src/npm-resource

RUN cargo install toml-cli

COPY ./apps/npm-resource/Cargo* ./

# We make a fake empty project and build it to make use of
# containers layer caching, this way we invalidate this layer only
# if Cargo.toml is changed and dependencies will be prebuilt
RUN mkdir src \
    &&  echo "fn main() {}" > src/main.rs \
    &&  cargo build --release \
    &&  rm -rf src \
    &&  rm -rf target/release/deps/npm_resource*

COPY ./apps/npm-resource/src src
COPY ./apps/npm-resource/tests tests
COPY ./version version

# Real build is here
RUN toml set Cargo.toml package.version $(cat version) > Cargo.versioned.toml \
    &&  mv Cargo.versioned.toml Cargo.toml \
    &&  cargo test \
    &&  cargo build --release

FROM gcr.io/distroless/cc-debian10

COPY --from=build /usr/src/npm-resource/target/release/in /opt/resource/in
COPY --from=build /usr/src/npm-resource/target/release/out /opt/resource/out
COPY --from=build /usr/src/npm-resource/target/release/check /opt/resource/check
