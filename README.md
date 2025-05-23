### 2.1. Original code of broadcast chat

(![alt text](image-1.png))

Implementasi broadcast chat berhasil dijalankan dengan server yang mendengarkan di port 2000 dan tiga client yang terhubung secara bersamaan. Screenshot menunjukkan bagaimana tiap pesan ("ini satu", "ini dua", "ini tiga") yang dikirim oleh satu client diteruskan oleh server ke semua client yang terhubung. Server mencatat setiap koneksi baru dengan alamat IP dan port masing-masing client (127.0.0.1:51245, 127.0.0.1:51246, 127.0.0.1:51247). Sistem broadcast berfungsi sebagaimana mestinya, dimana setiap pesan yang dikirim ke server didistribusikan kembali ke semua client tanpa terkecuali. Arsitektur broadcast ini memungkinkan komunikasi grup dimana semua peserta dapat melihat pesan dari siapa saja dalam jaringan.


### 2.2. Modifying port

![alt text](image-2.png)

Untuk implementasi broadcast chat dengan port yang dimodifikasi, perlu dilakukan perubahan pada dua file utama. Pertama, pada server.rs, port binding diubah dari 8000 menjadi 8080 dengan mengubah 

`TcpListener::bind("127.0.0.1:8000")` menjadi `TcpListener::bind("127.0.0.1:8080")`. 


Kedua, pada client.rs, URI WebSocket diubah dari 

`ws://127.0.0.1:8000` menjadi `ws://127.0.0.1:8080`.

Aplikasi tetap menggunakan protokol WebSocket yang diindikasikan oleh skema URI ws:// pada client dan penggunaan library tokio_websockets dengan komponen ServerBuilder dan WebSocketStream pada server. Setelah modifikasi port dilakukan, fungsionalitas broadcast masih berjalan dengan baik karena seluruh komponen aplikasi telah disesuaikan untuk menggunakan port yang sama (8080) dan protokol yang sama (WebSocket). Pengujian dengan multiple client menunjukkan bahwa pesan tetap berhasil didistribusikan ke semua client yang terhubung.