FROM debian:10

RUN apt-get update && apt-get -y install libpq5

COPY server /opt/server

EXPOSE 50051

ENTRYPOINT /opt/server