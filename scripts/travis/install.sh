#!/usr/bin/env bash
set -euo pipefail -o posix -o functrace
# The script is needed to install all the needed dependencies on the CI server
# Usage: bash <path to this file>

install_make() {
  # Install make on windows
  if [[ "${TRAVIS_OS_NAME}" == "windows" ]]; then
      choco install make
  fi
}

build_venv() {
    make -C "${TRAVIS_BUILD_DIR}" venv
}

install_kcov() {
  if [[ ${MAKE_TARGET} == "coverage" ]]; then
    if [[ "${KCOV_DIR:-}" == "" ]]; then
        KCOV_DIR="$(mktemp -d)"
    fi
    mkdir -p "${KCOV_DIR}"
    pushd "${KCOV_DIR}"
    if command -v cargo-kcov &> /dev/null; then
        echo "# Skipping cargo-kcov as already present"
    else
        # TODO: use `cargo install cargo-kcov once https://github.com/kennytm/cargo-kcov/issues/41 is closed
        cargo install --git https://github.com/kennytm/cargo-kcov cargo-kcov
    fi
    if command -v kcov &> /dev/null; then
        echo "# Skipping kcov install as already present"
    else
        cargo kcov --print-install-kcov-sh | PARALLEL_BUILD=enabled sh
    fi
    if [[ "${TRAVIS_OS_NAME}" == "linux" ]]; then
      # Make sure that we could run kcov tool on linux
      sudo sh -c "echo 0 > /proc/sys/kernel/yama/ptrace_scope"
    fi
    pip install --no-cache --user coverage
    popd
  else
    echo "# Skipping kcov install as target is not coverage" > /dev/stderr
  fi
}

install_lint_tools() {
  # Install pre-commit and lint tools
  if [[ ${MAKE_TARGET} == "lint" ]]; then
    if rustup component list --toolchain="${TRAVIS_RUST_VERSION}" | grep installed | grep -q rustfmt; then
        echo "# Skipping rustfmt install as already present"
    else
        rustup component add rustfmt --toolchain="${TRAVIS_RUST_VERSION}"
    fi
    if rustup component list --toolchain="${TRAVIS_RUST_VERSION}" | grep installed | grep -q clippy; then
        echo "# Skipping clippy install as already present"
    else
        # Workaround in case clippy is not available in the current nightly release (https://github.com/rust-lang/rust-clippy#travis-ci)
        rustup component add clippy --toolchain="${TRAVIS_RUST_VERSION}" || cargo +"${TRAVIS_RUST_VERSION}" install --git https://github.com/rust-lang/rust-clippy/ --force clippy
    fi

  else
    echo "# Skipping lint-tools install as target is not coverage" > /dev/stderr
  fi
}

install_make
build_venv
install_kcov
install_lint_tools
