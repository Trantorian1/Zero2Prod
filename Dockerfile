FROM clux/muslrust:1.85.0-stable AS base
WORKDIR /app

# Note that we do not install cargo chef and sccache through cargo to avoid
# having to compile them from source
ENV SCCACHE_URL=https://github.com/mozilla/sccache/releases/download/v0.10.0/sccache-v0.10.0-x86_64-unknown-linux-musl.tar.gz
ENV SCCACHE_TAR=sccache-v0.10.0-x86_64-unknown-linux-musl.tar.gz
ENV SCCACHE_BIN=/bin/sccache
ENV SCCACHE_DIR=/sccache
ENV SCCACHE=sccache-v0.10.0-x86_64-unknown-linux-musl/sccache
ENV CHEF_URL=https://github.com/LukeMathWalker/cargo-chef/releases/download/v0.1.71/cargo-chef-x86_64-unknown-linux-gnu.tar.gz
ENV CHEF_TAR=cargo-chef-x86_64-unknown-linux-gnu.tar.gz
ENV RUSTC_WRAPPER=/bin/sccache

RUN apt-get update -y  && apt-get install -y wget
RUN wget $SCCACHE_URL && tar -xvpf $SCCACHE_TAR && mv $SCCACHE $SCCACHE_BIN && mkdir sccache
RUN wget $CHEF_URL && tar -xvpf $CHEF_TAR && mv cargo-chef /bin

# Step 1: cache dependencies
FROM base AS planner

COPY . .
RUN --mount=type=cache,target=$SCCACHE_DIR,sharing=locked \
	--mount=type=cache,target=/usr/local/cargo/registry \
	cargo chef prepare --recipe-path recipe.json

# Step 2: compile project
FROM base AS builder

COPY --from=planner /app/recipe.json .
RUN --mount=type=cache,target=$SCCACHE_DIR,sharing=locked \
	--mount=type=cache,target=/usr/local/cargo/registry \
	cargo chef cook --target=x86_64-unknown-linux-musl --release --recipe-path recipe.json

COPY . .
RUN --mount=type=cache,target=$SCCACHE_DIR,sharing=locked \
	--mount=type=cache,target=/usr/local/cargo/registry \
	cargo build --target=x86_64-unknown-linux-musl --release

# Step 3: tini
FROM builder AS tini

ENV TINI_VERSION=v0.19.0
ADD https://github.com/krallin/tini/releases/download/${TINI_VERSION}/tini-static /bin/tini
RUN chmod +x /bin/tini

# Step 4: runner
FROM scratch
WORKDIR /bin

COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/zero2prod .
COPY --from=tini /bin/tini .

ENTRYPOINT ["tini", "--", "zero2prod"]
