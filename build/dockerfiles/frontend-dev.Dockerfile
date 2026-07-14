FROM node:24@sha256:5711a0d445a1af54af9589066c646df387d1831a608226f4cd694fc59e745059 AS final

WORKDIR /app

RUN <<EOF
set -e
apt-get update
apt-get install -y wget default-jre-headless
EOF

CMD ["npm", "run", "dev"]