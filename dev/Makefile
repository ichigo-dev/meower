.PHONY: frontend backend migration install

################################################################################
# Service commands
################################################################################

# Initialize the project
init:
	@echo "===== Initializing ====="
	@cp .env.example .env
	@echo "===== Finish ====="

# Build the containers
build:
	@echo "===== Building ====="
	@sudo docker build . -f ./docker/base/Dockerfile -t rust_base
	@sudo docker-compose build
	@echo "===== Finish ====="

# Start the containers
up:
	@sudo docker-compose up -d

# Stop the containers
down:
	@sudo docker-compose down

# Watch the logs
logs:
	@sudo docker-compose logs -f


################################################################################
# Containers
################################################################################

dev:
	@sudo docker-compose exec dev bash

auth:
	@sudo docker-compose exec auth-server bash

frontend:
	@sudo docker-compose exec frontend bash

backend:
	@sudo docker-compose exec backend bash

db:
	@sudo docker-compose exec postgres bash

migration:
	@sudo docker-compose exec migration bash


################################################################################
# Utilities
################################################################################

# Install data
install:
	@sudo docker-compose exec install \
		bash -c 'cd /install && cargo run --bin ${bin}'

# Convert SCSS to CSS
sass:
	@echo "===== Converting SCSS to CSS ====="
	@sudo docker-compose exec dev \
		grass /system/assets/scss/style.scss /system/assets/css/style.css
	@echo "===== Finish ====="

# Check the dependencies
udeps:
	@echo "===== Checking core dependencies ====="
	@sudo docker-compose exec dev \
		bash -c 'cd /system/core && cargo +nightly udeps'
	@echo "===== Finish ====="
	@echo "===== Checking auth_server dependencies ====="
	@sudo docker-compose exec dev \
		bash -c 'cd /system/auth_server && cargo +nightly udeps'
	@echo "===== Finish ====="
	@echo "===== Checking datastore dependencies ====="
	@sudo docker-compose exec dev \
		bash -c 'cd /system/datastore && cargo +nightly udeps --workspace'
	@echo "===== Finish ====="
	@echo "===== Checking service dependencies ====="
	@sudo docker-compose exec dev \
		bash -c 'cd /system/service && cargo +nightly udeps --workspace'
	@echo "===== Finish ====="

# Cryptographic review
crev:
	@echo "===== Reviewing core dependencies ====="
	@sudo docker-compose exec dev \
		bash -c 'cd /system/core && cargo crev verify --show-all'
	@echo "===== Finish ====="
	@echo "===== Reviewing auth_server dependencies ====="
	@sudo docker-compose exec dev \
		bash -c 'cd /system/auth_server && cargo crev verify --recursive --show-all'
	@echo "===== Finish ====="
	@echo "===== Reviewing datastore dependencies ====="
	@sudo docker-compose exec dev \
		bash -c 'cd /system/datastore && cargo crev verify --recursive --show-all'
	@echo "===== Finish ====="
	@echo "===== Reviewing service dependencies ====="
	@sudo docker-compose exec dev \
		bash -c 'cd /system/service && cargo crev verify --recursive --show-all'
	@echo "===== Finish ====="

# Audit dependencies
audit:
	@echo "===== Auditing core ====="
	@sudo docker-compose exec dev \
		bash -c 'cd /system/core && cargo audit fix --dry-run'
	@echo "===== Finish ====="
	@echo "===== Auditing auth-server ====="
	@sudo docker-compose exec dev \
		bash -c 'cd /system/auth_server && cargo audit fix --dry-run'
	@echo "===== Finish ====="
	@echo "===== Auditing backend ====="
	@sudo docker-compose exec dev \
		bash -c 'cd /system/service/backend && cargo audit fix --dry-run'
	@echo "===== Finish ====="
	@echo "===== Auditing frontend ====="
	@sudo docker-compose exec dev \
		bash -c 'cd /system/service/frontend && cargo audit fix --dry-run'
	@echo "===== Finish ====="
	@echo "===== Auditing entity ====="
	@sudo docker-compose exec dev \
		bash -c 'cd /system/datastore/entity && cargo audit fix --dry-run'
	@echo "===== Finish ====="
	@echo "===== Auditing migration ====="
	@sudo docker-compose exec dev \
		bash -c 'cd /system/datastore/migration && cargo audit fix --dry-run'
	@echo "===== Finish ====="
