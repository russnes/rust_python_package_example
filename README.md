# Rust Python Module Example
Example of how to make a python module out of a Rust library implementations, using the pyo3 library.

To make python package:
```shell
python3 setup.py sdist
```

To install as a python package you will need to have the Rust toolchain on the nightly
channel to compile the library for your architecture.

Install Rust as recommended by https://rustup.rs/.

It was required to have Rust on nightly when I did this (when rust nightly was at 1.35, and stable on 1.33) so if it still is enable nightly with
```shell
rustup default nightly
```

Also you will need to install the required python3 modules, but just do that with pip3 if you get an error about 'no module found'.

Then run setup.py to install the package

```shell
python3 setup.py install --user
```

Finally import rustpythonmoduleexample in a python script like a normal library, and run functions with
```shell
import rustpythonmoduleexample.rustpythonmoduleexample as rustpythonmoduleexample

rustpythonmoduleexample.init_lib()
rustpythonmoduleexample.print_info("Hello World from Rust!")
```

The Rust log library that I made needs to be initialized, so I made a separate function for that, but you of course won't need to have an init_lib() function in your program.

