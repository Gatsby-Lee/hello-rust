hello-cargo-sample project
=========================

Create Project
--------------

..code-block:: bash

  $ cargo new hello-cargo-sample
  Created binary (application) `hello-cargo-sample` package

  $ tree hello-cargo-sample/
  hello-cargo-sample/
  |-- Cargo.toml
  `-- src
      `-- main.rs
  1 directory, 2 files


Run sample
----------

..code-block:: bash

  $ cargo run
    Compiling hello-cargo-sample v0.1.0 (/home/web/hello-rust/hello-cargo-sample)
    Finished dev [unoptimized + debuginfo] target(s) in 0.97s
    Running `target/debug/hello-cargo-sample`
    Hello, world!

  $ tree hello-cargo-sample/
  hello-cargo-sample/
  |-- Cargo.lock
  |-- Cargo.toml
  |-- src
  |   `-- main.rs
  `-- target
      `-- debug
          |-- build
          |-- deps
          |   |-- hello_cargo_sample-6c167dffc3a543e0
          |   |-- hello_cargo_sample-6c167dffc3a543e0.d
          |   `-- hello_cargo_sample-6c167dffc3a543e0.dSYM
          |       `-- Contents
          |           |-- Info.plist
          |           `-- Resources
          |               `-- DWARF
          |                   `-- hello_cargo_sample-6c167dffc3a543e0
          |-- examples
          |-- hello-cargo-sample
          |-- hello-cargo-sample.d
          |-- hello-cargo-sample.dSYM -> deps/hello_cargo_sample-6c167dffc3a543e0.dSYM
          |-- incremental
          |   `-- hello_cargo_sample-1zkx98zq36jse
          |       |-- s-fdwr59sf2i-ew6eu3-2x805mx5316mi
          |       |   |-- 1u4nqu3phub93x6v.o
          |       |   |-- 2zkqy7htxi7vnu9j.o
          |       |   |-- 40ug5lfkuevesfe6.o
          |       |   |-- 4sqkhh947pu0xdzy.o
          |       |   |-- 4zey182l2avp0z44.o
          |       |   |-- dep-graph.bin
          |       |   |-- onig1zxt6havmp.o
          |       |   |-- query-cache.bin
          |       |   `-- work-products.bin
          |       `-- s-fdwr59sf2i-ew6eu3.lock
          `-- native

  15 directories, 19 files
