## Build remarks
Borrowing can be debugged with the `debug_refcell` but you need rebuild
the standard library. It is possible only with `nightly`.

### Memo
* to see available targets use `rustc --print target-list`
* to see active toolchain use `rustup show` 
* to switch the channel use `rustup default nightly` or `rustup default stable`

### Windows
* install necessary toolchain `rustup toolchain install nightly-x86_64-pc-windows-msvc`.
* ~~install necessary toolchain `rustup component add rust-src --toolchain nightly-x86_64-pc-windows-msvc`.~~
* ~~re-compile std library `cargo +nightly r --target=x86_64-pc-windows-msvc
  -Zbuild-std -Zbuild-std-features=core/debug_refcell`~~
* compile your project the following way example:
`run --package <your_pakage> --features <your features> --target=x86_64-pc-windows-msvc -Zbuild-std -Zbuild-std-features=core/debug_refcell`
