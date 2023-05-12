# Use the base builder image for Rust projects as the builder stage
FROM ghcr.io/nottirb/fuzz-rust:latest as builder

# Add source to the build stage
ADD . /bimap-rs
WORKDIR /bimap-rs

# Build the fuzz targets
RUN cd ./fuzz && cargo +nightly hfuzz build

# Use Ubuntu 20.04 as the final image
FROM ubuntu:20.04

# Copy the fuzz targets from the builder stage
COPY --from=builder bimap-rs/fuzz/hfuzz_target/x86_64-unknown-linux-gnu/release/fuzz_get_by_left /
COPY --from=builder bimap-rs/fuzz/hfuzz_target/x86_64-unknown-linux-gnu/release/fuzz_get_by_right /
COPY --from=builder bimap-rs/fuzz/hfuzz_target/x86_64-unknown-linux-gnu/release/fuzz_insert /