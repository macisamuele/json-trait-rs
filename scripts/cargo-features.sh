#!/usr/bin/env bash
set -euo pipefail -o posix -o functrace

cargo metadata --format-version 1 | python -c "$(cat <<EOF
from __future__ import print_function
import json
import sys

metadata = json.load(sys.stdin)
package_metadata = next(item for item in metadata['packages'] if item['id'] == metadata['resolve']['root'])
print(' '.join(filter(lambda item: item != 'default', package_metadata['features'])))
EOF
)"
