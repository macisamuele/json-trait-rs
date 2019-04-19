#!/usr/bin/env bash
set -euo pipefail -o posix -o functrace

REPO_ROOT="$( cd "$( dirname "${BASH_SOURCE[0]}" )/../" >/dev/null 2>&1 && pwd )"

if ! test -d "${REPO_ROOT}/venv"; then
    echo "Run \`make -C \"${REPO_ROOT}\" venv\`" > /dev/stderr
    exit 1
fi

"${REPO_ROOT}/venv/bin/python" -c "$(cat <<EOF
from __future__ import print_function
import toml
with open("${REPO_ROOT}/Cargo.toml") as f:
    print(toml.load(f).get("package", {}).get("version"))
EOF
)"
