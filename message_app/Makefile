ctn=

create_db:
	docker exec -it message-db createdb --username=message --owner=message message

drop_db:
	docker exec -it message-db dropdb message -U message

# Run this command to install sea-orm-cli(only one time)
install_sea_orm:
	cargo install sea-orm-cli

# Run this command to initialize migration(only one time)
init_migration:
	sea-orm-cli migrate init

create_migration:
	sea-orm-cli migrate generate $(ctn)

migrate_up:
	sea-orm-cli migrate up

generate_entity:
	sea-orm-cli generate entity -o entity/src

.PHONY:  create_db drop_db install_sea_orm init_migration generate_entity create_migration migrate_up

