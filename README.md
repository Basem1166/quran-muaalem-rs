# muaalem-engine
بفضل الله وحده لا شريك له:  محرك المعلم القرآني لتصحيح  تلاوة القرآن الكريم من حروف وصفات وتجويد يعمل في أي مكان وب SKD ل Python, Kotlin, Swift عال الأداء بلغة ال Rust

## Embedded Quran data assets

The `quran-muaalem-transcript` crate embeds its Quran JSON assets with
`include_str!`. This keeps the SDK self-contained, so applications do not need
to install or locate separate data files at runtime.

The three embedded files total **3,356,048 bytes (3.20 MiB)**:

- `begin_with_hamzat_wasl.json`: 11,267 bytes
- `quran-alphabet.json`: 10,607 bytes
- `quran-uthmani-imlaey.json`: 3,334,174 bytes

`quran-uthmani-imlaey-map.json` (13,491,416 bytes) ships in `assets/` but is
not embedded into the binary.

## BoltFFI bindings

The `quran-muaalem-engine` crate exports its SDK API with [BoltFFI](https://boltffi.dev).
Every binding has a smoke test in [`crates/engine/tests/bindings/`](crates/engine/tests/bindings/):
it packs the real native library and calls `version()` from that language.
All commands below run from the repo root; `make help` lists them.

### Quick start

```sh
make install-boltffi     # the boltffi CLI version pinned in Cargo.toml
make doctor              # check every toolchain; prints the fix command for anything missing
make test-bind-all       # pack + smoke-test every binding this machine can run
```

Run one language with `make test-bind-<lang>`. It runs `make doctor-<lang>` first, so a missing
toolchain fails fast with the fix instead of after a full release build.

### Requirements

| Binding | Needs | `make bind-<lang>` installs (into `.toolchains/`, no sudo) |
|---|---|---|
| Python | `python3` with the `venv` module (on Debian/Ubuntu: `python3-venv`) | Python 3.12 via [uv](https://docs.astral.sh/uv/), if `python3` + `venv` aren't usable |
| Java (JVM) | a full **JDK** (`javac` + `JAVA_HOME`), not just a JRE | Temurin JDK 21 |
| Kotlin | JDK, `kotlinc`, `rustup`, and the Android NDK (`ANDROID_NDK_HOME`) | JDK, kotlinc 2.2.20 and NDK r27c (whichever are missing) |
| Dart | Dart SDK ≥ 3.10.8 | Dart 3.13.5 |
| Swift | macOS with Xcode | nothing (Xcode comes from the App Store) |

All of them also need Rust and the `boltffi` CLI (`make install-boltffi`).
`make bind-all` installs everything this machine can test.

### What's tested

| Binding | What the smoke test proves |
|---|---|
| Python | the wheel builds, installs into a fresh venv, and `version()` works |
| Java | the JNI library builds and `version()` works |
| Kotlin | **desktop JVM only**: the Android Kotlin bindings run through BoltFFI's `desktop_pack` on a normal JVM, not on Android |
| Dart | the host platform's native library only |
| Swift | the **macOS slice only**; no iOS device/simulator slices |

Not tested yet: wasm, the other Android architectures, iOS, and running on a real device or emulator.

Dart, Swift and Kotlin pass `run.sh` an `--overlay` config that builds only what the test needs.
`crates/engine/boltffi.toml` itself keeps every release target.
Packaged output lands in `crates/engine/dist/<target>/`, except Dart, which goes to
`crates/engine/tests/bindings/dart/generated/` (the generated package needs a Dart workspace root).

### CI

`.github/workflows/bindings.yml` runs `make test-bind-<lang>` for each binding: Swift on macOS, the
rest on Ubuntu. CI gets language SDKs from the official `actions/setup-*` actions, which GitHub
caches. `make bind-<lang>` is the local equivalent, and CI uses it only for `kotlinc`, which has
no official setup action. The `boltffi` CLI version comes from `Cargo.toml` in both places, so a
version bump is a one-file change. `.github/workflows/ci.yml` runs `make lint` and `make test`.

CI runs on Linux and macOS only; Windows isn't tested.
