FROM node:alpine as builder

WORKDIR /build
RUN corepack enable pnpm

COPY ./frontend/package.json /build/
COPY ./frontend/pnpm-lock.yaml /build

RUN pnpm i

COPY ./frontend /build
RUN pnpm build

FROM nginx:alpine

COPY ./frontend/nginx.conf /etc/nginx/conf.d/default.conf
COPY --from=builder /build/build /usr/share/nginx/html