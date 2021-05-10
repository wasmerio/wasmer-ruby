use crate::{
    error::{to_ruby_err, RuntimeError},
    prelude::*,
};
use rutie::{Encoding, RString};

#[rubyfunction]
pub fn wat2wasm(wat: &RString) -> RubyResult<RString> {
    wat::parse_str(wat.to_str())
        .map(|bytes| RString::from_bytes(bytes.as_slice(), &Encoding::us_ascii()))
        .map_err(to_ruby_err::<RuntimeError, _>)
}

#[rubyfunction]
pub fn wasm2wat(bytes: &RString) -> RubyResult<RString> {
    Ok(RString::new_utf8(
        &wasmprinter::print_bytes(bytes.to_bytes_unchecked())
            .map_err(to_ruby_err::<RuntimeError, _>)?,
    ))
}
