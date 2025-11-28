# Rust API with Redis

A simple Rust API using Actix Web with Redis integration, Docker, and Docker Compose support.

## Features

- **GET /hello?name={user}** - Greet a user and track greetings in Redis
- **GET /health** - Health check endpoint (checks Redis connectivity)
- **GET /readiness** - Readiness probe endpoint (checks if service is ready)
- Redis integration for tracking greetings
- Docker support with multi-stage builds
- Docker Compose for easy deployment

## Prerequisites

- Rust 1.70+ (for local development)
- Docker and Docker Compose

## Local Development

```bash
# Install dependencies and run
cargo run

# The API will be available at http://localhost:3000
```

Make sure Redis is running locally on port 6379, or set the `REDIS_URL` environment variable.

## Docker

Build and run with Docker:

```bash
# Build the image
docker build -t rust-api-redis .

# Run with a Redis container
docker run -d --name redis redis:7-alpine
docker run -p 3000:3000 --link redis -e REDIS_URL=redis://redis:6379 rust-api-redis
```

## Docker Compose

The easiest way to run the entire stack:

```bash
# Start all services
docker-compose up -d

# View logs
docker-compose logs -f

# Stop all services
docker-compose down
```

## API Endpoints

### Hello Endpoint
```bash
curl "http://localhost:3000/hello?name=World"
# Response: {"message":"Hello, World!"}
```

### Health Check
```bash
curl http://localhost:3000/health
# Response: {"status":"healthy"}
```

### Readiness Check
```bash
curl http://localhost:3000/readiness
# Response: {"status":"ready"}
```

## Environment Variables

- `REDIS_URL` - Redis connection URL (default: `redis://localhost:6379`)
- `RUST_LOG` - Log level (default: `info`)

## Kubernetes Ready

The `/health` and `/readiness` endpoints are designed to work with Kubernetes:

```yaml
livenessProbe:
  httpGet:
    path: /health
    port: 3000
  initialDelaySeconds: 30
  periodSeconds: 10

readinessProbe:
  httpGet:
    path: /readiness
    port: 3000
  initialDelaySeconds: 5
  periodSeconds: 5
```