FROM guergeiro/pnpm AS builder
WORKDIR /app

RUN pnpm config set registry https://registry.npmmirror.com
RUN pnpm add -g pnpm

COPY package.json pnpm-lock.yaml ./
RUN pnpm install

COPY . .
RUN pnpm install

RUN echo "Current working directory: $(pwd)"
RUN ls -la
RUN pnpm list
RUN pnpm run build


FROM nginx AS runner
COPY --from=builder /app/dist /usr/share/nginx/html
EXPOSE 80
CMD ["nginx", "-g", "daemon off;"]