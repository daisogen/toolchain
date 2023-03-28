# Daisogen's Rust toolchain
In order to build Daisogen using the distribution builder, you must compile and install the toolchain. This is a heavily automated process, but it takes a while (~15 minutes) and uses about 12GB of space, so have that in mind.

This is a fork of the Rust compiler, but instead of forking the whole project, the Makefile patches a specific version of it. Run it with `make`. When it's finished, you can run `make enable` and add the `dev-x86_64-unknown-daisogen` toolchain to the system (check with `rustup toolchain list`).

Once that's done, you should be able to use with the distribution builder.

Note: if you pull this repository, make sure to **delete** the `rust` directory, since it's very likely that the pulled patches cannot be applied to already modified files!
