# gifos

"gifos" is an OS written in Rust by gifnksm.

* [Writing an OS in Rust](https://os.phil-opp.com)

## Requirements

* [QEMU]
* [cargo-xbuild], [bootimage]

    ```console
    $ cargo install cargo-xbuild bootimage
    ...
    ```

[QEMU]: https://www.qemu.org/
[cargo-xbuild]: https://crates.io/crates/cargo-xbuild/
[bootimage]: https://crates.io/crates/bootimage

## Operations

* Build

    ```console
    $ cargo xbuild
    ...
    ```

* Boot

    ```console
    $ cargo xrun
    ...
    ```

* Run unittest & integration test

    ```console
    $ cargo xtest
    ...
    ```
