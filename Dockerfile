FROM rust:latest AS builder

RUN update-ca-certificates

RUN rustup target add thumbv6m-none-eabi
