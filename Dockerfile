# syntax=docker/dockerfile:1
# NOTE: This Dockerfile is meant to EXTEND the provided filler docker image.
# Place this file alongside the game_engine, maps/, robots/, and solution/ directories.
# Then: docker build -t filler .
# And:  docker run -v "$(pwd)/solution":/filler/solution -it filler

FROM rust:1.78-slim AS builder
WORKDIR /filler/solution
COPY solution/ .
RUN cargo build --release

FROM debian:bookworm-slim
WORKDIR /filler
# Copy game artifacts from provided image (user must supply game_engine, maps, robots)
COPY --from=builder /filler/solution/target/release/filler_robot /filler/solution/filler_robot
COPY game_engine /filler/game_engine
COPY maps /filler/maps
COPY robots /filler/robots
RUN chmod +x /filler/game_engine
CMD ["/bin/bash"]
