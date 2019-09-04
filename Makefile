CARGO_BIN ?= cargo

DOCKERFILE := docker/Dockerfile
DOCKERTAG := rlua_examples
DOCKERCMD :=
TARGET := x86_64-unknown-linux-gnu

SERVER_CMD := http-server
WATCH_CMD := "make test"
README := README.md

all: build test

.PHONY: build
build:
	@$(CARGO_BIN) build --target=$(TARGET)

.PHONY: check
check:
	@$(CARGO_BIN) clippy

.PHONY: test
test:
	@$(CARGO_BIN) test --target=$(TARGET)

.PHONY: clean
clean:
	@$(CARGO_BIN) clean

.PHONY: watch
watch:
	@$(CARGO_BIN) watch -i $(README) -s $(WATCH_CMD)

.PHONY: bench
bench:
	@$(CARGO_BIN) bench --target=$(TARGET)

.PHONY: bench-report
bench-serve:
	$(SERVER_CMD) target/criterion

.PHONY: doc
doc: readme
	$(CARGO_BIN) doc

.PHONY: doc-view
doc-serve:
	$(SERVER_CMD) target/doc

.PHONY: readme
readme:
	$(CARGO_BIN) readme > $(README)

doc-watch: WATCH_CMD := "make doc"
doc-watch: watch

.PHONY: musl
musl: TARGET := x86_64-unknown-linux-musl
musl: build

.PHONY: musl-test
musl-test: TARGET := x86_64-unknown-linux-musl
musl-test: test

.PHONY: musl-bench
musl-bench: TARGET := x86_64-unknown-linux-musl
musl-bench: bench

.PHONY: docker
docker:
	docker build -f $(DOCKERFILE) -t $(DOCKERTAG) .

.PHONY: docker-run
docker-run:
	docker run -it --rm --device=/dev/isgx $(DOCKERTAG) $(DOCKERCMD)

docker-bench: DOCKERCMD := cargo bench --target=x86_64-unknown-linux-musl
docker-bench: docker-run

scone: DOCKERFILE := $(DOCKERFILE).sgx
scone: DOCKERTAG := $(DOCKERTAG)_scone
scone: docker

scone-run: DOCKERTAG := $(DOCKERTAG)_scone
scone-run: docker-run

scone-bench: DOCKERTAG := $(DOCKERTAG)_scone
scone-bench: DOCKERCMD := scone-cargo bench --target=x86_64-scone-linux-musl
scone-bench: docker-run
