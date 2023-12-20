################################################################################
# Makefile
################################################################################

include .env


################################################################################
# Service commands
################################################################################

# Initialize the project
init:
	@echo "===== Initializing ====="
	@cp .env.example .env
	@echo "===== Finish ====="

# Setup the project
setup:
	@echo "===== Initializing ====="
	@cp .env.example .env
	$(MAKE) build
	@docker compose up -d
	$(MAKE) install bin=install_test_data
	@echo "===== Finish ====="


# Build the containers
build:
	@echo "===== Building ====="
	@docker build . -f ./docker/base/rust/Dockerfile -t rust_base
	@docker compose build
	@echo "===== Finish ====="

# Start the containers
up:
	@docker compose up -d

# Stop the containers
down:
	@docker compose down

# Watch the logs
logs:
	@docker compose logs -f


################################################################################
# Containers
################################################################################

exec:
	@docker compose exec dev bash

balancer:
	@docker compose exec balancer bash

postgres:
	@docker compose exec postgres \
		bash -c 'psql ${POSTGRES_DB} -U ${POSTGRES_USER}'


################################################################################
# Utilities
################################################################################

# Table definition
tbls:
	@docker-compose run --rm tbls doc --force --config .tbls.yml
