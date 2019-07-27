#!/usr/bin/env bash
set -euo pipefail -o posix -o functrace

if echo "${RUST_TOOLCHAIN:-stable}" | grep --quiet "^nightly" ; then
    features_to_ignore=""
else
    features_to_ignore="$(grep -v '#' features-enabled-on-nightly-only 2> /dev/null || true)"
    echo "Ignoring the following features as they are available only on nightly rust: ${features_to_ignore}" > /dev/stderr
fi

cargo metadata --format-version 1 | FEATURES_TO_IGNORE=${features_to_ignore} python -c "$(cat <<EOF
from __future__ import print_function
import os
import json
import sys

features_to_ignore = os.environ['FEATURES_TO_IGNORE'].split()
metadata = json.load(sys.stdin)
package_metadata = next(item for item in metadata['packages'] if item['id'] == metadata['resolve']['root'])
print(' '.join(filter(lambda item: item != 'default' and item not in features_to_ignore, package_metadata['features'])))
EOF
)"
