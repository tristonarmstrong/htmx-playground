# Hot Reload Axum

I got tired of restarting my `shuttle run` server pretty fast. I needed a solution.

Dug around a bit and found [tower_livereload](https://github.com/leotaku/tower-livereload). It was actually mentioned in the [shuttle](https://www.shuttle.rs/) docs [here](https://docs.shuttle.rs/getting-started/local-run#live-reload-frontend-with-tower-livereload).

That isn't enought tho. You actually need one more piece. A more important piece i might add.
[cargo watch](https://crates.io/crates/cargo-watch)

This crate will auto reload your server on save, combined with `tower livereload` will also refresh the page, so you dont have to do anythign but wait, really.

## Show me the important stuff

### Install cargo watch

```bash
cargo install cargo-watch
```

### Add Tower livereload

```bash
cargo add tower-livereload
```

### Inject the necessary code in your project

```rust
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app = Router::new()
        .route() // some route
        .layer(tower_livereload::LiveReloadLayer::new()); // <--- add here

    // --- ignore all of this ---v
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await?;
    axum::serve(listener, app).await?;

    Ok(())
}
```

### Run your shuttle server in watch mode

```bash
# This will e(x)ecute `cargo shuttle run` when you save a file.
cargo watch -x 'shuttle run'
# This will also (q)uietly (c)lear the console between runs.
cargo watch -qcx 'shuttle run'

# There are many other helpful options, see `cargo watch --help`
```
