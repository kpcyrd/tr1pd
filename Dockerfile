FROM rust:latest
RUN apt-get -q update \
    && apt-get install -qy libsodium-dev
WORKDIR /app
COPY . .
RUN cargo build --release --verbose
FROM busybox:1-glibc
COPY --from=0 /lib/x86_64-linux-gnu/libdl.so.2 \
              /lib/x86_64-linux-gnu/librt.so.1 \
              /lib/x86_64-linux-gnu/libgcc_s.so.1 \
              /lib/x86_64-linux-gnu/
COPY --from=0 /usr/lib/x86_64-linux-gnu/libsodium.so.18 /usr/lib/x86_64-linux-gnu/
COPY docker-entry.sh /docker-entry.sh
COPY --from=0 /app/target/release/tr1pd /app/target/release/tr1pctl /usr/local/bin/
VOLUME /etc/tr1pd
VOLUME /var/lib/tr1pd
VOLUME /run/tr1pd
ENV TR1PD_DATADIR=/var/lib/tr1pd
ENV TR1PD_SOCKET=ipc:///run/tr1pd/tr1pd.sock
ENTRYPOINT ["/docker-entry.sh"]
CMD ["tr1pd"]
