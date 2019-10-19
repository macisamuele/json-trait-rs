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

install_grcov() {
  if [[ ${MAKE_TARGET} == "coverage" ]]; then
    if ! command -v grcov @> /dev/null; then
      cargo install grcov
    fi
  fi
}

download_codecov_bash_script() {
  if [[ ${MAKE_TARGET} == "coverage" ]]; then
    make "${CODECOV_DIR}/codecov.bash"
  fi
}

install_lint_tools() {
  # Install pre-commit and lint tools
  if [[ ${MAKE_TARGET} == "lint" ]]; then
    if ! (pip freeze | grep -q pre-commit); then
      pip install --no-cache-dir --user pre-commit
    fi
    if ! (rustup component list --toolchain="${TRAVIS_RUST_VERSION}" | grep installed | grep -q rustfmt); then
      rustup component add rustfmt --toolchain="${TRAVIS_RUST_VERSION}"
    fi
    if ! (rustup component list --toolchain="${TRAVIS_RUST_VERSION}" | grep installed | grep -q clippy); then
      # Workaround in case clippy is not available in the current nightly release (https://github.com/rust-lang/rust-clippy#travis-ci)
      rustup component add clippy --toolchain="${TRAVIS_RUST_VERSION}" || cargo +"${TRAVIS_RUST_VERSION}" install --git https://github.com/rust-lang/rust-clippy/ --force clippy
    fi
  fi
}

install_make
install_python
install_grcov
download_codecov_bash_script
install_lint_tools
