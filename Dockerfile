# THe builder container
FROM rust:alpine as builder
WORKDIR /app

# Make sure all dependencies are installed
RUN apk add --update musl-dev pkgconf npm libsass make g++ git

# Install typescript
RUN npm i -g typescript

# Copy source code over
COPY . .
# Compile and link the app
RUN RUSTFLAGS="-C target-feature=-crt-static" cargo install --path .

# The app container
FROM alpine:latest
LABEL Author="Zachary Kohnen"
WORKDIR /app

RUN apk add --update libgcc

# Copy binary to the app
COPY --from=builder /usr/local/cargo/bin/sxfs /usr/local/bin/sxfs

# Expose the data
VOLUME [ "/app/data" ]

# Expose the port
EXPOSE 8000

# Run the app
CMD ["sxfs"]