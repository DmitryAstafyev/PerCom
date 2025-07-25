# --- Build stage ---
FROM eclipse-temurin:21-jdk as build

RUN apt-get update && apt-get install -y curl

WORKDIR /app

COPY gradlew /app/
COPY gradle /app/gradle
COPY app/build.gradle.kts /app/
COPY settings.gradle.kts /app/
COPY app/src /app/src

RUN chmod +x gradlew
RUN ./gradlew bootJar --no-daemon

# --- Runtime stage ---
FROM eclipse-temurin:21-jre

WORKDIR /app
COPY --from=build /app/build/libs/*.jar app.jar

EXPOSE 8080

CMD ["java", "-jar", "app.jar"]
