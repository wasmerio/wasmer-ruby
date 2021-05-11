# Compile and install the Ruby extension.
build:
	rake build_lib

# Run the tests.
test:
	rake test
	cargo test --manifest-path crates/wasmer/Cargo.toml --doc

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
