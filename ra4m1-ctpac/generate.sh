#!/usr/bin/env bash

SVD="R7FA4M1AB.svd"
SUPPORT_DIR="../support"
SVD_DIR="${SUPPORT_DIR}/svd"
TRANSFORMS_DIR="${SUPPORT_DIR}/transforms"

FORM_VERSION="0.13.0"
SED=${SED:-$(which gsed)}
SED=${SED:-$(which sed)}

if [ "x${CHIPTOOL_CMD}" = "x" ]; then
    cargo install --git https://github.com/embassy-rs/chiptool --locked
fi

cargo install --version "${FORM_VERSION}" form || (echo "Couldn't install form, exiting" && exit 1)

FORM_CMD=$(which form)
if [ x"${FORM_CMD}" = "x" ]; then
    echo "Dependency ‘form’ does not appear to be installed.  Try: cargo install form"
    exit 1
fi

RUST_LOG=${RUST_LOG:-warn}
CHIPTOOL_CMD=${CHIPTOOL_CMD:-$(which chiptool)}

if [ x"${CHIPTOOL_CMD}" = "x" ]; then
    echo "Dependency ‘chiptool’ does not appear to be installed.  Try: cargo install --git https://github.com/embassy-rs/chiptool --locked"
    exit 1
fi

rm -f lib.rs device.x
rm -rf ./src/*

# Yeah, yeah, yeah
set -e

echo "*** Scanning for transforms"

_transforms=""
TRANSFORMS=""

for i in $(find ${TRANSFORMS_DIR} -name '*.yaml'); do
    if [ -f "${i}" ]; then
        _transforms="${_transforms} $(basename ${i})"
        TRANSFORMS="${TRANSFORMS} --transform $i"
    fi
done

echo "${_transforms}"

echo "*** Running Chiptool on ${SVD_DIR}/${SVD}"
RUST_LOG="${RUST_LOG}" "${CHIPTOOL_CMD}" generate --svd "${SVD_DIR}/${SVD}" ${TRANSFORMS}

echo "*** Running form"
RUST_LOG="${RUST_LOG}" "${FORM_CMD}" -i lib.rs -o ./src

echo "*** Reformatting"
rm -f lib.rs
cargo fmt
${SED} -i -e '1i #![doc = include_str!("../README.md")]' src/lib.rs

APPEND_TARGET="src/pfs.rs"

if [ ! -f "${APPEND_TARGET}" ]; then
    echo "${APPEND_TARGET} doesn't seem to exist. please update generate.rs"
    exit 1
fi

echo >> "${APPEND_TARGET}"
cat "${SUPPORT_DIR}/pfs_accessor.rs" >> ${APPEND_TARGET}
