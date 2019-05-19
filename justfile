# Compile and install the Ruby extension.
build:
	rake thermite:build

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
