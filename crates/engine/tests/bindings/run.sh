#!/usr/bin/env bash
# Pack each BoltFFI binding and run its smoke test against the real native library.
#
# Usage: crates/engine/tests/bindings/run.sh [python] [java] [dart]   (default: all)
#
# Needs the boltffi CLI (`cargo install boltffi_cli`), plus per language:
#   python: python3 (override with PYTHON=...)
#   java:   a JDK with JAVA_HOME set
#   dart:   Dart SDK >= 3.10.8
# On Windows the Python/Java C glue needs MSVC >= 17.5 (C11 atomics) or clang-cl.
set -euo pipefail

cd "$(dirname "$0")/../.."
tests=tests/bindings

ENGINE_VERSION="$(cargo pkgid | sed 's/.*[@#]//')"
export ENGINE_VERSION

case "${OSTYPE:-}" in
    msys* | cygwin* | win32*) classpath_sep=";" ;;
    *) classpath_sep=":" ;;
esac

run_python() {
    boltffi pack python --release

    local venv="$tests/python/.venv"
    "${PYTHON:-python3}" -m venv "$venv"
    local py="$venv/bin/python"
    [ -x "$py" ] || py="$venv/Scripts/python.exe"

    "$py" -m pip install --quiet --force-reinstall dist/python/wheelhouse/*.whl
    "$py" -m unittest discover -s "$tests/python" -v
}

run_java() {
    boltffi pack java --release

    local classes=dist/java-test-classes
    rm -rf "$classes"
    javac -d "$classes" $(find dist/java -name '*.java') "$tests/java/VersionTest.java"

    local native_dirs
    native_dirs="$(find dist/java/native -mindepth 1 -maxdepth 1 -type d | paste -sd "$classpath_sep" -)"
    java -cp "$classes${classpath_sep}dist/java" -Djava.library.path="$native_dirs" VersionTest
}

dart_host_target() {
    local os arch
    case "$(uname -s)" in
        Linux) os=linux ;;
        Darwin) os=macos ;;
        MINGW* | MSYS* | CYGWIN*) os=windows ;;
        *) echo "unsupported OS: $(uname -s)" >&2; return 1 ;;
    esac
    case "$(uname -m)" in
        x86_64 | amd64) arch=x86_64 ;;
        aarch64 | arm64) arch=arm64 ;;
        *) echo "unsupported arch: $(uname -m)" >&2; return 1 ;;
    esac
    echo "$os:$arch"
}

run_dart() {
    # By default Dart packs every platform; the smoke test only needs the host.
    local overlay=dist/dart-host.toml
    mkdir -p dist
    printf '[targets.dart]\nnative_targets = ["%s"]\n' "$(dart_host_target)" > "$overlay"
    boltffi pack dart --release --overlay "$overlay"

    (cd "$tests/dart" && dart pub get && dart test)
}

langs=("$@")
[ ${#langs[@]} -gt 0 ] || langs=(python java dart)

for lang in "${langs[@]}"; do
    echo "==> $lang"
    "run_$lang"
done
