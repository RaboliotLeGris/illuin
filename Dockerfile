FROM rustlang/rust:nightly-buster-slim AS builder

RUN apt-get update && \
	apt-get install -y git && \
	rm -rf /var/lib/apt/lists/*

WORKDIR /build
RUN git clone https://github.com/RaboliotLeGris/illuin.git
WORKDIR /build/illuin
RUN cargo build --release

FROM debian:buster-slim

WORKDIR /
COPY entrypoint.sh .

WORKDIR /illuin
COPY --from=builder /build/illuin/target/release/illuin illuin
COPY --from=builder /build/illuin/static static
COPY --from=builder /build/illuin/templates templates

ENTRYPOINT ["/entrypoint.sh"]
CMD ["./illuin"]