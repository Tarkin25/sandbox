FROM gradle:jdk11
RUN mkdir -p /app
COPY --chown=gradle:gradle . /app
WORKDIR /app
RUN gradle bootJar
RUN cp /app/build/libs/*.jar /app/app.jar
ENTRYPOINT ["java", "-jar", "/app/app.jar"]