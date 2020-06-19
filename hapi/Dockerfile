FROM gcr.io/distroless/nodejs
ADD build /app
ADD node_modules /app/node_modules
ADD .docker.env /app/.env
WORKDIR /app
CMD ["Index.js"]
