################################################################################
# Makefile
################################################################################

.PHONY: env
include .env


################################################################################
# Service commands
################################################################################

# Copy .env.example to .env
env:
	@echo "===== Initializing ====="
	@echo "Copying .env.example to .env"
	@cp .env.example .env
	@echo "Generating JWT keys"
	@mkdir env/dev -p
	@openssl genrsa -out env/dev/${JWT_PRIVATE_KEY} 2048
	@openssl rsa -in env/dev/${JWT_PRIVATE_KEY} -pubout > env/dev/${JWT_PUBLIC_KEY}
	@echo "===== Finish ====="

# Setup the project
setup:
	$(MAKE) env --no-print-directory
	$(MAKE) build --no-print-directory
	$(MAKE) up --no-print-directory
	$(MAKE) migrate-auth cmd=fresh --no-print-directory
	$(MAKE) install-auth bin=install_initial_data --no-print-directory
	$(MAKE) migrate-app cmd=fresh --no-print-directory

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
		bash -c 'psql ${AUTH_DB_NAME} -U ${AUTH_DB_USER} -p ${AUTH_DB_PORT}'

app_db:
	@docker compose exec app_db \
		bash -c 'psql ${APP_DB_NAME} -U ${APP_DB_USER} -p ${APP_DB_PORT}'

account_db:
	@docker compose exec account_db \
		bash -c 'psql ${ACCOUNT_DB_NAME} -U ${ACCOUNT_DB_USER} -p ${ACCOUNT_DB_PORT}'

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

migrate-app:
	@docker compose exec dev \
		bash -c 'cd /system/app && \
			sea-orm-cli migrate ${cmd} -u ${APP_DB_URL}'

migrate-account:
	@docker compose exec dev \
		bash -c 'cd /system/apis/account && \
			sea-orm-cli migrate ${cmd} -u ${ACCOUNT_DB_URL}'

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
	@docker compose run --rm tbls doc ${APP_DB_URL} ./app --force
