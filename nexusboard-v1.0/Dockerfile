# NexusBoard v1.0 — VPS & Self-Hosted Container (<60MB Alpine)
FROM node:22-alpine
WORKDIR /app
COPY . .
VOLUME ["/app/.nexusboard"]
EXPOSE 4300
ENV NXB_DATA_DIR=/app/.nexusboard
ENV PORT=4300
CMD ["node", "packages/cli/src/nxb.mjs", "serve", "--port", "4300"]
