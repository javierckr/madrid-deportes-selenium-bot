# Madrid Deportes Selenium Bot
Bot for automating bookings on Madrid Deportes website using Selenium.


## Requirements
- Selenium WebDriver
Needed a webdriver with chrome: https://www.selenium.dev/documentation/grid/getting_started/

Or a Selenium Grid, for kubernetes: https://artifacthub.io/packages/helm/selenium-grid/selenium-grid/0.22.0

## Usage
The bot works with Docker or in a Kubernetes cluster.

### Docker
Create a `.env` from env-example file and fill in the required variables, then
run the bot with:
```
$ docker run --env-file .env javierckr/madrid-deportes-selenium-bot
```
For automatic execution, you can use a cron job to run the bot at specific
times, e.g:
```
00 18 * * 1,2,3,6,0 docker run --env-file /path/to/.env javierckr/madrid-deportes-selenium-bot
```
This will run the bot every day at 18:00 except on Thursdays and Fridays, you
can book with 49 hours in advance, so this will book tickets for Monday,
Tuesday, Wednesday, Thursday and Fryday at 19:00.

### Kubernetes
Kubernetes manifests are provided in the `k8s` folder.
Create a `.env` from env-example file and fill in the required variables, then
modify the k8s/cronjob.yaml and install with:
```
$ kubectl apply -k k8s/
```

## Building
To build the Docker image, run:
```
$ docker build -t madrid-deportes-selenium-bot .
```
To build rust binary, run:
```
$ cargo build --release
```



