ifdef TRAVIS_RUST_VERSION
RUST_TOOLCHAIN := ${TRAVIS_RUST_VERSION}
endif

ifndef RUST_TOOLCHAIN
RUST_TOOLCHAIN := $(shell cat ${CURDIR}/rust-toolchain 2> /dev/null | head -n 1)
ifeq ("${RUST_TOOLCHAIN}","")
RUST_TOOLCHAIN := stable
endif
endif
export RUST_TOOLCHAIN

ifndef CODECOV_DIR
CODECOV_DIR := ${CURDIR}
endif

# Usage: $(call call_all_features,makefile_target)
define call_all_features
set -eu && \
    ( \
        for cargo_args in "" --no-default-features; do CARGO_ARGS="${CARGO_ARGS} $${cargo_args}" ${MAKE} $(1); done; \
        for feature in $$(bash ${CURDIR}/scripts/cargo-features.sh); do CARGO_ARGS="${CARGO_ARGS} --features '$${feature}'" ${MAKE} $(1); done; \
        CARGO_ARGS="${CARGO_ARGS} --features '$$(bash ${CURDIR}/scripts/cargo-features.sh)'" ${MAKE} $(1); \
    )
endef

.PHONY: development
development: install-hooks
	git submodule update --init --recursive

.PHONY: install-hooks
install-hooks: .git/hooks/pre-commit
	@true

.git/hooks/pre-commit: venv .pre-commit-config.yaml
	./venv/bin/pre-commit install -f --install-hooks

# Python Virtual Environment needed to allow usage of pre-commit.com
venv:
	virtualenv venv
	venv/bin/pip install --no-cache pre-commit

.PHONY: clean
clean: clean-coverage clean-docker-volumes
	rm -rf target
	rm -rf venv
	rm -rf .coverage

.PHONY: bump-submodules
bump-submodules:
	bash scripts/bump-all-submodules.sh

.PHONY: clippy
clippy:
	touch src/lib.rs   # touch a file of the rust project to "force" cargo to recompile it so clippy will actually run
	cargo +${RUST_TOOLCHAIN} clippy --all-targets ${CARGO_ARGS} -- -D clippy::pedantic -D clippy::nursery

.PHONY: clippy-all-flavours
clippy-all-flavours:
	$(call call_all_features,clippy)

.PHONY: pre-commit
pre-commit: venv
	./venv/bin/pre-commit run --all-files

.PHONY: lint
lint: pre-commit clippy
	$(call call_all_features,clippy)

.PHONY: build
build:
	cargo +${RUST_TOOLCHAIN} build --all-targets ${CARGO_ARGS}

.PHONY: build-all-flavours
build-all-flavours:
	$(call call_all_features,build)

.PHONY: test
test:
	cargo +${RUST_TOOLCHAIN} test --all-targets ${CARGO_ARGS}

.PHONY: test-all-flavours
test-all-flavours:
	$(call call_all_features,test)

.PHONY: doc
doc:
	cargo +${RUST_TOOLCHAIN} doc --no-deps ${CARGO_ARGS}

${CODECOV_DIR}/codecov.bash:
	mkdir -p ${CODECOV_DIR}
	curl -s https://codecov.io/bash > ${CODECOV_DIR}/codecov.bash

.coverage:
	mkdir -p ${CURDIR}/.coverage

.PHONY: coverage
coverage: clean-coverage .coverage ${CODECOV_DIR}/codecov.bash
	find ${CURDIR}/target/ -name "*$$(bash scripts/cargo-project-name.sh)*" -type f -executable | xargs --no-run-if-empty rm -rf
	RUSTFLAGS="-Clink-dead-code" CARGO_ARGS="--tests" ${MAKE} build-all-flavours
	find target/ -maxdepth 2 -name "*$$(bash scripts/cargo-project-name.sh)*" -type f -executable | while read executable; do \
	echo "Run $${executable}" > /dev/stderr && \
	mkdir -p ${CURDIR}/.coverage/$$(basename $${executable}) &&  \
	kcov --include-path=${CURDIR} --strip-path=${CURDIR} ${CURDIR}/.coverage/$$(basename $${executable}) $${executable}; \
	done
	find ${CURDIR}/.coverage/ -maxdepth 1 -type d -name "*$$(bash scripts/cargo-project-name.sh)*" | \
		xargs kcov --merge ${CURDIR}/.coverage/merged/
	[ "${TRAVIS}" = "true" ] && \
		bash ${CODECOV_DIR}/codecov.bash -f ${CURDIR}/.coverage/merged/kcov-merged/cobertura.xml || \
		echo "Skip codecov uploads"

# Docker support
# The support is mostly needed to be able to run coverage tools on non Linux systems (ie. Mac OS)
REPO_NAME := $(shell basename ${CURDIR} | tr A-Z a-z)
GIT_SHA := $(shell git rev-parse HEAD 2> /dev/null || echo "no-sha")
DOCKER_PREFIX := ${REPO_NAME}

.PHONY: docker-container-build
docker-container-build:
	docker build -t ${REPO_NAME}:${GIT_SHA} .

.PHONY: docker-volumes-create
docker-volumes-create:
	docker volume create ${DOCKER_PREFIX}_registry
	docker volume create ${DOCKER_PREFIX}_target
	docker volume create ${DOCKER_PREFIX}_sccache

.PHONY: clean-docker-volumes
clean-docker-volumes:
	docker volume rm ${DOCKER_PREFIX}_registry ${DOCKER_PREFIX}_target ${DOCKER_PREFIX}_sccache

.PHONY: start-container
docker-start: docker-container-build docker-volumes-create
	docker run \
		--env RUST_BACKTRACE=full \
		--interactive \
		--privileged \
		--rm \
		--volume ${CURDIR}/.coverage:/code/.coverage \
		--volume ${CURDIR}/Cargo.toml:/code/Cargo.toml:ro \
		--volume ${CURDIR}/Makefile:/code/Makefile:ro \
		--volume ${CURDIR}/src/:/code/src/:ro \
		--volume ${CURDIR}/scripts/:/code/scripts/:ro \
		--volume ${DOCKER_PREFIX}_registry:/root/.cargo/registry \
		--volume ${DOCKER_PREFIX}_target:/code/target \
		--tty \
		${REPO_NAME}:${GIT_SHA} \
		${CONTAINER_COMMAND}

.PHONY: clean-coverage
clean-coverage:
	rm -rf ${CURDIR}/.coverage/*

.PHONY: coverage-in-container
coverage-in-container: clean-coverage .coverage
	CONTAINER_COMMAND='make coverage' ${MAKE} docker-start

.PHONY: expand-macros
expand-macros:
	cargo +${RUST_TOOLCHAIN} rustc --tests --all-features -- -Z external-macro-backtrace -Z unstable-options --pretty=expanded
