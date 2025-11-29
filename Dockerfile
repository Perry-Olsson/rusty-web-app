FROM rust:1.91.1 AS builder

ARG APP_NAME
FROM builder AS dev

WORKDIR /${APP_NAME}

CMD ["scripts/run.sh"]
