FROM rust:1.94

ARG USER_UID=1000
ARG USER_GID=1000

RUN groupadd -g $USER_GID jehan \
    && useradd -m -u $USER_UID -g $USER_GID -s /bin/bash jehan

WORKDIR /app

RUN mkdir -p /app/.cargo /app/.rustup \
    && chown -R jehan:jehan /app

USER jehan

ENV CARGO_HOME=/app/.cargo
ENV RUSTUP_HOME=/app/.rustup
ENV PATH="/app/.cargo/bin:${PATH}"

RUN rustup default stable
RUN cargo install cargo-edit

COPY --chown=jehan:jehan Cargo.toml Cargo.lock* ./
COPY --chown=jehan:jehan src ./src

RUN cargo fetch
RUN cargo build

CMD ["sleep", "infinity"]