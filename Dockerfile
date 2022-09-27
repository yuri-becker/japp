FROM rust
EXPOSE 8080

# Build project
ENV SOURCES=/sources
ENV ROCKET_PORT=8080
ENV ROCKET_ADDRESS=0.0.0.0
RUN mkdir -p $SOURCES
ADD /assets/icons $SOURCES
WORKDIR $SOURCES
RUN cargo build --release

# Run
CMD cargo run --release

