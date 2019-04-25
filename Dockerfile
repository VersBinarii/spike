FROM rust:1.34

ENV APP_ROOT /usr/src/spike_rs

RUN mkdir -p ${APP_ROOT}
WORKDIR ${APP_ROOT}
COPY . ${APP_ROOT}

RUN cargo install --path .

CMD ["spike_rs"]

EXPOSE 8080