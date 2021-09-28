FROM gradle:jdk11 as build
RUN mkdir -p /app
COPY --chown=gradle:gradle . /app
WORKDIR /app
RUN gradle bootJar
RUN cp /app/build/libs/*.jar /app/app.jar

FROM postman/newman
RUN mkdir -p /app
COPY --from=build /app/app.jar /app/app.jar
ENTRYPOINT [ "sh" ]