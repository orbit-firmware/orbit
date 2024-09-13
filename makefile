.PHONY: help compile flash clean docker docs

MAKEFLAGS += --no-print-directory

PLAYDIR := .dev/play

compile: #/ [kb=keyboard] [features="list of features"]
ifeq ($(kb),)
	@make help -B
	@exit 1
endif
	@make play target=compile args="$(kb) $(features)"
	

flash: #/ flashes the firmware
ifeq ($(kb),)
	@make help -B
	@exit 1
endif
	@make _ensure_cargo_play -B
	@make play target=flash

clean: #/ cleans build files
	@rm -rf .build
	@rm -rf firmware.bin
	@rm -rf firmware.hex

docker: #/ runs the dev container
	@cd .dev/docker && docker-compose up -d && docker exec -it rmk bash

docs: #/ starts the docs server
	@cd .dev/docs && npm install && npm run docs:dev

_ensure_cargo_play:
	@if ! cargo install --list | grep -q cargo-play; then \
		cargo install cargo-play; \
	fi

help:
	@grep -E '^[a-zA-Z_-]+:.*?#/ .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?#/ "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'

play:
	@make _ensure_cargo_play -B
	@cd $(PLAYDIR) && cargo play $(shell cd $(PLAYDIR) && find $(target) -name '*.rs' | sort) -- $(args)
	