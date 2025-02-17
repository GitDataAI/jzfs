FROM nginx AS runner
WORKDIR /app
COPY . .
RUN ls . \
    && mv /app/dist /usr/share/nginx/html \
    && rm -rf /app
EXPOSE 80
CMD ["nginx", "-g", "daemon off;"]