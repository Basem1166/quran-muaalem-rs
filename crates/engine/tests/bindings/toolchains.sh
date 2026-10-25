#!/usr/bin/env bash
# Toolchain helper for the BoltFFI binding smoke tests. Called by the root Makefile:
#
#   toolchains.sh install-boltffi        install the CLI pinned by Cargo.toml ([workspace.dependencies] boltffi)
#   toolchains.sh install <lang>|all     download a language toolchain into .toolchains/ (no sudo)
#   toolchains.sh doctor [cli] [<lang>...] pre-flight checks (default: all this host can test); prints fixes
#   toolchains.sh run <cmd...>           run a command with .toolchains/ on PATH (JAVA_HOME, ANDROID_NDK_HOME, ...)
#
# Languages: python java kotlin dart swift. Downloads support Linux and macOS (x86_64/arm64).
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../../../.." && pwd)"
TC_DIR="$ROOT/.toolchains"

# Pinned toolchain versions (CI installs the same ones where it downloads them).
KOTLIN_VERSION=2.2.20
DART_VERSION=3.13.5
DART_MIN_VERSION=3.10.8 # the generated pubspec requires sdk ^3.10.8
JDK_MAJOR=21
NDK_VERSION=r27c
PYTHON_VERSION=3.12

boltffi_version() {
    sed -n 's/^boltffi = "\(.*\)"/\1/p' "$ROOT/Cargo.toml"
}

# --- environment ----------------------------------------------------------------

setup_env() {
    mkdir -p "$TC_DIR/bin"
    export PATH="$TC_DIR/bin:$PATH"
    if [ -d "$TC_DIR/jdk" ]; then
        export JAVA_HOME="$TC_DIR/jdk"
        export PATH="$JAVA_HOME/bin:$PATH"
    elif [ -z "${JAVA_HOME:-}" ] && [ -x /usr/libexec/java_home ] && /usr/libexec/java_home >/dev/null 2>&1; then
        JAVA_HOME="$(/usr/libexec/java_home)" # macOS: /usr/bin/javac is only a stub
        export JAVA_HOME
    elif [ -z "${JAVA_HOME:-}" ] && command -v javac >/dev/null; then
        # boltffi needs JAVA_HOME for the JNI headers; derive it from javac
        JAVA_HOME="$(dirname "$(dirname "$(realpath "$(command -v javac)")")")"
        export JAVA_HOME
    fi
    [ -d "$TC_DIR/kotlinc" ] && export PATH="$TC_DIR/kotlinc/bin:$PATH"
    [ -d "$TC_DIR/dart-sdk" ] && export PATH="$TC_DIR/dart-sdk/bin:$PATH"
    if [ -z "${ANDROID_NDK_HOME:-}" ] && [ -d "$TC_DIR/ndk" ]; then
        export ANDROID_NDK_HOME="$TC_DIR/ndk"
    fi
    return 0
}

host_os() {
    case "$(uname -s)" in
        Linux) echo linux ;;
        Darwin) echo macos ;;
        *) echo unsupported ;;
    esac
}

host_arch() {
    case "$(uname -m)" in
        x86_64 | amd64) echo x86_64 ;;
        aarch64 | arm64) echo arm64 ;;
        *) echo unsupported ;;
    esac
}

require_supported_host() {
    if [ "$(host_os)" = unsupported ] || [ "$(host_arch)" = unsupported ]; then
        echo "error: toolchain downloads support Linux and macOS only ($(uname -s) $(uname -m)); install it manually" >&2
        exit 1
    fi
}

download() { # url dest
    echo "  downloading $1"
    curl -fL --progress-bar -o "$2" "$1"
}

version_ge() { # $1 >= $2, dotted versions
    local IFS=.
    local -a a=($1) b=($2)
    local i
    for i in 0 1 2; do
        ((10#${a[i]:-0} > 10#${b[i]:-0})) && return 0
        ((10#${a[i]:-0} < 10#${b[i]:-0})) && return 1
    done
    return 0
}

# --- doctor ---------------------------------------------------------------------

failures=0
ok() { printf '  \033[32m✓\033[0m %s\n' "$1"; }
bad() { # message fix
    printf '  \033[31m✗\033[0m %s\n      fix: %s\n' "$1" "$2"
    failures=$((failures + 1))
}

check_cli() {
    echo "boltffi CLI"
    local want have
    want="$(boltffi_version)"
    command -v cargo >/dev/null && ok "cargo" || bad "cargo not found" "install Rust from https://rustup.rs"
    if ! command -v boltffi >/dev/null; then
        bad "boltffi not found (want $want)" "make install-boltffi"
        return
    fi
    have="$(boltffi --version | awk '{print $2}')"
    if [ "$have" = "$want" ]; then
        ok "boltffi $have (matches Cargo.toml)"
    else
        bad "boltffi $have, but Cargo.toml pins $want" "make install-boltffi"
    fi
}

system_pkg_hint() { # package -> install command for this OS
    if [ "$(host_os)" = macos ]; then echo "xcode-select --install"; else echo "sudo apt install $1 (or your distro's equivalent)"; fi
}

check_c_compiler() { # the Python wheel's C extension is built by setuptools with the system compiler
    [ "$(host_os)" = unsupported ] && return # e.g. Windows: setuptools uses MSVC, not cc
    command -v cc >/dev/null && ok "C compiler: $(command -v cc)" || bad "no C compiler (cc) found" "$(system_pkg_hint build-essential)"
}

check_clang() { # boltffi links the JNI desktop library with clang on Linux and macOS
    [ "$(host_os)" = unsupported ] && return
    command -v clang >/dev/null && ok "clang" || bad "clang not found (boltffi uses it to link the JNI library)" "$(system_pkg_hint clang)"
}

check_python_venv() {
    local py="${PYTHON:-python3}"
    if ! command -v "$py" >/dev/null; then
        bad "$py not found" "make bind-python"
    elif ! "$py" -c 'import venv, ensurepip' 2>/dev/null; then
        bad "$py has no working venv module (e.g. Debian/Ubuntu without python3-venv)" "make bind-python"
    else
        ok "$("$py" --version) with venv"
    fi
}

check_python() {
    echo "python"
    check_python_venv
    check_c_compiler
}

check_jdk() {
    if ! command -v javac >/dev/null; then
        bad "javac not found (a JRE is not enough; a full JDK is needed)" "make bind-java"
    elif [ -z "${JAVA_HOME:-}" ] || [ ! -f "$JAVA_HOME/include/jni.h" ]; then
        bad "JAVA_HOME is not a JDK (no include/jni.h): ${JAVA_HOME:-unset}" "make bind-java, or point JAVA_HOME at a JDK"
    else
        ok "JDK: $(javac -version 2>&1) (JAVA_HOME=$JAVA_HOME)"
    fi
}

check_java() {
    echo "java"
    check_jdk
    check_clang
}

check_kotlin() {
    echo "kotlin"
    check_jdk
    check_clang
    command -v kotlinc >/dev/null && ok "kotlinc" || bad "kotlinc not found" "make bind-kotlin"
    command -v rustup >/dev/null && ok "rustup" || bad "rustup not found (needed to add the Android Rust target)" "install Rust from https://rustup.rs"
    if [ -n "${ANDROID_NDK_HOME:-}" ] && [ -d "$ANDROID_NDK_HOME" ]; then
        ok "Android NDK: $ANDROID_NDK_HOME"
    else
        bad "Android NDK not found (ANDROID_NDK_HOME=${ANDROID_NDK_HOME:-unset})" "make bind-kotlin, or set ANDROID_NDK_HOME"
    fi
}

check_dart() {
    echo "dart"
    if ! command -v dart >/dev/null; then
        bad "dart not found" "make bind-dart"
        return
    fi
    local v
    v="$(dart --version 2>&1 | sed -n 's/.*version: \([0-9.]*\).*/\1/p')"
    if version_ge "$v" "$DART_MIN_VERSION"; then
        ok "Dart $v"
    else
        bad "Dart $v is older than $DART_MIN_VERSION" "make bind-dart"
    fi
}

check_swift() {
    echo "swift"
    if [ "$(host_os)" != macos ]; then
        bad "the Swift test packs an Apple xcframework, so it needs macOS (CI runs it on macos-latest)" "run it on a Mac"
        return
    fi
    command -v xcodebuild >/dev/null && ok "xcodebuild" || bad "xcodebuild not found" "install Xcode from the App Store, then: sudo xcode-select -s /Applications/Xcode.app"
    command -v swift >/dev/null && ok "$(swift --version 2>&1 | head -1)" || bad "swift not found" "install Xcode"
}

passes() { # quietly run check_<name>; succeed only if it reported no problems
    (failures=0; "check_$1" >/dev/null; [ "$failures" -eq 0 ])
}

doctor() {
    if [ $# -eq 0 ]; then # default: everything this machine can test (Swift needs macOS)
        set -- cli python java kotlin dart
        [ "$(host_os)" = macos ] && set -- "$@" swift
    fi
    local what
    for what in "$@"; do "check_$what"; done
    if [ "$failures" -gt 0 ]; then
        echo "doctor: $failures problem(s) found" >&2
        exit 1
    fi
    echo "doctor: all good"
}

# --- installs -------------------------------------------------------------------

install_boltffi() {
    local want
    want="$(boltffi_version)"
    if command -v boltffi >/dev/null && [ "$(boltffi --version | awk '{print $2}')" = "$want" ]; then
        echo "boltffi $want already installed"
        return
    fi
    cargo install boltffi_cli --version "$want" --locked
}

install_python() {
    if passes python_venv; then
        echo "python: already available ($(${PYTHON:-python3} --version))"
        return
    fi
    require_supported_host
    if [ ! -x "$TC_DIR/bin/uv" ]; then
        curl -LsSf https://astral.sh/uv/install.sh | env UV_UNMANAGED_INSTALL="$TC_DIR/bin" sh
    fi
    UV_PYTHON_INSTALL_DIR="$TC_DIR/python" "$TC_DIR/bin/uv" python install "$PYTHON_VERSION"
    ln -sf "$(UV_PYTHON_INSTALL_DIR="$TC_DIR/python" "$TC_DIR/bin/uv" python find "$PYTHON_VERSION")" "$TC_DIR/bin/python3"
    echo "python: installed $("$TC_DIR/bin/python3" --version) into .toolchains/"
}

install_jdk() {
    if passes jdk; then
        echo "jdk: already available (JAVA_HOME=$JAVA_HOME)"
        return
    fi
    require_supported_host
    local os arch tmp
    os="$(host_os)"; [ "$os" = macos ] && os=mac
    arch="$(host_arch)"; arch="${arch/x86_64/x64}"; arch="${arch/arm64/aarch64}"
    tmp="$(mktemp -d)"
    download "https://api.adoptium.net/v3/binary/latest/$JDK_MAJOR/ga/$os/$arch/jdk/hotspot/normal/eclipse" "$tmp/jdk.tar.gz"
    tar -xzf "$tmp/jdk.tar.gz" -C "$tmp"
    rm -rf "$TC_DIR/jdk"
    local home
    home="$(find "$tmp" -maxdepth 4 -type f -path '*/bin/javac' | head -1)"
    mv "$(dirname "$(dirname "$home")")" "$TC_DIR/jdk"
    rm -rf "$tmp"
    echo "jdk: installed Temurin $JDK_MAJOR into .toolchains/jdk"
}

install_java() {
    install_jdk
}

install_kotlin() {
    install_jdk
    require_supported_host
    if command -v kotlinc >/dev/null; then
        echo "kotlinc: already available"
    else
        local tmp
        tmp="$(mktemp -d)"
        download "https://github.com/JetBrains/kotlin/releases/download/v$KOTLIN_VERSION/kotlin-compiler-$KOTLIN_VERSION.zip" "$tmp/kotlinc.zip"
        rm -rf "$TC_DIR/kotlinc"
        unzip -q "$tmp/kotlinc.zip" -d "$TC_DIR"
        rm -rf "$tmp"
        echo "kotlinc: installed $KOTLIN_VERSION into .toolchains/kotlinc"
    fi
    if [ -n "${ANDROID_NDK_HOME:-}" ] && [ -d "$ANDROID_NDK_HOME" ]; then
        echo "ndk: already available ($ANDROID_NDK_HOME)"
    else
        install_ndk
    fi
}

install_ndk() {
    local tmp
    tmp="$(mktemp -d)"
    rm -rf "$TC_DIR/ndk"
    if [ "$(host_os)" = linux ]; then
        download "https://dl.google.com/android/repository/android-ndk-$NDK_VERSION-linux.zip" "$tmp/ndk.zip"
        unzip -q "$tmp/ndk.zip" -d "$tmp"
        mv "$tmp/android-ndk-$NDK_VERSION" "$TC_DIR/ndk"
    else
        download "https://dl.google.com/android/repository/android-ndk-$NDK_VERSION-darwin.dmg" "$tmp/ndk.dmg"
        hdiutil attach -quiet -nobrowse -mountpoint "$tmp/mnt" "$tmp/ndk.dmg"
        cp -R "$(find "$tmp/mnt" -maxdepth 4 -type d -name NDK | head -1)" "$TC_DIR/ndk"
        hdiutil detach -quiet "$tmp/mnt"
    fi
    rm -rf "$tmp"
    export ANDROID_NDK_HOME="$TC_DIR/ndk"
    echo "ndk: installed $NDK_VERSION into .toolchains/ndk"
}

install_dart() {
    if passes dart; then
        echo "dart: already available"
        return
    fi
    require_supported_host
    local os arch tmp
    os="$(host_os)"
    arch="$(host_arch)"; arch="${arch/x86_64/x64}"
    tmp="$(mktemp -d)"
    download "https://storage.googleapis.com/dart-archive/channels/stable/release/$DART_VERSION/sdk/dartsdk-$os-$arch-release.zip" "$tmp/dart.zip"
    rm -rf "$TC_DIR/dart-sdk"
    unzip -q "$tmp/dart.zip" -d "$TC_DIR"
    rm -rf "$tmp"
    echo "dart: installed $DART_VERSION into .toolchains/dart-sdk"
}

install_swift() {
    if [ "$(host_os)" != macos ]; then
        echo "swift: skipped, the Swift test needs macOS with Xcode (CI runs it on macos-latest)"
        return
    fi
    if passes swift; then
        echo "swift: already available"
    else
        echo "swift: Xcode can't be installed from the command line. Install it from the App Store, then run: sudo xcode-select -s /Applications/Xcode.app" >&2
        exit 1
    fi
}

install() {
    local lang="${1:?usage: toolchains.sh install <lang>|all}"
    if [ "$lang" = all ]; then
        for lang in python java kotlin dart swift; do "install_$lang"; done
    else
        "install_$lang"
    fi
}

# --- main -----------------------------------------------------------------------

setup_env
cmd="${1:?usage: toolchains.sh install-boltffi | install <lang> | doctor [cli] [<lang>...] | run <cmd...>}"
shift
case "$cmd" in
    install-boltffi) install_boltffi ;;
    install) install "$@" ;;
    doctor) doctor "$@" ;;
    run) exec "$@" ;;
    *) echo "unknown command: $cmd" >&2; exit 2 ;;
esac
