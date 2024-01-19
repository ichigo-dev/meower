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
	@docker build . -f ./docker/common/rust/Dockerfile -t rust_base
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
	$(MAKE) down --no-print-directory
	$(MAKE) up --no-print-directory

# Watch the logs
logs:
	@docker compose logs -f

# Show the status of containers
ps:
	@docker compose ps


################################################################################
# Enter containers
################################################################################

auth_db:
	@docker compose exec auth_db \
		bash -c 'psql ${AUTH_DB_NAME} -U ${AUTH_DB_USER}'

dev:
	@docker compose exec dev bash


################################################################################
# Database utilities
################################################################################

# Migrate database
migrate-auth:
	@docker compose exec dev \
		bash -c 'cd /system/auth && \
			sea-orm-cli migrate ${cmd} -u ${AUTH_DB_URL}'

# Install data
install-auth:
	@docker compose exec dev \
		bash -c 'cd /system/auth/install && \
			env DATABASE_URL=${AUTH_DB_URL} cargo run --bin ${bin}'


################################################################################
# Development utilities
################################################################################

# Table definition
tbls:
	@docker compose run --rm tbls doc ${AUTH_DB_URL} ./auth --force
