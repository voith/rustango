# Rustango

**Rustango** is a minimalist web framework written in pure Rust, inspired by Django’s class-based views and Python’s expressiveness — but built the Rust way.

This is my **first Rust project**, and I'm learning the language by building everything from scratch: TCP handling, HTTP parsing, dynamic dispatch, and routing. It's part experiment, part playground, and part education — but it's already capable of handling basic web requests cleanly and elegantly.

---

## ✨ Features

- Built on top of `std::net::TcpListener` and `httparse`
- Trait-based views like Django (`get`, `post`, etc.)
- Dynamic route registration via a central `urls.rs`
- Auto-fallback for unsupported methods (`405`)
- Auto-fallback for unknown paths (`404`)
- Fully synchronous (for now)

---

## 🚀 Getting Started

-  Clone & Run

```bash
git clone https://github.com/voith/rustango.git
cd rustango/app
cargo run
```

- Then Visit 
```text
http://127.0.0.1:<PORT>
The port will be mentioned in the terminal
```

---
## ✍️ Example View
```rust
use rustango::server::{Response, View};

pub struct HomePageView;

impl View for HomePageView {
    fn get(&self) -> Response {
        Response {
            data: "<h1>Hello, Rustango!</h1>".to_string(),
            status_code: 200,
            version: "1.1",
        }
    }
}
```

---

## 🛣️ Registering Routes
In urls.rs:
```rust
use std::collections::HashMap;

use <YOUR_CREATE>::views::{HomePageView, SomePageView}; // assuming you have created these views in your crate
use rustango::views::BoxedView;

pub fn get_routes() -> HashMap<String, BoxedView> {
    let mut map: HashMap<String, BoxedView> = HashMap::new();
    map.insert("/".to_string(), Box::new(HomePageView));
    map.insert("/somepage".to_string(), Box::new(SomePageView));
    map
}
```
## Starting the sever
In main.rs:
```rust
use rustango::server::{Server, ServerConfig};
use std::sync::Arc;
use <YOUR_CRATE>::urls::get_routes;

fn main() {
    let config = ServerConfig {
        routes: get_routes(),
        port: Some(8080),
        pool_size: Some(4)
    };
    let server = Arc::new(Server::new(config));
    server.start();
}
```

---

## 📚 Learning Goals
- Understand ownership, borrowing, and lifetimes

- Build abstractions with traits and dynamic dispatch

- Get comfortable with low-level systems programming

- Explore zero-copy parsing with httparse

- Learn by doing, debugging, and improving iteratively 

---

## ⚠️ Disclaimer
This is a personal learning project, don't use this in production.
