Hasil screenshot setelah menjalankan 3 client dan 1 server : 
![Hasil komunikasi 3 client dan 1 server](resources/Result1.png)

Explanation : 
Untuk menjalankan program, kita perlu membuka 4 terminal dengan satu terminal menjalankan server (cargo run --bin server) dab 3 lainnya menjalankan client (cargo run --bin client).

Berdasarkan gambar, ketika kita mengetik sebuah teks (misalnya "halo1", "halo2", atau "halo3") di salah satu client dan menekan enter, terjadi sebuah siklus komunikasi asynchronous dua arah:

- Di sisi Client (Pengirim): Karena makro tokio::select!, aplikasi client dapat memantau input keyboard dan aliran data dari jaringan secara bersamaan tanpa saling menunggu (non-blocking). Saat kita menekan Enter, tugas pertama menangkap teks "halo1" dari terminal dan langsung mengirimkannya ke server melalui WebSocket.

- Di sisi Server: Fungsi handle_connection di server menerima pesan tersebut dari stream WebSocket. Server kemudian merangkai ulang pesannya dengan menambahkan identitas alamat IP dan port si pengirim (menjadi format 127.0.0.1:64272: halo1). Setelah itu, server melempar pesan yang sudah diformat tersebut ke dalam channel broadcast (bcast_tx).

- Distribusi Pesan (Broadcast): Karena setiap client yang terhubung memiliki receiver (bcast_rx) yang subscribe ke channel yang sama, channel tersebut mendistribusikan pesan 127.0.0.1:64272: halo1 ke semua fungsi handler koneksi yang sedang berjalan di server.

- Di sisi Client (Penerima): Server meneruskan kembali pesan tersebut ke semua client (termasuk si pengirim aslinya) melalui koneksi WebSocket masing-masing. Tugas kedua pada tokio::select! di setiap terminal client mendeteksi adanya pesan masuk dari server, lalu mencetaknya ke layar terminal.

