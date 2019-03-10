# Rust Python Module Example
Example of how to make a python module out of a Rust library implementations.

To make python package:
```shell
python3 setup.py sdist
```

To install as a python package you will need to have the Rust toolchain on the nightly
channel to compile the library for your architecture.

Install Rust as recommended by https://rustup.rs/ and enable nightly with
```shell
rustup default nightly
```

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
