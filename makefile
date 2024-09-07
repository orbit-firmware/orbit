.PHONY: all help link compile flash copy_firmware push
MAKEFLAGS += --no-print-directory

help:
	@grep -E '^[a-zA-Z_-]+:.*?#/ .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?#/ "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'

all: help

build: #/ [kb=keyboard]
	@if [ "$(kb)" = "" ]; then \
		echo "No keyboard specified. Use make build kb=<keyboard_name>"; \
		exit 1; \
	elif [ -d "keyboards/$(kb)" ]; then \
		chmod +x ./dev/firmware_build.sh; \
		./dev/firmware_build.sh $(kb); \
	else \
		echo "Keyboard $(kb) not found"; \
		exit 1; \
	fi

docker: #/ runs the dev container
	@cd dev && docker-compose up -d
	@docker exec -it rmk bash