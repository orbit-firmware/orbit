# lists recipes
default:
  @just --list

# builds the firmware (kb=_emulator runs the emulator instead)
compile kb features="": (prepare kb features)
  #!/usr/bin/env sh
  cd build
  if [ "{{kb}}" = "_emulator" ]; then
    printf '\033[34mEmulator detected, starting...\033[0m\n'
    cargo run --release
  else
    cargo build --release {{ if features != "" { "--features " + features } else { "" } }}
  fi

# flashes the firmware via probe-rs (features="debug" for defmt logs)
flash kb features="": (prepare kb features)
  cd build && cargo embed {{ if features != "" { "--features " + features } else { "" } }}

# cleans build files
clean:
  rm -rf build firmware.bin firmware.hex

# runs the dev container
docker:
  cd docker && docker-compose up -d && docker exec -it orbit bash

# starts the docs server
docs:
  cd docs && npm install && npm run docs:dev

# merges orbit + chip + keyboard toml into ./build
[private]
prepare kb features="":
  @cargo install --list | grep -q cargo-play || cargo install cargo-play
  cargo play ./orbit/build.rs -- {{kb}} {{features}}

# runs the host emulator, rebuilding and restarting it whenever orbit/ changes
emulate:
  #!/usr/bin/env bash
  # builds in a mirror so it never touches ./build (the macros find the root by the "orbit" dir name)
  sim=/tmp/kbemu/orbit
  bin="$sim/build/target/release/_emulator"
  export CARGO_NET_OFFLINE=true
  mkdir -p "$sim"
  sync() { rsync -a --delete --exclude target --itemize-changes orbit "$sim/"; }
  trap 'printf "\033[?1049l\033[?25h"; stty sane' EXIT
  sync >/dev/null
  while true; do
    printf '\033[?1049l\033[?25h'; stty sane; clear
    printf '\033[34mbuilding emulator (%s)...\033[0m\n' "$(date +%T)"
    if (cd "$sim" && cargo play -q ./orbit/build.rs -- _emulator >/dev/null 2>&1 && cd build && cargo build --release -q 2>/tmp/kbemu/err); then
      # the emulator keeps the terminal (it reads ctrl+r / ctrl+c); a background loop
      # restarts it when orbit/ changes
      (while [ -z "$(sync)" ]; do sleep 1; done; pkill -f "$bin") &
      watcher=$!
      "$bin"
      code=$?
      kill $watcher 2>/dev/null
      [ $code -eq 0 ] && exit 0 # ctrl+c
    else
      grep -A12 '^error' /tmp/kbemu/err
      printf '\033[31mbuild failed, waiting for changes\033[0m\n'
      while [ -z "$(sync)" ]; do sleep 1; done
    fi
  done

# runs the emulator's key tests without a screen (exit 1 on failure)
test:
  #!/usr/bin/env bash
  set -e
  sim=/tmp/kbtest/orbit
  mkdir -p "$sim"
  rsync -a --delete --exclude target orbit "$sim/"
  cd "$sim" && cargo play -q ./orbit/build.rs -- _emulator >/dev/null 2>&1
  cd build && cargo build --release -q 2>/dev/null
  ORBIT_EMULATOR_TEST=1 ./target/release/_emulator
