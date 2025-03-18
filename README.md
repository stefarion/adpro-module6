# Advanced Programming - Concurrency in Rust
**Nama:**   &nbsp; Stefanus Tan Jaya<br>
**NPM:**    &nbsp;&ensp; 2306152456<br>
**Kelas:**  &nbsp; Pemrograman Lanjut A<br>

### Module Reflection 6
- [Commit 1](#commit-1)
- [Commit 2](#commit-2)
- [Commit 3](#commit-3)
- [Commit 4](#commit-4)
- [Commit 5](#commit-5)
- [Bonus](#commit-bonus)

## Commit 1
Identifikasi isi fungsi `handle_connection`:
- Fungsi ini menerima parameter `stream` bertipe `TcpStream` dan bersifat _mutable_ karena nantinya data dapat dibaca dan diubah dari `stream` tersebut.
- Variabel `buf_reader` bertugas untuk menyimpan hasil bacaan data dari `stream` menggunakan `BufReader`. `BufReader` membantu membaca data lebih efisien terhadap data besar dengan memotongnya menjadi bagian yang lebih kecil.
- Variabel `http_request` bertugas untuk menyimpan vektor _request_ HTTP yang berasal dari `buf_reader`.
- _Method_ `lines()` akan mengembalikan setiap baris teks pada `buf_reader` dalam iterasi. Setiap elemen dari iterator ini bertipe `Result<String, Error>`.
- Selanjutnya, `map()` digunakan untuk mengubah setiap `Result` menjadi String dengan `unwrap()`. Jika terjadi error, `unwrap()` akan menyebabkan _panic_ dan menghentikan program.
- `take_while(|line| !line.is_empty())` akan mengambil baris-baris dari iterator sampai menemukan baris kosong dan berhenti. Dalam protokol HTTP, baris kosong menandakan akhir _header request_.
- `collect()` bertugas untuk mengumpulkan semua baris dan menyimpannya dalam bentuk `Vec` atau `Vector`. Terakhir, vektor `http_request` di-_print_ ke terminal. 

## Commit 2
![](/images/commit2.png)<br>

Identifikasi _line_ terbaru dari fungsi `handle_connection`:
- Tujuan penambahan _line_ untuk menampilkan _response_ HTTP.
- Variabel `status_line` berisi _string_ `"HTTP/1.1 200 OK"` yang menandakan bahwa _request_ berhasil diproses.
- Selanjutnya, program membaca isi _file_ `hello.html` dari `fs` atau `filesystem` dengan _method_ `fs::read_to_string()` dan menyimpannya ke variabel `contents` dalam bentuk String. `unwrap()` bertugas untuk mendapatkan nilai _string_-nya atau jika terjadi error, akan menyebabkan _panic_ dan menghentikan program.
- Variabel `length` bertugas untuk menyimpan panjang dari `contents`.
- Variabel `response` dibentuk dengan `format!` yang menggabungkan `status_line` sebagai status HTTP, `length` sebagai _header_, dan isi `contents` HTML. 
- `stream.write_all()` akan mengembalikan seluruh _response_ ke dalam _stream_ TCP. `as_bytes()` akan mengubah _string response_ menjadi _array bytes_ dan `unwrap()` akan memastikan penulisan ke `stream` berhasil. Jika gagal atau error, akan menyebabkan _panic_ dan menghentikan program. 

## Commit 3
![](/images/commit3.png)<br>

Implementasi kondisi halaman web tidak tersedia:
- Karena saya memanfaatkan variabel `buf_reader`, saya menghapus potongan kode berikut terlebih dahulu.
    ```
    let http_request:Vec<_> = buf_reader
    .lines()
    .map(|result|result.unwrap())
    .take_while(|line|!line.is_empty()) 
    .collect();
    ```
    Alasannya karena menimbulkan error `use of moved value`. Namun, potongan kode tersebut sudah tidak dibutuhkan lagi karena fungsi `handle_connection` saat ini sudah tidak menampilkan _request_ HTTP di terminal, melainkan menangani _response_ dari _request_ HTTP.
- `buf_reader` digunakan untuk membaca _request_ HTTP yang dikirim dan menyimpannya dalam variabel `request_line`.
- Kemudian, program mengecek _request_ tersebut dan memberikan _response_ yang sesuai dengan fungsi `send_response`. Jika request adalah `GET / HTTP/1.1`, maka akan mengembalikan `hello.html` dengan status `200 OK`. Selain itu, akan mengembalikan `404.html` dengan status `404 NOT FOUND`.
- Fungsi `send_response` merupakan hasil _refactor_ yang khusus untuk menangani pengiriman _response_. Tujuannya supaya tiap fungsi melakukan tugasnya masing-masing.
- Dalam `send_response`, saya mengimplementasikan cara yang sama pada Commit 2 untuk mengembalikan _response_ ke dalam _stream_ TCP dan penanganan error dengan `unwrap()`. Penyesuaian _response_ dilakukan pada `status_line` dan `file_name` HTML yang ingin ditunjukkan sebagai parameter fungsi `send_response`.

## Commit 4
Pemahaman simulasi _slow response_:
- Saya melakukan sedikit penyesuaian dalam menguji simulasi _slow response_. Saya membuat fungsi `handle_slow_response` khusus untuk menyimulasikan server dengan banyak _user_ yang mengaksesnya. Fungsi terdiri dari pembacaan _request_ seperti `handle_connection`, potongan kode pada modul yang menggunakan `thread` dan `Duration`, serta pemanggilan fungsi `send_response`.
- Dari potongan kode yang diberikan, setiap kali _request_ yang dikirim ke `http://127.0.0.1:7878/sleep` akan menghasilkan _delay_ selama 10 detik baru program akan mengirim _response_. Kekurangan dari kondisi ini adalah server tidak bisa menangani banyak _request_ karena server bersifat _single-threaded_, sehingga _request_ harus diproses satu per satu.

## Commit 5
Implementasi _Multithreaded Server_:
- Sebelumnya saya menghapus fungsi `handle_slow_response` dan memindahkan potongan kode yang menyimulasikan banyaknya _request_ ke fungsi `handle_connection` karena kedua fungsi tersebut menjalankan tugas yang sama.
- `ThreadPool` digunakan dalam program untuk membuat server dapat menangani banyak _request_ pada waktu yang bersamaan. `Threadpool` akan mengandalkan _worker_ yang sudah didefinisikan pada `lib.rs`.
- `let pool = ThreadPool::new(4)` berarti server membuat sebuah `Threadpool` dengan 4 _workers_. Variabel `pool` akan digunakan untuk mendistribusikan pekerjaan (`Job`) ke beberapa _thread_ agar server dapat menangani banyak _request_ secara konkuren.
- Setiap `stream` yang diterima akan menjalankan _method_ `execute` dari `ThreadPool`. Pekerjaan berupa `handle_connection(stream)` akan dibungkus dalam `Job` dan dikirim melalui _channel_ `Sender`. Salah satu _worker_ akan menerima `Job` tersebut dengan `receiver`.
- Variabel `receiver` akan diakses oleh banyak _worker_, sehingga `Arc` digunakan untuk kepemilikan bersama dan `Mutex` digunakan untuk memastikan hanya satu _thread worker_ yang mengakses `receiver` pada satu waktu.
- Setiap _worker_ akan berjalan dalam _loop_ terus-menerus hingga menerima `Job` melalui _channel_. Jika _channel_ terputus atau error, `unwrap()` akan menyebabkan _panic_ dan menghentikan program. Setelah menerima `Job`, _worker_ akan mengeksekusi pekerjaan tersebut dan setelah selesai, _worker_ akan kembali ke dalam _loop_.

## Commit Bonus
Dalam melakukan _improvement_ pada `ThreadPool`, saya mengganti fungsi `new` menjadi `build` yang sama-sama menerima parameter `size`, di mana `size` menentukan jumlah _thread worker_ yang akan dibuat. Perbedaan `new` sebelumnya dengan `build` terletak pada penanganan error. Dalam fungsi `new`, _method_ `assert!` akan membuat program _panic_ ketika `size` bernilai tidak valid (<=0) dan program langsung berhenti. Dalam fungsi `build`, ketika menghadapi `size` tidak valid, fungsi akan mengembalikan error `Err` berupa _string_ statis daripada menghentikan program langsung. Tujuannya untuk memberikan kesempatan bagi pemanggil fungsi untuk menangani error tersebut selagi program masih berjalan.