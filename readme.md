## About

This is a sample [Zellij](https://github.com/zellij-org/zellij) which will print out the current time to the pane every second.

You can learn more about developing plugins in the [Zellij Documentation](https://zellij.dev/documentation/plugins.html).

Make sure you have the `wasm32-wasi` rust target installed.

```sh
rustup target add wasm32-wasi
```

### Build with `cargo` and Test in Zellij

```sh
# If you don't have Zellij installed already
cargo install zellij
# Building the plugin
cargo build
# Running in Zellij
zellij -l plugin.yaml
```
