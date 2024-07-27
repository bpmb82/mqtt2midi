FROM debian:bookworm-slim

# Install Rust system-wide
ENV RUSTUP_HOME=/opt/rust
ENV CARGO_HOME=/opt/rust
ENV CARGO_REGISTRIES_CRATES_IO_PROTOCOL=sparse

RUN apt update && apt install -y gcc curl libasound2-dev libasound2 openssl pkg-config && \
    curl https://sh.rustup.rs -sSf | sh -s -- -y --no-modify-path
ENV PATH="/opt/rust/bin:${PATH}"

COPY entrypoint.sh /entrypoint.sh

WORKDIR /mqtt2midi

CMD [ "/entrypoint.sh" ]