# pastelpaste

pastelpaste is a minimal and modern pastebin web app written in rust using axum and askama.

- no database required: all pastes are stored in a local json file
- elegant ui: clean and minimal, easy-to-use design

## features

- create and share code/text pastes instantly
- view, track, and revisit your pastes
- no external database—just a simple `pastes.json` file

## getting started

### 1. clone the repo
```sh
git clone https://github.com/ni5arga/pastelpaste
cd pastelpaste
```

### 2. build and run
```sh
cargo run
```

### 3. open in your browser
```
http://127.0.0.1:8080/
```

## data storage

- all pastes are saved in a file called `pastes.json` in the project directory
- you do **not** need to add `pastes.json` to your repo. the file will be created automatically when you run the app and create your first paste

## libraries 

- [axum](https://github.com/tokio-rs/axum)
- [askama](https://github.com/djc/askama)
- [tokio](https://github.com/tokio-rs/tokio)
- [serde](https://github.com/serde-rs/serde)

## license

mit 
