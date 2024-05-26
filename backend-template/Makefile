start-docker:
	cd ./docker/dev/ && ./up.sh

start:
	sh ./run

run-tests:
	sh ./test

migrate-db:
	cargo run --bin migration -- up -u $DATABASE_URL

update-erd:
	planter postgres://username:password@localhost:5432/database_name\?sslmode=disable -o docs/schema.puml
