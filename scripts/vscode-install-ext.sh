#!/bin/sh
# Make sure that you turn autoupdate off, otherwise
# you might end up wondering why everything still works
# even when you break it on purpose.
(code --install-extension node/vscode/*.vsix --force)