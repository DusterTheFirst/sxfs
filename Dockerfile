# THe builder container
FROM rust:slim as builder
WORKDIR /app

RUN apt update && apt install -y curl build-essential

# Make sure all dependencies are installed
RUN curl -sL https://deb.nodesource.com/setup_10.x | bash -
RUN apt install -y nodejs

# Install typescript
RUN npm i -g typescript

# Build dummy project with just dependencies
COPY Cargo.toml .
COPY rust-toolchain .
# Create dummy files
RUN echo "fn main() {}" > dummymain.rs
RUN echo "" > dummylib.rs
RUN echo "fn main() {}" > dummybuild.rs
# Point to dummy files
RUN sed -i 's#src/main.rs#dummymain.rs#; s#src/lib.rs#dummylib.rs#; s#build.rs#dummybuild.rs#' Cargo.toml
# Build
RUN cargo build --release
# Revert dummy files
RUN sed -i 's#dummymain.rs#src/main.rs#; s#dummylib.rs#src/lib.rs#; s#dummybuild.rs#build.rs#' Cargo.toml
# Delete dummy files
RUN rm dummy*.rs

# Copy source code over
COPY . .
# Compile the app
RUN cargo build --release

# The app container
FROM gcr.io/distroless/cc-debian10
LABEL Author="Zachary Kohnen"
WORKDIR /app

# Copy binary to the app
COPY --from=builder /app/target/release/sxfs /app

# Expose the data
VOLUME [ "/app/data" ]

# Expose the port
EXPOSE 8000

# Run the app
CMD ["./sxfs"]