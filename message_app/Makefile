install_sqlx:
		cargo install sqlx-cli --features postgres

create_migration:
		sqlx migrate add ${NAME}

run_migration:
		sqlx migrate run

revert_migration:
		sqlx migrate revert

.PHONY: install_sqlx create_migration run_migration revert_migration