## News Feed

Just a project to play with Tokio (rust), rabbitmq and cassandra in anger. Just 
to make it clear, I wasn't angry when I wrote this code.

At the moment it only takes covid news. 

## Arch

![Arch](https://user-images.githubusercontent.com/2847315/125882627-7a4b0efc-edcb-4ed3-ac12-28399f53c535.jpg)


## How to use it locally

You'll need an api key from https://newsapi.org/ and set the `API_KEY` env var with it

### Compile it

`> cargo build`

### Run infraestructure

`> docker-compose up`

### Run producer

`> cargo run --bin producer`

### Run Consumers

In diferent Terminals or detached processes run as many as you like of the following

`> cargo run --bin consumer ar` (where ar is the iso2 country code, e.g au, en, us)
