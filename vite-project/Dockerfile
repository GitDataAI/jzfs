FROM nginx AS runner
WORKDIR /app
COPY . .
RUN ls . \
    && mv /app/dist/* /usr/share/nginx/html \
    && rm -rf /app
EXPOSE 80
COPY nginx.conf /etc/nginx/nginx.conf
CMD ["nginx", "-g", "daemon off;"]