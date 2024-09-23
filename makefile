.PHONY: help compile flash clean docker docs

MAKEFLAGS += --no-print-directory


compile: #/ [kb=keyboard] [features="list of features"]
ifeq ($(kb),)
	@make help -B
	@exit 1
endif
	@make pre_compile -B args="$(kb) $(features)" || { \
		echo "pre_compile failed, aborting."; \
		exit 1; \
	}
ifeq ($(kb),_emulator)
	@echo "\x1b[34mEmulator detected, starting...\x1b[0m"
	@cd build && cargo run --release
else
	@cd build && cargo objcopy --release -- -O binary ../firmware.bin
	@cd build && cargo objcopy --release -- -O ihex ../firmware.hex
endif

flash: #/ flashes the firmware [debug=true/false]
	@make pre_compile -B args="$(kb) $(features)" || { \
		echo "pre_compile failed, aborting."; \
		exit 1; \
	}
ifeq ($(debug),true)
	cd build && cargo embed --features debug
else
	cd build && cargo embed
endif

clean: #/ cleans build files
	@rm -rf build
	@rm -rf firmware.bin
	@rm -rf firmware.hex

docker: #/ runs the dev container
	@cd docker && docker-compose up -d && docker exec -it orbit bash

docs: #/ starts the docs server
	@cd docs && npm install && npm run docs:dev

help:
	@grep -E '^[a-zA-Z_-]+:.*?#/ .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?#/ "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'

pre_compile:
	@rustup default stable 2>/dev/null
	@if ! cargo install --list | grep -q cargo-play; then \
		cargo install cargo-play; \
	fi
	@cargo play ./orbit/build.rs -- $(args)
	