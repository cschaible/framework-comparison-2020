# Comparison of different languages and frameworks

This repository shows a sample application implemented in different programming languages (go, java, kotlin, rust) using different frameworks (actix, fasthttp, micronaut, quarkus, rocket, spring-mvc, spring-r2dbc, vert.x, warp).

## Application

The initial version was written by Novatec colleague [csh0711](https://github.com/csh0711/micronaut-data-graalvm-kotlin) in kotlin using micronaut (the micronaut implementation in this repo is a modified version of it).

The use case is a simple CRUD application where a "Footballer" entity can be managed.  
All applications have the same API with the following endpoints:  

| Method | Endpoint | Request Body | Response |
| --- | --- | --- | --- |
| GET | /footballers[?position=\<position\>] | | Footballer[] |  
| GET | /footballers/{id} | | Footballer |  
| POST | /footballers |Footballer | Footballer |  
| DELETE | /footballers/{id} | |

A footballer object looks like:  
```json
{
  "firstName": "Max",
  "lastName": "Smith",
  "position": "Goal Keeper"
}
```

## Required software to build and run the applications

The following software is required to build and run all applications:  
- [Docker](https://www.docker.com/)  
- [Docker-Compose](https://docs.docker.com/compose/)  
- Java 11 (can be setup by using [sdkman](https://sdkman.io/))  
- GraalVM (can be setup by  using [sdkman](https://sdkman.io/))  
- libpq-devel (redhat based systems) or libpq-dev (debian based systems) for the actix example  
- openssl-devel (redhat based systems) or libssl-dev (debian based systems) for the actix-sqlx example  
- [rust/cargo](https://www.rust-lang.org/learn/get-started) for the actix example  
- [go](https://golang.org/) (or download it in GoLand IDE) for the fasthttp example

## Build and run

All applications use a PostgreSQL database. In the `docker` directory is a Docker-Compose script to set it up (see PostgreSQL / pgAdmin description below).

### PostgreSQL / pgAdmin
- Run the `docker/up.sh` script to start a PostgreSQL Database and pgAdmin with a Docker-Compose script.  
- Run the `docker/reset-db.sh` script to reset the database before switching to another technology or before re-running a load test  
- Run the `docker/down.sh`script to stop and clean-up the DB docker containers.

### Spring MVC [Java]
- Run the `spring-mvc/build.sh` script to build the application and package it into a docker container.  
- Run the `spring-mvc/up.sh` script to start the docker container.  
- Run the `spring-mvc/down.sh` script to stop the docker container.

### Spring R2DBC [Java]
- Run the `spring-r2dbc/build.sh` script to build the application and package it into a docker container.  
- Run the `spring-r2dbc/up.sh` script to start the docker container.  
- Run the `spring-r2dbc/down.sh` script to stop the docker container.

### Micronaut [Kotlin] (JVM)
- Run the `micronaut/build-jvm.sh` script to build the application and package it into a docker container.  
- Run the `micronaut/up-jvm.sh` script to start the docker container.  
- Run the `micronaut/down.sh` script to stop the docker container.

### Micronaut [Kotlin] (Native)
- Run the `micronaut/install-native-image.sh` to install GraalVM native-imaage tool if not already installed.  
- Run the `micronaut/build-native.sh` script to build the application and package it into a docker container.  
- Run the `micronaut/up-native.sh` script to start the docker container.  
- Run the `micronaut/down.sh` script to stop the docker container.

### Quarkus [Java] (JVM)
- Run the `quarkus/build-jvm.sh` script to build the application and package it into a docker container.  
- Run the `quarkus/up-jvm.sh` script to start the docker container.  
- Run the `quarkus/down.sh` script to stop the docker container.

### Quarkus [Java] (Native)
- Run the `quarkus/install-native-image.sh` to install GraalVM native-imaage tool if not already installed.  
- Run the `quarkus/build-native.sh` script to build the application and package it into a docker container.  
- Run the `quarkus/up-native.sh` script to start the docker container.  
- Run the `quarkus/down.sh` script to stop the docker container.

### Quarkus Vert.x [Java] (JVM)
- Run the `quarkus-vertx/build-jvm.sh` script to build the application and package it into a docker container.  
- Run the `quarkus-vertx/up-jvm.sh` script to start the docker container.  
- Run the `quarkus-vertx/down.sh` script to stop the docker container.

### Quarkus Vert.x [Java] (Native)
- Run the `quarkus-vertx/install-native-image.sh` to install GraalVM native-imaage tool if not already installed.  
- Run the `quarkus-vertx/build-native.sh` script to build the application and package it into a docker container.  
- Run the `quarkus-vertx/up-native.sh` script to start the docker container.  
- Run the `quarkus-vertx/down.sh` script to stop the docker container.

### Fasthttp [Go]
- Run the `fasthttp/build.sh` script to build the application and package it into a docker container.  
- Run the `fasthttp/up.sh` script to start the docker container.  
- Run the `fasthttp/down.sh` script to stop the docker container.

### Rocket [Rust]
- Run the `rocket/setup_diesel_cli.sh` script to install the diesel (db-migration) CLI.
- Run the `rocket/build.sh` script to build the application and package it into a docker container.  
- Run the `rocket/up.sh` script to start the docker container.  
- Run the `rocket/down.sh` script to stop the docker container.

### Actix [Rust]
- Run the `actix/setup_diesel_cli.sh` script to install the diesel (db-migration) CLI.
- Run the `actix/build.sh` script to build the application and package it into a docker container.  
- Run the `actix/up.sh` script to start the docker container.  
- Run the `actix/down.sh` script to stop the docker container.

### Actix + SQLx [Rust]
- Run the `actix-sqlx/build.sh` script to build the application and package it into a docker container.  
- Run the `actix-sqlx/up.sh` script to start the docker container.  
- Run the `actix-sqlx/down.sh` script to stop the docker container.

### Warp + SQLx [Rust]
- Run the `warp/build.sh` script to build the application and package it into a docker container.
- Run the `warp/up.sh` script to start the docker container.
- Run the `warp/down.sh` script to stop the docker container.

## Loadtest

There are two implementations of load tests. One is implemented with Locust (Python) and one with JMeter (Java/JMX).
The implementation with Locust is too inefficient to create enough load for the fastest frameworks on my Lenovo T480 (Intel(R) Core(TM) i7-8650U with 4 cores). Therefore another implementation with JMeter is available.

### Locust

Locust is a load testing tool written in Python where the tests are also written in Python.

#### Install
To run the locust load test the following software is addtionally required:  
- python 3.x  
- pip  

#### Run

- Run the `loadtest_locust/install.sh` script to install Locust.  
- Run the `loadtest_locust/up.sh` script to start a Locust cluster.  

Open a browser and navigate to http://localhost:8089/ to start the test.  
Press Ctrl+C to stop Locust.

### JMeter

JMeter is a load testing tool written in Java. 

#### Install
To run the JMeter loadtest [JMeter](https://jmeter.apache.org/) is required.

#### Run

- Run the `loadtest_jmeter/run.sh` script to run the loadtest. Optionally a duration (in seconds) can be appended.
