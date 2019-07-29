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

install_python() {
  # Install python on windows
  if [[ "${TRAVIS_OS_NAME}" == "windows" ]]; then
    choco install python --version "${PYTHON_VERSION:-3.7.4}"
  fi
}

install_kcov() {
  if [[ ${MAKE_TARGET} == "coverage" ]]; then
    GITHUB_GRCOV="https://api.github.com/repos/mozilla/grcov/releases/latest"
    GRCOV_DEFAULT_VERSION="v0.5.1"
    if [[ ${TRAVIS_OS_NAME} == "windows" ]]; then OS_NAME="win"; else OS_NAME=${TRAVIS_OS_NAME}; fi

    # Usage: download and install the latest kcov version by default.
    # Fall back to ${KCOV_DEFAULT_VERSION} from the kcov archive if the latest is unavailable.
    GRCOV_VERSION=$(curl --silent --show-error --fail ${GITHUB_GRCOV} | jq -Mr .tag_name || echo)
    GRCOV_VERSION=${GRCOV_VERSION:-${GRCOV_DEFAULT_VERSION}}
    GRCOV_TAR_BZ2="https://github.com/mozilla/grcov/releases/download/${GRCOV_VERSION}/grcov-${OS_NAME}-x86_64.tar.bz2"
    curl -L --retry 3 "${GRCOV_TAR_BZ2}" | tar xjf - -C "${CARGO_HOME:-${HOME}/.cargo/bin}"
  fi
}

install_lint_tools() {
  # Install pre-commit and lint tools
  if [[ ${MAKE_TARGET} == "lint" ]]; then
    pip install --no-cache-dir --user pre-commit
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
install_python
install_kcov
install_lint_tools
