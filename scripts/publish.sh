#!/usr/bin/env bash
set -euo pipefail

if [[ $# -ne 1 ]]; then
    echo "Usage: $0 <new-version>"
    exit 1
fi

VERSION="$1"
if [[ ! "$VERSION" =~ ^[0-9]+\.[0-9]+\.[0-9]+$ ]]; then
    echo "error: version must be in SemVer format (e.g. 1.2.3)" >&2
    exit 1
fi

if ! command -v cargo >/dev/null 2>&1; then
    echo "error: cargo is required" >&2
    exit 1
fi

if ! command -v git >/dev/null 2>&1; then
    echo "error: git is required" >&2
    exit 1
fi

if ! git diff --quiet || ! git diff --cached --quiet; then
    echo "error: working tree is dirty. Commit or stash changes first." >&2
    exit 1
fi

if git rev-parse "v${VERSION}" >/dev/null 2>&1; then
    echo "error: tag v${VERSION} already exists" >&2
    exit 1
fi

echo "==> Bumping crate version to ${VERSION}"
cargo set-version "${VERSION}"

echo "==> Running tests"
cargo test

echo "==> Creating release commit"
git commit -am "Release v${VERSION}"

echo "==> Packaging crate"
cargo package

echo "==> Publishing crate to crates.io"
cargo publish

echo "==> Creating tag v${VERSION}"
git tag "v${VERSION}"

current_branch="$(git rev-parse --abbrev-ref HEAD)"
echo "==> Pushing branch ${current_branch} and tag v${VERSION}"
git push origin "${current_branch}"
git push origin "v${VERSION}"

echo "==> Release v${VERSION} published!"
