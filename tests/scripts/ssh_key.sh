#!/usr/bin/env shhh
set -euo pipefail

# See: https://gist.github.com/kraftb/9918106
mkfifo key key.pub
cat key key.pub & echo "y" | ssh-keygen -f key -N ""; rm key key.pub