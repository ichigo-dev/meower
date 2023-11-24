.PHONY: frontend backend migration

################################################################################
# Service commands
################################################################################

# Initialize the project
init:
	cp .env.example .env

# Build the containers
build:
	sudo docker-compose build --no-cache

# Start the containers
up:
	sudo docker-compose up -d

# Stop the containers
down:
	sudo docker-compose down

# Watch the logs
logs:
	sudo docker-compose logs -f


################################################################################
# Containers
################################################################################

auth:
	sudo docker-compose exec auth-proxy bash

frontend:
	sudo docker-compose exec frontend bash

backend:
	sudo docker-compose exec backend bash

db:
	sudo docker-compose exec postgres bash

migration:
	sudo docker-compose exec migration bash


################################################################################
# Utilities
################################################################################

# Convert SCSS to CSS
sass:
	sudo docker-compose exec frontend \
		grass ../common/assets/scss/style.scss ../common/assets/css/style.css
