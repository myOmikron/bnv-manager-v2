FROM node:24-bookworm-slim@sha256:03eae3ef7e88a9de535496fb488d67e02b9d96a063a8967bae657744ecd513f2 AS buildfrontend

WORKDIR /app

COPY ./frontend/package.json .
COPY ./frontend/package-lock.json .
COPY ./frontend/ .

RUN --mount=type=cache,target=./node_modules/ \
    <<EOF
set -e
npm clean-install
npm run build
mv ./dist /frontend
EOF

FROM debian:bookworm-slim@sha256:4724b8cc51e33e398f0e2e15e18d5ec2851ff0c2280647e1310bc1642182655d AS buildswagger

WORKDIR /app

RUN <<EOF
set -e
apt-get update
apt-get install -y wget
EOF

RUN <<EOF
set -e
wget https://github.com/swagger-api/swagger-ui/archive/refs/tags/v5.18.2.tar.gz
tar xf v5.18.2.tar.gz
mv swagger-ui-5.18.2/dist swagger-ui
EOF


FROM  dhi.io/nginx:1.29@sha256:d457a278247531873df03b7fdaabd96de0b530624f8e72c73d41e6b81c4487dd AS final
LABEL org.opencontainers.image.source=https://github.com/myOmikron/bnv-manager-v2

COPY --from=buildfrontend /frontend /usr/share/nginx/html/frontend
COPY --from=buildswagger /app/swagger-ui /usr/share/nginx/html/swagger-ui
COPY ./build/nginx/swagger-initializer.js /usr/share/nginx/html/swagger-ui/
