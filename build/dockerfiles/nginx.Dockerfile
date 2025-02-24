FROM node:23-bookworm-slim AS buildfrontend

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

FROM nginx:alpine AS final

COPY --from=buildfrontend /frontend /usr/share/nginx/html