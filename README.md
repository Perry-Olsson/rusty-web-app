# Rusty Web App

A simple web service built with Rust.

## Development Setup

This project uses Docker Compose for cross-platform development.

### Prerequisites

- Docker
- Docker Compose
- Make (optional, for convenience commands)

### Quick Start

```bash
make dev    # Start the development environment
make logs   # View logs
make build  # Build the Rust project (auto restarts the server)
make down   # Stop the environment
```

Run `make help` to see all available commands.

### Make Commands

| Command | Description |
|---------|-------------|
| `make dev` or `make up` | Start the development environment |
| `make down` | Stop the development environment |
| `make build` | Build the Rust project inside the container |
| `make rebuild` | Rebuild the Docker image |
| `make restart` | Restart the service |
| `make logs` | View logs (follow mode) |
| `make clean` | Stop and remove containers, networks, and volumes |
| `make shell` | Open a bash shell in the container |
| `make help` | Show all available commands |

### Docker Compose Commands (if not using Make)

**Start the development environment:**
```bash
docker compose up -d
```

**Stop the development environment:**
```bash
docker compose down
```

**View logs:**
```bash
docker compose logs -f
```

**Build the Rust project inside the container:**
```bash
docker compose exec app cargo build --manifest-path /app/Cargo.toml
```

**Rebuild the Docker image:**
```bash
docker compose build
```

**Restart the service:**
```bash
docker compose restart
```

### Migration Notes

The old shell scripts in `scripts/` have been replaced with Docker Compose:
- `scripts/dev.sh` → `docker compose up -d`
- `scripts/dev.sh down` → `docker compose down`
- `scripts/build.sh` → `docker compose exec app cargo build --manifest-path /app/Cargo.toml`

The scripts can be removed once you've verified the Docker Compose setup works for your workflow.
