FROM node:16-alpine as svelte-builder
WORKDIR /usr/src/app
COPY rollup.config.js ./
COPY package*.json ./

RUN npm ci
COPY ./src ./src
COPY ./public ./public

RUN npm run-script build

FROM nginx:1.21-alpine
COPY --from=svelte-builder /usr/src/app/public /usr/share/nginx/html
EXPOSE 80
CMD ["nginx", "-g", "daemon off;"]
