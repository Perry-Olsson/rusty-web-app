.PHONY: help dev up down build rebuild restart logs clean shell

help: ## Show this help message
	@echo 'Usage: make [target]'
	@echo ''
	@echo 'Available targets:'
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "  %-15s %s\n", $$1, $$2}'

dev: ## Start the development environment
	docker compose up -d

up: dev ## Alias for dev

down: ## Stop the development environment
	docker compose down

build: ## Build the Rust project inside the container
	docker compose exec app cargo build

rebuild: ## Rebuild the Docker image
	docker compose build

restart: ## Restart the service
	docker compose restart

test:
	docker compose exec app cargo nextest run 

logs: ## View logs (follow mode)
	docker compose logs -f

clean: ## Stop and remove containers, networks, and volumes
	docker compose down -v

shell: ## Open a shell in the container
	docker compose exec app bash
