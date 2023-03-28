# Daisogen's Rust toolchain
In order to build Daisogen using the distribution builder, you must compile and install the toolchain. This is a heavily automated process, but it takes a while (~15 minutes) and uses about 12GB of space, so have that in mind.

This is a fork of the Rust compiler, but instead of forking the whole project, the `run.sh` script patches a specific version of it. Run it, and, when it's finished, you should see the `dev-x86_64-unknown-daisogen` toolchain in the list you get when running `rustup toolchain list`.

Once that's done, you should be able to use with the distribution builder.
