# UTS PPL

## Build & Run

```sh
cargo run # or cargo build
```

__NOTE: Example implemntation at `main.rs`__

- [Main Entry](./src/main.rs)

## 1. Strategy pattern untuk kalkulator

- Strategy Pattern memungkinkan untuk mendefinisikan sekelompok algoritma (dalam hal ini, operasi matematika) dan membuatnya dapat dipertukarkan satu sama lain saat runtime. Pola ini memisahkan logika algoritma dari struktur utama aplikasi (tidak menggunakan switch atau if).

- [Source code](./src/calc.rs)

- menggunakan rust dan system traits rust yang mirip dengan inheritance tapi bukan inheritance.
- menggunakan `<T>` (Generics) agar bisa digukanan dengan berbagai tipe data number.

## 2. Command dan memento untuk todolist

- Menggunakan strategy command agar dapat memisah logika dalam object sendiri (TodoHist) yang dapat diparse dengan mudah.
- Disini menggunakan DOD (Data Oriented design) jadi tidak menggunakan inheritance dan sebagainya.
- Menggunakan memento agar dapat menyimpan action yang dilakukan oleh user agar mendapatkan feature undo, redo.

- [Source code](./src/todo.rs)

- Menggunakan TodoHist untuk menyimpan command yang dirunning
- Terdapat enum yang berupa macam command yang possible (tidak menggunakan OOP konsep)
- Menyimpan undo dan redo list ke dalam Vector jadi akan bisa store data tergantung memory anda berapa besar.

## 3. Observer untuk suhu

- Strategy observer digunakan agar obj bisa saling berkomunikasi satu sama lain bila ada perubahan.

- [Source code](./src/suhu.rs)

- Definisi beberapa traits yang akan digunakan seperti subject dan observer.
- Display mengimplemntasikan traits observer agar dapat menerima data yang akan dikirimkan.
- Subject sebagai pemanggil observer.
