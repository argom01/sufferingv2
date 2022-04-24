FROM rust:1.59 as builder

RUN USER=root cargo new --bin latin-website
WORKDIR ./latin-website
COPY ./Cargo.toml ./Cargo.toml
RUN cargo build --release
RUN rm src/*.rs

ADD . ./

RUN rm ./target/release/deps/latin_website*
RUN cargo build --release

FROM debian:buster-slim as runner
ARG APP=/usr/src/website

RUN apt-get update -y
RUN apt remove mysql-server
RUN apt autoremove
RUN apt-get remove --purge mysql\*
RUN apt-get install wget -y
# Required to install mysql
# default-libmysqlclient-dev necessary for diesel's mysql integration
RUN apt-get install -y default-libmysqlclient-dev
# Add Oracle MySQL repository
RUN apt-get update
RUN apt-get install -y gnupg lsb-release wget
RUN wget https://dev.mysql.com/get/mysql-apt-config_0.8.22-1_all.deb
RUN DEBIAN_FRONTEND=noninteractive dpkg -i mysql-apt-config_0.8.22-1_all.deb
RUN apt update
# Add Oracle's libmysqlclient-dev
RUN apt-get install -y libmysqlclient-dev mysql-community-client

EXPOSE 8000

ENV TZ=Etc/UTC \
    APP_USER=appuser \
    RUST_LOG=actix=info

RUN groupadd $APP_USER \
    && useradd -g $APP_USER $APP_USER \
    && mkdir -p ${APP}

COPY --from=builder /latin-website/target/release/latin-website ${APP}/latin-website

RUN chown -R $APP_USER:$APP_USER ${APP}

USER $APP_USER
WORKDIR ${APP}

CMD ["./latin-website"]