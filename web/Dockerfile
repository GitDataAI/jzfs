FROM idotdigital/node-pnpm as build
WORKDIR /app
COPY . .
RUN pnpm install
RUN pnpm build:all


FROM nginx:alpine
COPY --from=build /app/dist /usr/share/nginx/html
EXPOSE 80
CMD ["nginx", "-g", "daemon off;"]