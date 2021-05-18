FROM 	rustlang/rust:nightly-buster-slim AS builder

RUN 	apt-get update && \
	    apt-get install --no-install-recommends -y git && \
		rm -rf /var/lib/apt/lists/*

WORKDIR	/build
RUN     git clone https://github.com/RaboliotLeGris/illuin.git
WORKDIR	/build/illuin
RUN	    cargo build --release

FROM	debian:buster-slim


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
