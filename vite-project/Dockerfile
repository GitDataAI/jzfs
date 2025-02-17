FROM nginx AS runner
WORKDIR /app
COPY . .
RUN ls . \
    && mv /app/dist /usr/share/nginx/html
EXPOSE 80
CMD ["nginx", "-g", "daemon off;"]