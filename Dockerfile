# Plain docker container with no overhead
FROM scratch
# Copy the executable over
COPY ./target/x86_64-unknown-linux-musl/release/spe3d /spe3d
# Copy the frontend over
COPY ./www/ /www/
# Expose the webserver & websocket portEXPOSE 8000 8001
# Finally, we configure our binary as entrypoint (you need to adjust this too)
ENTRYPOINT ["./spe3d"]