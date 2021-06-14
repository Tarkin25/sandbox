FROM maven:3-adoptopenjdk-8 as build
RUN mkdir -p /app
COPY ./pom.xml /app
COPY ./src /app
WORKDIR /app
RUN mvn package

FROM postman/newman:ubuntu
RUN apt update
RUN apt install openjdk-8-jre -y
RUN mkdir -p /app
COPY system-tests/*.postman_collection.json /app/postman_collection.json
COPY entrypoint.sh /app/entrypoint.sh
COPY target/*.jar /app/app.jar
ENV SPRING_PROFILES_ACTIVE=system-test
WORKDIR /app
ENTRYPOINT ["/app/entrypoint.sh"]

