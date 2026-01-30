# Latihan — Contoh kecil Rust dengan tracing

Project ini berisi contoh sederhana operasi aritmatika di Rust, dilengkapi logging menggunakan `tracing`.

Kenapa lihat ini?

- Cocok untuk pemula yang ingin melihat cara modul dan crate bekerja di Rust.
- Menunjukkan penggunaan `tracing` + `tracing-subscriber` untuk logging yang rapi.

Fitur

- Fungsi aritmatika: `tambah`, `kurang`, `hasil` (modulo)
- Logging level `info` untuk setiap operasi

Cara pakai

1. Build:

```bash
cargo build
```

2. Run:

```bash
cargo run
```
atau watch
```bash
cargo watch -q -c -w src/ -x run
```
Contoh output (dengan subscriber aktif):

```
INFO my_crate::aritmath: a ditambah 2 = 12
INFO my_crate::aritmath: b dikurang 2 = 8
INFO my_crate::aritmath: c dibagi 2 sisa = 0
```

Struktur penting

- `src/main.rs` — binary, menginisialisasi subscriber dan memanggil fungsi
- `src/lib.rs` — exposes `aritmath` sebagai library
- `src/aritmath/math.rs` — implementasi fungsi aritmatika

Kontribusi

Pull request dan isu diterima. Jika ingin bereksperimen: tambahkan lebih banyak operasi, tests, atau konfigurasi logging.

Lisensi

Gunakan bebas untuk eksperimen dan pembelajaran.

# a simple code from noob

