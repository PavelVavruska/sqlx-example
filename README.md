# Rust SQLx on Docker in Manjaro Linux

A sample project using PostreSQL DB in Docker container from SQLx Rust Toolkit.

## API

```
/
```
   - GET: list all tasks as JSON

```
/add
```
   - POST: add new task into DB using form (`application/x-www-form-urlencoded`)

## How To Build It ?

Download Rust compiler from [here](https://www.rust-lang.org/en-US/), change the working directory to the root of this project, then execute the following command:

```
> cargo build
```

The executable binary will appear in `target/debug`, called `sqlx`.

## How to install Docker on Manjaro Linux

- Update system: 
```
sudo pacman -Syu
```

- Install docker: 
```
sudo pacman -S docker
```

- Following commands starts Docker service and enable it to run after system start. If you get Error, try restarting your system.
```
sudo pacman -S docker
```

- Verify Docker version: 
```
sudo docker version
```

## How to install and run PostgreSQL on Docker

https://docs.docker.com/samples/postgresql_service/

```
sudo docker pull postgres 
```

```
sudo docker run --name some-postgres -e POSTGRES_PASSWORD=mysecretpassword -d postgres
```