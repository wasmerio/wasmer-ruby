# Compile and install the Rust library.
rust:
	rake build_lib

# Run the tests.
test:
	rake test

# Build the `.gem` file.
gem:
	rake build

# Clean the project.
clean:
	cargo clean
	rake clean
	rake clobber


# Local Variables:
# mode: makefile
# End:
# vim: set ft=make :
