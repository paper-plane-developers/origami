# Origami

[Paper Plane](https://github.com/paper-plane-developers/paper-plane) related set of gtk widgets that can be usable outside of it.

# Add to project
You should already have [libadwaita](https://crates.io/crates/libadwaita) and [gtk4](https://crates.io/crates/gtk4) in your project.

Add with command
```
cargo add --git=https://github.com/yuraiz/origami origami --rename ori
```
Line in Cargo.toml
```
ori = { git = "https://github.com/yuraiz/origami", version = "0.1.0", package = "origami" }
```

### ori::init()
`ori::init()` calls `static_type` for every exported widget, so it makes easier to use them in templates .

# Demo

Run the demo
```
cargo run
```
Open a page directly
```
cargo run [page-name]
```

# Dependencies
That library will use latest stable versions of gtk and libadwaita.

Building demo requires `blueprint-compiler`.
