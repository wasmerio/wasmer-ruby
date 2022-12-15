# Compile and install the Ruby extension.
build:
	rake build_lib
	bundle install
	# rake bundle_install

# Run all the tests.
test-all: test-lib test-doc test-example

# Run the tests of the library.
test-lib:
	rake test

# Run the tests of the documentation.
test-doc:
	cargo test --manifest-path crates/wasmer/Cargo.toml --doc

# Run the examples as tests.
test-example:
	for example in $(ls examples/*.rb); do \
		ruby $example; \
	done

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
