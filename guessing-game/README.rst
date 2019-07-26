Guessing Game
=============

Add dependencies into Cargo.toml
--------------------------------

.. code-block:: cfg

    [dependencies]

    rand = "0.3.14"


.. code-block:: bash

    $ cargo build
        Updating crates.io index
    Downloaded rand v0.3.23
    Downloaded libc v0.2.59
    Downloaded rand v0.4.6
    Compiling libc v0.2.59
    Compiling rand v0.4.6
    Compiling rand v0.3.23
    Compiling guessing-game v0.1.0 (/home/web/hello-rust/guessing-game)
        Finished dev [unoptimized + debuginfo] target(s) in 55.73s

