#!/bin/bash

set -eu

SOURCE_DIR="$(dirname -- ${BASH_SOURCE[0]:-$0})"

log() {
    echo -e "$1" >&2
}

_DEFAULT_INSTALL_DIR=${HOME}/bin
_INSTALL_DIR=${INSTALL_DIR:-${_DEFAULT_INSTALL_DIR}}
CMD_NAME="taplo"
COMMAND="${_INSTALL_DIR}/${CMD_NAME}"

TARGET=${INPUT_FILES:-"."}
if [[ -z $(ls ${TARGET} 2>/dev/null) ]]; then
    log "ERROR: Input files (${TARGET}) not found"
    exit 1
fi

if [[ ! -x ${COMMAND} ]]; then
    VERSION=${INPUT_VERSION:-"0.9.3"}
    if [[ "$(uname -m)" == "arm64" || "$(uname -m)" == "aarch64" ]]; then
        ARCH="aarch64"
    else
        ARCH="x86_64"
    fi
    UNAME=$(uname -s)
    if [[ "$UNAME" == "Darwin" ]]; then
        TARGET_FILE="full-darwin-${ARCH}"
        FILE_EXT="gz"
    elif [[ "$UNAME" == CYGWIN* || "$UNAME" == MINGW* || "$UNAME" == MSYS* ]]; then
        TARGET_FILE="windows-${ARCH}"
        FILE_EXT="zip"
    else
        TARGET_FILE="linux-${ARCH}"
        FILE_EXT="gz"
    fi
    FILE_NAME="${CMD_NAME}-${TARGET_FILE}.${FILE_EXT}"
    log "Downloading '${CMD_NAME}' v${VERSION}"
    wget --progress=dot:mega "https://github.com/tamasfe/taplo/releases/download/${VERSION}/${FILE_NAME}"

    mkdir -p ${_INSTALL_DIR}
    if [[ "$FILE_EXT" == "zip" ]]; then
        unzip -o "${FILE_NAME}" -d ${_INSTALL_DIR} ${CMD_NAME}.exe
    else
        gzip -d "${FILE_NAME}" -c >${COMMAND}
        chmod +x ${COMMAND}
    fi
    rm "${FILE_NAME}"
fi
echo "taplo_cmd=${COMMAND}" >>"${GITHUB_ENV}"
