# Plain docker container with no overhead
FROM scratch
# Copy the executable over
COPY ./target/x86_64-unknown-linux-musl/debug/spe3d /spe3d
# Expose the webserver & websocket port
EXPOSE 8000 8001
# Finally, we configure our binary as entrypoint (you need to adjust this too)
ENTRYPOINT ["./spe3d"]