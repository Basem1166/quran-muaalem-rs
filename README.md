# muaalem-engine
بفضل الله وحده لا شريك له:  محرك المعلم القرآني لتصحيح  تلاوة القرآن الكريم من حروف وصفات وتجويد يعمل في أي مكان وب SKD ل Python, Kotlin, Swift عال الأداء بلغة ال Rust

## Embedded Quran data assets

The `quran-muaalem-transcript` crate embeds its Quran JSON assets with
`include_str!`. This keeps the SDK self-contained, so applications do not need
to install or locate separate data files at runtime.

The four embedded files total **16,847,464 bytes (16.07 MiB)**:

- `begin_with_hamzat_wasl.json`: 11,267 bytes
- `quran-alphabet.json`: 10,607 bytes
- `quran-uthmani-imlaey.json`: 3,334,174 bytes
- `quran-uthmani-imlaey-map.json`: 13,491,416 bytes

On 2026-09-12, using `rustc 1.96.0` on Linux x86-64, two minimal programs were
compiled with `rustc -C opt-level=3 -C strip=symbols`. The baseline executable
was 338,704 bytes and the executable referencing all four embedded assets was
17,186,224 bytes: an increase of **16,847,520 bytes (16.07 MiB)**. That is 56
bytes more than the raw asset total. Final application size can vary by target,
toolchain, linker, compression, and which library sections are retained.
