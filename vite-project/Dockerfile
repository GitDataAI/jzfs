FROM guergeiro/pnpm AS builder
WORKDIR /app
COPY . .
RUN pnpm config set registry https://registry.npmmirror.com
RUN pnpm install
RUN pnpm run build


FROM nginx:stable-alpine3.20-perl
COPY --from=builder /app/dist /usr/share/nginx/html
EXPOSE 80
CMD ["nginx", "-g", "daemon off;"]

