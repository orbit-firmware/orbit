.PHONY: all help docker docs

MAKEFLAGS += --no-print-directory

build: #/ [kb=keyboard]
	@cd dev && chmod +x ./build.sh
	@cd dev && ./build.sh $(kb)

build-debug: #/ [kb=keyboard]
	@cd dev && chmod +x ./build.sh
	@cd dev && ./build.sh $(kb) debug

flash: #/ [kb=keyboard]
	@cd dev && chmod +x ./flash.sh
	@cd dev && ./flash.sh $(kb)

clean: #/ cleans cargo and build files
	@cd rmk && cargo clean
	@cd rmk && rm -rf tmp
	@cd rmk && rm -rf user
	@rm -rf firmware.bin
	@rm -rf firmware.hex

help:
	@grep -E '^[a-zA-Z_-]+:.*?#/ .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?#/ "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'

docker: #/ runs the dev container
	@cd dev && docker-compose up -d
	@docker exec -it rmk bash

docs:
	@cd docs && npm install
	@cd docs && npm run docs:dev