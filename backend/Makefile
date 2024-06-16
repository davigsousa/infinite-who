dev:
	@echo "starting cargo-watch..."
	cargo watch -x run

start:
	cargo run

db-up:
	docker-compose up -d

db-create-migration:
	diesel migration generate $(name)

db-migrate:
	diesel migration run

db-rollback:
	diesel migration redo
