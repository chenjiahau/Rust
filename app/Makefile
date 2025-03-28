create_db:
	docker exec -it accounting createdb --username=accounting --owner=accounting accounting

drop_db:
	docker exec -it accounting dropdb accounting -U accounting

create_migration:
	sea-orm-cli migrate generate ${file_name}

migrate_up:
	sea-orm-cli migrate up

generate_entity:
	sea-orm-cli generate entity -o entity/src

.PHONY: create_db drop_db create_migration migrate_up generate_entity

