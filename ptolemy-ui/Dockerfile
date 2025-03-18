FROM node:18-slim AS build

WORKDIR /app

COPY ptolemy-ui/package*.json ./

RUN npm ci

COPY ptolemy-ui/ .

RUN npm run build

FROM nginx:1.27.4-bookworm

COPY --from=build /app/dist /usr/share/nginx/html

EXPOSE 80

CMD ["nginx", "-g", "daemon off;"]
