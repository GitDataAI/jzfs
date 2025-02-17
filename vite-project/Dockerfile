FROM guergeiro/pnpm AS builder
WORKDIR /app
RUN mkdir /app/node_modules
COPY . .
RUN rm -rf /app/node_modules
RUN pnpm config set registry https://registry.npmmirror.com
RUN pnpm add -g pnpm

RUN pnpm install

RUN echo "Current working directory: $(pwd)"
RUN ls -la
RUN pnpm list
RUN pnpm run build


FROM nginx AS runner
COPY --from=builder /app/dist /usr/share/nginx/html
EXPOSE 80
CMD ["nginx", "-g", "daemon off;"]