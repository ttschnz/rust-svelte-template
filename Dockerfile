# build the backend
FROM rust:1.67 as api-builder
WORKDIR /usr/src/api
COPY ./api/Cargo.toml .
COPY ./api/src ./src
RUN cargo install --path .
RUN cargo build --release

# build the frontend
FROM node:16-alpine as svelte-builder
WORKDIR /usr/src/app
COPY ./app/rollup.config.js ./
COPY ./app/package*.json ./

RUN npm ci
COPY ./app/src ./src
COPY ./app/public ./public

RUN npm run-script build

# run the backend and serve the frontend
FROM debian:bullseye-slim
COPY --from=api-builder /usr/src/api/target/release/api /usr/local/bin/api
COPY --from=svelte-builder /usr/src/app/public /www
WORKDIR /usr/local/bin/
EXPOSE 80
CMD [ "api" ]