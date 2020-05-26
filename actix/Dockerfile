FROM debian:10.3-slim
WORKDIR /app
ADD target/release/footballmanager /app/footballmanager
RUN apt-get update && apt-get install -y libpq-dev

CMD ["./footballmanager"]