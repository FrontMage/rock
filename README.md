# rock
Rust and webassembly based blog.

Backend is based on [Rocket](https://rocket.rs/) and `MongoDB`, frontend is based on [yew](https://github.com/DenisKolodin/yew).

The css framework is [spectre](https://github.com/picturepan2/spectre).

Why am I doing this?

- Just wanna try to use rust for both backend and frontend.

Does it work?

- Works fine.

Production?

- Not yet, due to the size of `.wasm` file. You can see it on the screenshot below.

### Build & Run

```sh
$ cd blog
$ cargo web build --release --target=wasm32-unknown-unknown 
$ cd ..
$ ROCKET_PORT=8080 cargo run
```

Visit `http://localhost:8080/blog.html`.

---

![screenshot](https://raw.githubusercontent.com/FrontMage/rock/master/screenshots/rock.png)

![screenshot](https://raw.githubusercontent.com/FrontMage/rock/master/screenshots/wasm.png)