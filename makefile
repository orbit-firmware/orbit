.PHONY: all help docker docs

MAKEFLAGS += --no-print-directory

compile: #/ [kb=keyboard]
	@cd rmk && RMK_KEYBOARD=$(kb) cargo build --release
	@cd rmk && RMK_KEYBOARD=$(kb) cargo objcopy --release -- -O binary firmware.bin
	@cd rmk && RMK_KEYBOARD=$(kb) cargo objcopy --release -- -O binary firmware.hex

help:
	@grep -E '^[a-zA-Z_-]+:.*?#/ .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?#/ "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'

docker: #/ runs the dev container
	@cd dev && docker-compose up -d
	@docker exec -it rmk bash

docs:
	@cd docs && npm install
	@cd docs && npm run docs:dev