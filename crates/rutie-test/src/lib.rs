#[macro_export]
macro_rules! test_ruby {
    ($code:expr) => {
        use rutie::VM;

        let code = format!(
            r#"
root = File.expand_path("../..", ENV["CARGO_MANIFEST_DIR"])

Dir.chdir(root)

class RbConfig
  CONFIG = {{
    "host_os" => ENV["TARGET"]
  }}
end

$LOAD_PATH.unshift(File.expand_path("lib", root))
$LOAD_PATH.unshift(File.expand_path("vendor/bundle/rutie/lib", root))

require "wasmer"

class AssertionError < RuntimeError
end

def assert &block
  raise AssertionError unless yield
end

{code}
"#,
            code = $code
        );

        VM::init();
        VM::init_loadpath();

        match VM::eval(&code) {
            Ok(value) => assert!(true),
            Err(err) => panic!("{:?}", err),
        }
    };
}

pub fn foo() {}
