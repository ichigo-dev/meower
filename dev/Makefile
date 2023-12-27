################################################################################
# Makefile
################################################################################

include .env


################################################################################
# Service commands
################################################################################

# Copy .env.example to .env
env:
	@echo "===== Initializing ====="
	@echo "Copying .env.example to .env"
	@cp .env.example .env
	@echo "===== Finish ====="

# Setup the project
setup:
	$(MAKE) env --no-print-directory
	$(MAKE) build --no-print-directory
	$(MAKE) up --no-print-directory

# Build the containers
build:
	@echo "===== Building ====="
	@docker build . -f ./docker/base/rust/Dockerfile -t rust_base
	@docker compose build
	@echo "===== Finish ====="

# Start the containers
up:
	@docker compose up -d --remove-orphans

# Stop the containers
down:
	@docker compose down

# Restart the containers
restart:
	@docker compose restart

# Watch the logs
logs:
	@docker compose logs -f

# Show the status of containers
ps:
	@docker compose ps


################################################################################
# Enter containers
################################################################################

balancer:
	@docker compose exec balancer bash

postgres:
	@docker compose exec postgres \
		bash -c 'psql ${POSTGRES_DB} -U ${POSTGRES_USER}'

dev:
	@docker compose exec dev bash


################################################################################
# Database utilities
################################################################################

# Migrate database
migrate:
	@docker compose exec dev \
		bash -c 'cd /system/database && sea-orm-cli migrate ${cmd}'

# Install data
install:
	@docker compose exec dev \
		bash -c 'cd /system/database/install && cargo run --bin ${bin}'


################################################################################
# Development utilities
################################################################################

# Table definition
tbls:
	@docker compose run --rm tbls doc --force --config .tbls.yml
