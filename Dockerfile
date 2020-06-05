# THe builder container
FROM rust:slim as builder
WORKDIR /app

# Make sure all dependencies are installed
RUN apk add --update musl-dev pkgconf npm libsass make g++ git

# Install typescript
RUN npm i -g typescript

# Copy source code over
COPY . .
# Compile and link the app
RUN cargo install --path .

# The app container
FROM gcr.io/distroless/cc
LABEL Author="Zachary Kohnen"
WORKDIR /app

# Copy binary to the app
COPY --from=builder /usr/local/cargo/bin/sxfs /

# Expose the data
VOLUME [ "/app/data" ]

# Expose the port
EXPOSE 8000

# Run the app
CMD ["./sxfs"]