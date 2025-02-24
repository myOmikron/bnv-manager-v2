FROM postgres:17-alpine AS final

COPY ./build/postgres/init-databases.sh /docker-entrypoint-initdb.d/init-databases.sh