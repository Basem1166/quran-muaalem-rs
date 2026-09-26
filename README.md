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
