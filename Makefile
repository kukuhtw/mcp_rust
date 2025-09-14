# Root Makefile untuk SMRT MCP PoC

PROJECT_NAME=smrt-mcp
DOCKER_COMPOSE=docker-compose -f docker/docker-compose.yml

.PHONY: build up down logs ps sh-backend sh-frontend sh-db clean

## Build semua image
build:
	$(DOCKER_COMPOSE) build

## Start semua service (detached mode)
up:
	$(DOCKER_COMPOSE) up -d

## Stop & remove containers
down:
	$(DOCKER_COMPOSE) down

## Lihat logs streaming
logs:
	$(DOCKER_COMPOSE) logs -f --tail=100

## List containers
ps:
	$(DOCKER_COMPOSE) ps

## Shell masuk ke container backend
sh-backend:
	docker exec -it smrt-mcp-backend /bin/bash

## Shell masuk ke container frontend
sh-frontend:
	docker exec -it smrt-mcp-frontend /bin/sh

## Shell masuk ke container database
sh-db:
	docker exec -it smrt-mcp-db mysql -usmrt -psmrtpass smrt_mcp

## Bersihkan volume & container
clean:
	$(DOCKER_COMPOSE) down -v --rmi local --remove-orphans
