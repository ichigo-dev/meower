.PHONY: frontend backend migration install

################################################################################
# Service commands
################################################################################

# Initialize the project
init:
	@echo "===== Initializing ====="
	@cp .env.example .env
	@echo "===== Finish ====="

setup:
	@echo "===== Initializing ====="
	@cp .env.example .env
	@docker-compose build
	@docker-compose up -d
	$(MAKE) install bin=install_test_data
	@echo "===== Finish ====="


# Build the containers
build:
	@echo "===== Building ====="
	@docker build . -f ./docker/base/Dockerfile -t rust_base
	@docker-compose build
	@echo "===== Finish ====="

# Start the containers
up:
	@docker-compose up -d

# Stop the containers
down:
	@docker-compose down

# Watch the logs
logs:
	@docker-compose logs -f


################################################################################
# Containers
################################################################################

dev:
	@docker-compose exec dev bash

balancer:
	@docker-compose exec balancer bash

entrance:
	@docker-compose exec entrance bash

frontend:
	@docker-compose exec frontend bash

backend:
	@docker-compose exec backend bash

db:
	@docker-compose exec postgres bash -c 'psql meower meower'


################################################################################
# Utilities
################################################################################

# Migrate database
migrate:
	@docker-compose exec dev \
		bash -c 'cd /system/datastore && sea-orm-cli migrate ${cmd}'

# Install data
install:
	@docker-compose exec dev \
		bash -c 'cd /system/datastore/install && cargo run --bin ${bin}'

# Table definition
tbls:
	@sudo rm ../spec/database_design/*
	@docker-compose run --rm tbls doc --force --config .tbls.yml

# Convert SCSS to CSS
sass:
	@echo "===== Converting SCSS to CSS ====="
	@docker-compose exec dev \
		grass /system/assets/scss/style.scss /system/assets/css/style.css
	@echo "===== Finish ====="

# Check the dependencies
udeps:
	@echo "===== Checking core dependencies ====="
	@docker-compose exec dev \
		bash -c 'cd /system/core && cargo +nightly udeps'
	@echo "===== Finish ====="
	@echo "===== Checking entrance dependencies ====="
	@docker-compose exec dev \
		bash -c 'cd /system/entrance && cargo +nightly udeps'
	@echo "===== Finish ====="
	@echo "===== Checking datastore dependencies ====="
	@docker-compose exec dev \
		bash -c 'cd /system/datastore && cargo +nightly udeps --workspace'
	@echo "===== Finish ====="
	@echo "===== Checking service dependencies ====="
	@docker-compose exec dev \
		bash -c 'cd /system/service && cargo +nightly udeps --workspace'
	@echo "===== Finish ====="

# Cryptographic review
crev:
	@echo "===== Reviewing core dependencies ====="
	@docker-compose exec dev \
		bash -c 'cd /system/core && cargo crev verify --show-all'
	@echo "===== Finish ====="
	@echo "===== Reviewing entrance dependencies ====="
	@docker-compose exec dev \
		bash -c 'cd /system/entrance && cargo crev verify --recursive --show-all'
	@echo "===== Finish ====="
	@echo "===== Reviewing datastore dependencies ====="
	@docker-compose exec dev \
		bash -c 'cd /system/datastore && cargo crev verify --recursive --show-all'
	@echo "===== Finish ====="
	@echo "===== Reviewing service dependencies ====="
	@docker-compose exec dev \
		bash -c 'cd /system/service && cargo crev verify --recursive --show-all'
	@echo "===== Finish ====="

# Audit dependencies
audit:
	@echo "===== Auditing core ====="
	@docker-compose exec dev \
		bash -c 'cd /system/core && cargo audit fix --dry-run'
	@echo "===== Finish ====="
	@echo "===== Auditing entrance ====="
	@docker-compose exec dev \
		bash -c 'cd /system/entrance && cargo audit fix --dry-run'
	@echo "===== Finish ====="
	@echo "===== Auditing backend ====="
	@docker-compose exec dev \
		bash -c 'cd /system/service/backend && cargo audit fix --dry-run'
	@echo "===== Finish ====="
	@echo "===== Auditing frontend ====="
	@docker-compose exec dev \
		bash -c 'cd /system/service/frontend && cargo audit fix --dry-run'
	@echo "===== Finish ====="
	@echo "===== Auditing entity ====="
	@docker-compose exec dev \
		bash -c 'cd /system/datastore/entity && cargo audit fix --dry-run'
	@echo "===== Finish ====="
	@echo "===== Auditing migration ====="
	@docker-compose exec dev \
		bash -c 'cd /system/datastore/migration && cargo audit fix --dry-run'
	@echo "===== Finish ====="
