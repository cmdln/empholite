FROM debian:buster-slim

EXPOSE 8989

RUN apt update && apt -y install ca-certificates libpq5

ENV CLIENT_PATH /client
ENV STATIC_PATH /static
ENV RUST_LOG logger=info,empholite=info

RUN mkdir /client
RUN mkdir /static
ADD static/ /static/
ADD target/release/empholite /
ADD client/pkg/bundle.js /client/
ADD client/pkg/client.js /client/
ADD client/pkg/client_bg.wasm /client/

CMD ["/empholite"]
