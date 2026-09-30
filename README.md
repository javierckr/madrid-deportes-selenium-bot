# Madrid Deportes Selenium Bot

Bot for automating bookings on the Madrid Deportes website using [Selenium](https://www.selenium.dev/).

The bot can run as a Docker container or as a Kubernetes CronJob, making it suitable for both manual and scheduled bookings.

## Requirements

- Docker, or a Kubernetes cluster
- A compatible Selenium WebDriver, or a Selenium Grid

For a standalone Selenium setup, see the [Selenium WebDriver documentation](https://www.selenium.dev/documentation/grid/getting_started/).

For Kubernetes, you can deploy [Selenium Grid](https://artifacthub.io/packages/helm/selenium-grid/selenium-grid/0.22.0) using the provided Helm chart.

## Configuration

Copy the example environment file and fill in the required variables:

```bash
cp env-example .env
```

Then edit `.env` with your Madrid Deportes credentials and booking configuration.

> **Security:** Do not commit `.env` to the repository. Add it to `.gitignore` if it is not already there.

## Usage

The bot supports both Docker and Kubernetes deployments.

### Docker

Build or pull the Docker image and run it with your environment file:

```bash
docker run --env-file .env javierckr/madrid-deportes-selenium-bot
```

#### Scheduled execution

You can use `cron` to run the bot automatically at specific times.

For example:

```cron
00 18 * * 1,2,3,6,0 docker run --rm --env-file /path/to/.env javierckr/madrid-deportes-selenium-bot
```

This runs the bot every day at **18:00**, except on **Thursday and Friday**.

Madrid Deportes allows bookings up to **49 hours in advance**, so running the bot on these days allows it to book sessions for the corresponding upcoming days.

Adjust the schedule according to the booking times and days you need.

### Kubernetes

Kubernetes manifests are provided in the [`k8s/`](k8s/) directory.

First, create and configure your environment file:

```bash
cp env-example .env
```

Then review and customize:

```text
k8s/cronjob.yaml
```

Deploy the manifests with:

```bash
kubectl apply -k k8s/
```

The Kubernetes configuration is intended to run the bot automatically as a CronJob.

## Building

### Docker

Build the Docker image locally:

```bash
docker build -t madrid-deportes-selenium-bot .
```

### Rust

The bot includes a Rust binary that can be built in release mode with:

```bash
cargo build --release
```

The resulting binary will be available under:

```text
target/release/
```

## Project Structure

```text
.
├── k8s/                 # Kubernetes manifests
├── src/                 # Rust source code
├── env-example          # Example environment configuration
├── Dockerfile           # Docker image definition
└── Cargo.toml           # Rust project configuration
```

## License

See the repository for license information.
