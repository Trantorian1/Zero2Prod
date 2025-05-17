FROM clux/muslrust:1.85.1-stable AS base
WORKDIR /app

# Note that we do not install cargo chef, sccache or mold through cargo to 
# avoid having to compile them from source

ENV VERSION_SCCACHE="0.10.0"
ENV VERSION_CHEF="0.1.71"
ENV VERSION_MOLD="2.39.1"

ENV SCCACHE_URL=https://github.com/mozilla/sccache/releases/download/v$VERSION_SCCACHE/sccache-v$VERSION_SCCACHE-x86_64-unknown-linux-musl.tar.gz
ENV SCCACHE_DIR=/sccache
ENV SCCACHE=sccache-v$VERSION_SCCACHE-x86_64-unknown-linux-musl/sccache
ENV RUSTC_WRAPPER=/usr/local/bin/sccache

ENV CHEF_URL=https://github.com/LukeMathWalker/cargo-chef/releases/download/v$VERSION_CHEF/cargo-chef-x86_64-unknown-linux-gnu.tar.gz

ENV MOLD_URL=https://github.com/rui314/mold/releases/download/v$VERSION_MOLD/mold-$VERSION_MOLD-x86_64-linux.tar.gz

ENV WGET="-O- --timeout=10 --waitretry=3 --retry-connrefused --progress=dot:mega"

RUN apt-get update -y  && apt-get install -y wget clang

RUN mkdir $SCCACHE_DIR
RUN wget $WGET $SCCACHE_URL | tar -C /bin -xzvpf -
RUN wget $WGET $CHEF_URL | tar -C /bin -xzvpf -
RUN wget $WGET $MOLD_URL | tar -C /usr/local --strip-components=1 --no-overwrite-dir -xzvpf -

# Force `rustup` to sync the toolchain in the base `chef` layer so that it 
# doesn't happen more than once
COPY rust-toolchain.toml .
RUN rustup show active-toolchain

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

WORKDIR /app

ENTRYPOINT ["tini", "--", "zero2prod"]
