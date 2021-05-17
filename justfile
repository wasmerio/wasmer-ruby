# Compile and install the Ruby extension.
build:
	rake build_lib
	rake bundle_install

# Run the tests.
test:
	rake test

# Run the doctests.
doctest:
	cargo test --manifest-path crates/wasmer/Cargo.toml --doc

# Run all the tests.
test-all: test doctest

# Build the `.gem` file.
gem:
	rake build

# Generate the documentation.
doc:
	cargo rustdoc --manifest-path crates/wasmer/Cargo.toml -- --extend-css doc/patch-rustdoc.css

# Clean the project.
clean:
	cargo clean
	rake clean
	rake clobber


# Local Variables:
# mode: makefile
# End:
# vim: set ft=make :
