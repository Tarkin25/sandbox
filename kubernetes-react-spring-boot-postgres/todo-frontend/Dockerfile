FROM node:alpine as build
RUN mkdir /app
WORKDIR /app
COPY . /app
RUN yarn install --no-progress
RUN yarn build

FROM nginx:stable-alpine
COPY --from=build /app/build /usr/share/nginx/html
CMD ["nginx", "-g", "daemon off;"]