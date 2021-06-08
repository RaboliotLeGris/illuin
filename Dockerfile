FROM 	rustlang/rust:nightly-buster-slim@sha256:2d9404f009d7816e47b0842b4bf54610bb40582e32f9a5217aa0a9678ddcb230 AS builder

RUN 	apt-get update && \
	    apt-get install --no-install-recommends -y git && \
		rm -rf /var/lib/apt/lists/*

WORKDIR	/build
RUN     git clone https://github.com/RaboliotLeGris/illuin.git
WORKDIR	/build/illuin
RUN	    cargo build --release

FROM	debian:buster-slim@sha256:33965bf1eaadb19ce2f9396595c4a669e3e04c1ab8cc073b8929f529c58404bb


WORKDIR	/illuin
COPY	entrypoint.sh .
COPY	--from=builder /build/illuin/target/release/illuin illuin
COPY	--from=builder /build/illuin/templates templates

RUN	    groupadd -g 1000 illuin && \
	    useradd -r -u 1000 -g illuin illuin && \
	    chown -R illuin:illuin /illuin
USER 	illuin

ENTRYPOINT ["/illuin/entrypoint.sh"]
CMD 	   ["./illuin"]
