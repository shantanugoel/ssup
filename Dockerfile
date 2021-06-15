# Base image for rapsberrypi 3 target
FROM rustembedded/cross:aarch64-unknown-linux-gnu

# Install libdbus libraries and pkg-config

RUN dpkg --add-architecture arm64 && \
	    apt-get update && \
	    apt-get install --assume-yes apt-utils pkg-config

RUN apt-get install --assume-yes libdbus-1-dev libdbus-1-dev:arm64