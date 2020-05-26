FROM golang:1.14 as build-env

WORKDIR /go/src/app
ADD . /go/src/app

RUN go build -o /go/bin/app

FROM gcr.io/distroless/base
COPY --from=build-env /go/bin/app /
COPY --from=build-env /go/src/app/.env.docker /
COPY --from=build-env /go/src/app/1_initital_migration.tx.down.sql /
COPY --from=build-env /go/src/app/1_initital_migration.tx.up.sql /
CMD ["/app"]
