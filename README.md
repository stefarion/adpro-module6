# Advanced Programming - Concurrency in Rust
**Nama:**   &nbsp; Stefanus Tan Jaya<br>
**NPM:**    &nbsp;&ensp; 2306152456<br>
**Kelas:**  &nbsp; Pemrograman Lanjut A<br>

### Module Reflection 6
- [Commit 1](#commit-1)

## Commit 1
Identifikasi isi fungsi `handle_connection`:
- Fungsi ini menerima parameter `stream` bertipe `TcpStream` dan bersifat _mutable_ karena nantinya data dapat dibaca dan diubah dari `stream` tersebut.
- Variabel `buf_reader` bertugas untuk menyimpan hasil bacaan data dari `stream` menggunakan `BufReader`. `BufReader` membantu membaca data lebih efisien terhadap data besar dengan memotongnya menjadi bagian yang lebih kecil.
- Variabel `http_request` bertugas untuk menyimpan vektor _request_ HTTP yang berasal dari `buf_reader`.
- _Method_ `lines()` akan mengembalikan setiap baris teks pada `buf_reader` dalam iterasi. Setiap elemen dari iterator ini bertipe `Result<String, Error>`.
- Selanjutnya, `map()` digunakan untuk mengubah setiap `Result` menjadi String dengan `unwrap()`. Jika terjadi error, `unwrap()` akan menyebabkan _panic_ dan menghentikan program.
- `take_while(|line| !line.is_empty())` akan mengambil baris-baris dari iterator sampai menemukan baris kosong dan berhenti. Dalam protokol HTTP, baris kosong menandakan akhir _header request_.
- `collect()` bertugas untuk mengumpulkan semua baris dan menyimpannya dalam bentuk `Vec` atau `Vector`. Terakhir, vektor `http_request` di-_print_ ke terminal. 