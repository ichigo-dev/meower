.PHONY: frontend backend migration

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
	@sudo docker-compose build --no-cache
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

auth:
	@sudo docker-compose exec auth-proxy bash

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

# Convert SCSS to CSS
sass:
	@echo "===== Converting SCSS to CSS ====="
	@sudo docker-compose exec frontend \
		grass ../common/assets/scss/style.scss ../common/assets/css/style.css
	@echo "===== Finish ====="

# Check the dependencies
udeps:
	@echo "===== Checking dependencies ====="
	@sudo docker-compose exec dev \
		cargo +nightly udeps --workspace
	@echo "===== Finish ====="

# Cryptographic review
crev:
	@echo "===== Reviewing dependencies ====="
	@sudo docker-compose exec dev \
		cargo crev verify --recursive --show-all
	@echo "===== Finish ====="

# Audit dependencies
audit:
	@echo "===== Auditing common ====="
	@sudo docker-compose exec dev \
		bash -c 'cd /common && cargo audit fix --dry-run'
	@echo "===== Finish ====="
	@echo "===== Auditing auth-proxy ====="
	@sudo docker-compose exec dev \
		bash -c 'cd /auth_proxy && cargo audit fix --dry-run'
	@echo "===== Finish ====="
	@echo "===== Auditing backend ====="
	@sudo docker-compose exec dev \
		bash -c 'cd /backend && cargo audit fix --dry-run'
	@echo "===== Finish ====="
	@echo "===== Auditing frontend ====="
	@sudo docker-compose exec dev \
		bash -c 'cd /frontend && cargo audit fix --dry-run'
	@echo "===== Finish ====="
	@echo "===== Auditing entity ====="
	@sudo docker-compose exec dev \
		bash -c 'cd /entity && cargo audit fix --dry-run'
	@echo "===== Finish ====="
	@echo "===== Auditing migration ====="
	@sudo docker-compose exec dev \
		bash -c 'cd /migration && cargo audit fix --dry-run'
	@echo "===== Finish ====="
