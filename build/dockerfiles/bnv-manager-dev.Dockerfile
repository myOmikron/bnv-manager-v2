ARG RUST_VERSION=1.85.0

FROM rust:${RUST_VERSION}-slim-bookworm AS buildrust

WORKDIR /app

RUN <<EOF
apt-get update
apt-get install -y openssl libssl-dev pkg-config
EOF

RUN --mount=type=bind,source=mailcow/,target=mailcow/ \
    --mount=type=bind,source=webserver/,target=webserver/ \
    --mount=type=bind,source=conf_updater/,target=conf_updater/ \
    --mount=type=bind,source=conf_updater_common/,target=conf_updater_common/ \
    --mount=type=bind,source=Cargo.toml,target=Cargo.toml \
    --mount=type=bind,source=Cargo.lock,target=Cargo.lock \
    --mount=type=cache,target=/app/target/ \
    --mount=type=cache,target=/usr/local/cargo/registry/ \
    <<EOF
set -e
cargo build --locked -p webserver
cp ./target/debug/webserver /bin/server
EOF


FROM debian:bookworm-slim AS final

RUN <<EOF
apt-get update
apt-get install -y libssl-dev nginx ca-certificates
EOF

# Copy startup script
COPY ./build/bnv-manager/startup.sh /
RUN chmod +x /startup.sh

# Copy the executable from the "build" stage.
COPY --from=buildrust /bin/server /bin/

# Create a non-privileged user that the app will run under.
# See https://docs.docker.com/develop/develop-images/dockerfile_best-practices/   #user
ARG UID=1000
RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "${UID}" \
    appuser

RUN mkdir /var/lib/bnv-manager /migrations
RUN chown ${UID} /var/lib/bnv-manager /migrations

# What the container should run when it is started.
CMD ["/startup.sh"]