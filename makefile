.PHONY: help compile flash clean docker docs

MAKEFLAGS += --no-print-directory


compile: #/ [kb=keyboard] [features="list of features"]
ifeq ($(kb),)
	@make help -B
	@exit 1
endif
	@make _ensure_cargo_script -B
	@cargo script .dev/scripts/compile.rs -- $(kb) $(features)
	

flash: #/ [kb=keyboard]
ifeq ($(kb),)
	@make help -B
	@exit 1
endif
	@make _ensure_cargo_script -B
	# @cd .dev/scripts && chmod +x ./flash.sh && ./flash.sh

clean: #/ cleans build files
	@rm -rf .build
	@rm -rf firmware.bin
	@rm -rf firmware.hex

docker: #/ runs the dev container
	@cd .dev/docker && docker-compose up -d && docker exec -it rmk bash

docs: #/ starts the docs server
	@cd .dev/docs && npm install && npm run docs:dev


_ensure_cargo_script:
	@if ! cargo install --list | grep -q cargo-script; then \
		cargo install cargo-script; \
	fi

help:
	@grep -E '^[a-zA-Z_-]+:.*?#/ .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?#/ "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'
