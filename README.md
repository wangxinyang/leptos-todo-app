## Requirements

### 1. rust install

```
$ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```



### 2. trunk install

```
$ cargo install trunk
```



### 3. compile wasm source target install

```
$ rustup target add wasm32-unknown-unknown
```



### 4. wasm-bindgen-cli install

```
$ cargo install trunk wasm-bindgen-cli
```



### 5. cargo install

```
$ cargo install
```



### 6. startup

```
$ trunk serve --open
```

## Capture
![](https://github.com/wangxinyang/leptos-todo-app/blob/main/capture/todo.gif)

## License

This project is distributed under the terms of MIT.

See [LICENSE](LICENSE.md) for details.

Copyright 2024 Xinyang Wang
