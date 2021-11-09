FROM 	rustlang/rust:nightly-bullseye-slim@sha256:21e80749e9b0c606944199f6becf1cfbec842255e4c363b93b963e2aacc9dbe0 AS builder

WORKDIR	/build
COPY 	. .

RUN	    cargo build --release

FROM	debian:buster-slim@sha256:33965bf1eaadb19ce2f9396595c4a669e3e04c1ab8cc073b8929f529c58404bb

WORKDIR	/illuin
COPY	entrypoint.sh .
COPY	--from=builder /build/target/release/illuin illuin
COPY	--from=builder /build/templates templates

RUN	    groupadd -g 1000 illuin && \
	    useradd -r -u 1000 -g illuin illuin && \
	    chown -R illuin:illuin /illuin && \
		chmod +x /illuin/entrypoint.sh
USER 	illuin

ENTRYPOINT ["/illuin/entrypoint.sh"]
CMD 	   ["./illuin"]
