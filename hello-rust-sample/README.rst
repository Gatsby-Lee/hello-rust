hello-rust-sample project
=========================

Create Project
--------------

..code-block:: bash

  $ cargo new hello-rust-sample
  Created binary (application) `hello-rust-sample` package

  $ tree hello-rust-sample/
  hello-rust-sample/
  |-- Cargo.toml
  `-- src
      `-- main.rs
  1 directory, 2 files


Run sample
----------

..code-block:: bash

  $ tree hello-rust-sample/
  hello-rust-sample/
  |-- Cargo.lock
  |-- Cargo.toml
  |-- src
  |   `-- main.rs
  `-- target
      `-- debug
          |-- build
          |-- deps
          |   |-- hello_rust_sample-6c167dffc3a543e0
          |   |-- hello_rust_sample-6c167dffc3a543e0.d
          |   `-- hello_rust_sample-6c167dffc3a543e0.dSYM
          |       `-- Contents
          |           |-- Info.plist
          |           `-- Resources
          |               `-- DWARF
          |                   `-- hello_rust_sample-6c167dffc3a543e0
          |-- examples
          |-- hello-rust-sample
          |-- hello-rust-sample.d
          |-- hello-rust-sample.dSYM -> deps/hello_rust_sample-6c167dffc3a543e0.dSYM
          |-- incremental
          |   `-- hello_rust_sample-1zkx98zq36jse
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
