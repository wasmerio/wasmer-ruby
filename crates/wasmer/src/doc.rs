macro_rules! x {
    () => {
        unimplemented!("This code exists only for documentation purposes");
    };
}

/// Declare Ruby native types. It's only for documentation purposes.
pub mod Ruby {
    /// A native Ruby boolean.
    ///
    /// # Example
    ///
    /// ```ruby
    /// true
    /// ```
    pub struct Boolean;

    /// A native Ruby integer.
    ///
    /// # Example
    ///
    /// ```ruby
    /// 42
    /// ```
    pub struct Integer;

    /// A native Ruby float.
    ///
    /// # Example
    ///
    /// ```ruby
    /// 4.2
    /// ```
    pub struct Float;

    /// A native Ruby string.
    ///
    /// # Example
    ///
    /// ```ruby
    /// "hello"
    /// ```
    pub struct String;

    /// A native Ruby array.
    ///
    /// # Example
    ///
    /// ```ruby
    /// [1, "two", 3.0]
    /// ```
    pub struct Array<T>;

    /// A native Ruby hash.
    ///
    /// # Example
    ///
    /// ```ruby
    /// {"foo": 42, "bar": 153}
    /// ```
    pub struct Hash<K, V>;

    /// Represents any kind of object.
    pub struct Any;
}

/// The `Wasmer` module provides the entire Wasmer API to manipulate
/// the WebAssembly runtime.
pub mod Wasmer {
    use crate::doc::Ruby::*;

    /// A WebAssembly type.
    ///
    /// # Example
    ///
    /// ```ruby
    /// Type::I32
    /// ```
    #[allow(non_camel_case_types)]
    pub enum Type {
        I32,
        I64,
        F32,
        F64,
        V128,
        EXTERN_REF,
        FUNC_REF,
    }

    /// Represents the signature of a function that is either
    /// implemented in WebAssembly module or exposed to WebAssembly by
    /// the host.
    ///
    /// WebAssembly functions can have 0 or more parameters and results.
    pub struct FunctionType;

    impl FunctionType {
        /// Creates a new `FunctionType`.
        ///
        /// # Example
        ///
        /// ```ruby
        /// function_type = FunctionType.new [Type::I32, Type::I64], [Type::I32]
        /// ```
        pub fn new(params: Array<Type>, results: Array<Type>) -> Self {
            x!()
        }

        /// Returns the parameters.
        pub fn params(&self) -> Array<Type> {
            x!()
        }

        /// Returns the results.
        pub fn results(&self) -> Array<Type> {
            x!()
        }
    }

    /// A descriptor for a WebAssembly memory type.
    ///
    /// Memories are described in units of pages (64Kb) and represent
    /// contiguous chunks of addressable memory.
    pub struct MemoryType;

    impl MemoryType {
        /// Creates a new `MemoryType`.
        ///
        /// # Example
        ///
        /// ```ruby
        /// memory_type = MemoryType.new 1, 3, true
        /// ```
        pub fn new(minimum: Integer, maximum: Option<Integer>, shared: Boolean) -> Self {
            x!()
        }

        /// Returns the minimum size of the memory.
        pub fn minimum(&self) -> Integer {
            x!()
        }

        /// Returns the maximum size of the memory of any.
        pub fn maximum(&self) -> Option<Integer> {
            x!()
        }

        /// Returns whether the memory is shared or not.
        pub fn shared(&self) -> Integer {
            x!()
        }
    }

    /// A descriptor for a WebAssembly global.
    pub struct GlobalType;

    impl GlobalType {
        /// Creates a new `GlobalType`.
        ///
        /// # Example
        ///
        /// ```ruby
        /// global_type = GlobalType.new Type::I32, true
        /// ```
        pub fn new(r#type: Type, mutable: Boolean) -> Self {
            x!()
        }

        /// Returns the type of the global.
        pub fn r#type(&self) -> Type {
            x!()
        }

        /// Returns whether the global is mutable.
        pub fn mutable(&self) -> Boolean {
            x!()
        }
    }

    /// A descriptor for a table in a WebAssembly module.
    ///
    /// Tables are contiguous chunks of a specific element, typically
    /// a funcref or externref. The most common use for tables is a
    /// function table through which call_indirect can invoke other
    /// functions.
    pub struct TableType;

    impl TableType {
        /// Creates a new `TableType`.
        ///
        /// # Example
        ///
        /// ```ruby
        /// table_type = TableType.new Type::I32, 7, 42
        /// ```
        pub fn new(r#type: Type, minimum: Integer, maximum: Option<Integer>) -> Self {
            x!()
        }

        /// Returns the type of table.
        pub fn r#type(&self) -> Type {
            x!()
        }

        /// Returns the minimum size of the table.
        pub fn minimum(&self) -> Integer {
            x!()
        }

        /// Returns the maximum size of the table if any.
        pub fn maximum(&self) -> Option<Integer> {
            x!()
        }
    }

    /// Represents the type of a module's export (not to be confused
    /// with an export of an instance). It is usually built from the
    /// [`Module::exports`] getter.
    pub struct ExportType;

    impl ExportType {
        /// Creates a new `ExportType`.
        ///
        /// `type` must be of type [`FunctionType`], [`MemoryType`],
        /// [`GlobalType`] or [`TableType`].
        ///
        /// # Example
        ///
        /// ```ruby
        /// export_type = ExportType.new "foo", Function.new([Type::I32], [])
        /// ```
        pub fn new(name: String, r#type: Any) -> Self {
            x!()
        }

        /// Returns the name of the export.
        pub fn name(&self) -> String {
            x!()
        }

        /// Returns the type of the export.
        pub fn r#type(&self) -> Type {
            x!()
        }
    }

    /// Represents the type of a module's import. It is usually built
    /// from the [`Module::imports`] getter.
    pub struct ImportType;

    impl ImportType {
        /// Creates a new `ImportType`.
        ///
        /// `type` must be of type [`FunctionType`], [`MemoryType`],
        /// [`GlobalType`] or [`TableType`].
        ///
        /// # Example
        ///
        /// ```ruby
        /// import_type = ImportType.new "foo", "bar", Function.new([Type::I32], [])
        /// ```
        pub fn new(module: String, name: String, r#type: Any) -> Self {
            x!()
        }

        /// Returns the module's name of the import.
        pub fn module(&self) -> String {
            x!()
        }

        /// Returns the name of the import.
        pub fn name(&self) -> String {
            x!()
        }

        /// Returns the type of the import.
        pub fn r#type(&self) -> Type {
            x!()
        }
    }

    /// The store represents all global state that can be manipulated
    /// by WebAssembly programs. It consists of the runtime
    /// representation of all instances of functions, tables,
    /// memories, and globals that have been allocated during the
    /// lifetime of the abstract machine.
    ///
    /// The `Store` holds the engine (that is —amongst many things— used
    /// to compile the WebAssembly bytes into a valid module
    /// artifact), in addition to the Tunables (that are used to
    /// create the memories, tables and globals). For the moment, it's
    /// not possible to tweak the engines and the compilers.
    ///
    /// Specification: <https://webassembly.github.io/spec/core/exec/runtime.html#store>
    ///
    /// # Example
    ///
    /// Use the store with the default engine and the default compiler:
    ///
    /// ```ruby
    /// store = Store.new
    /// ```
    pub struct Store;

    impl Store {
        /// Creates a new `Store`.
        ///
        /// # Example
        ///
        /// ```ruby
        /// store = Store.new
        /// ```
        pub fn new() -> Self {
            x!()
        }
    }

    /// A WebAssembly module contains stateless WebAssembly code that has
    /// already been compiled and can be instantiated multiple times.
    ///
    /// Creates a new WebAssembly `Module` given the configuration in the
    /// store.
    ///
    /// If the provided bytes are not WebAssembly-like (start with
    /// b"\0asm"), this function will try to to convert the bytes assuming
    /// they correspond to the WebAssembly text format.
    ///
    /// # Security
    ///
    /// Before the code is compiled, it will be validated using the store
    /// features.
    ///
    /// # Example
    ///
    /// ```ruby
    /// store = Store.new
    ///
    /// # Let's compile WebAssembly from bytes.
    /// module_ = Module.new store, wasm_bytes
    ///
    /// # Let's compile WebAssembly from WAT.
    /// module_ = Module.new store, "(module)"
    /// ```
    pub struct Module;

    impl Module {
        /// Validates a new WebAssembly Module given the configuration
        /// in the [`Store`].
        ///
        /// This validation is normally pretty fast and checks the
        /// enabled WebAssembly features in the Store engine to assure
        /// deterministic validation of the `Module`.
        ///
        /// # Example
        ///
        /// ```ruby
        /// Module.validate Store.new, wasm_bytes
        /// ```
        pub fn validate(store: Store, bytes: String) -> Boolean {
            x!()
        }

        /// Creates a new [`Module`].
        pub fn new(store: Store, bytes: String) -> Self {
            x!()
        }

        /// Get or set the current name of the module.
        ///
        /// This name is normally set in the WebAssembly bytecode by
        /// some compilers, but can be also overwritten.
        ///
        /// Not all modules have a name.
        ///
        /// # Example
        ///
        /// ```ruby
        /// store = Store.new
        ///
        /// # Module with an existing name.
        /// assert Module.new(store, "(module $moduleName)").name == "moduleName"
        ///
        /// # Module with no name.
        /// assert Module.new(store, "(module)").name.is_nil?
        ///
        /// # Change the module's name.
        /// module_ = Module.new store, "(module $moduleName)"
        /// module.name = "hello"
        /// assert module.name == "hello"
        /// ```
        pub fn name(&self, name: String) {
            x!()
        }

        /// Returns a list of [`ExportType`] objects, which represents
        /// all the exports of this module.
        ///
        /// The order of the exports is guaranteed to be the same as
        /// in the WebAssembly bytecode.
        ///
        /// # Example
        ///
        /// See the [`ExportType`] class to learn more.
        pub fn exports(&self) -> Array<ExportType> {
            x!()
        }

        /// Returns a list of [`ImportType`] objects, which represents
        /// all the imports of this module.
        ///
        /// The order of the imports is guaranteed to be the same as
        /// in the WebAssembly bytecode.
        ///
        /// # Example
        ///
        /// See the [`ImportType`] class to learn more.
        pub fn imports(&self) -> Array<ImportType> {
            x!()
        }

        /// Get the custom sections of the module given a `name`.
        ///
        /// # Important
        ///
        /// Following the WebAssembly specification, one name can have
        /// multiple custom sections. That's why a list of bytes is
        /// returned rather than bytes.
        ///
        /// Consequently, the empty list represents the absence of a
        /// custom section for the given name.
        ///
        /// # Example
        ///
        /// ```ruby
        /// bytes = IO.read "custom_sections.wasm", mode: "rb"
        /// module_ = Module.new Store.new, bytes
        ///
        /// assert module_.custom_sections("easter_egg") == ["Wasmer"]
        /// assert module_.custom_sections("hello") == ["World!"]
        /// assert module_.custom_sections("foo") == []
        /// ```
        pub fn custom_sections(&self, name: String) -> Array<String> {
            x!()
        }

        /// Serializes a module into a binary representation that the
        /// engine can later process via [`Module::deserialize`].
        ///
        /// # Example
        ///
        /// ```ruby
        /// module_ = Module.new Store.new, "(module)"
        /// assert_kind_of String, module_.serialize
        /// ```
        pub fn serialize(&self) -> String {
            x!()
        }

        /// Deserializes a serialized module binary into a Module.
        ///
        /// **Note**: the module has to be serialized before with the
        /// serialize method.
        ///
        /// # Safety
        ///
        /// This function is inherently unsafe as the provided bytes:
        ///
        /// 1. Are going to be deserialized directly into Rust objects.
        /// 2. Contains the function assembly bodies and, if
        ///    intercepted, a malicious actor could inject code into
        ///    executable memory.
        ///
        /// And as such, the deserialize method is unsafe.
        ///
        /// # Example
        ///
        /// ```ruby
        /// store = Store.new
        ///
        /// serialized_module = Module.new(
        ///   store,
        ///   (<<~WAST)
        ///   (module
        ///     (func (export "function") (param i32 i64)))
        ///   WAST
        /// ).serialize
        ///
        /// module_ = Module.deserialize store, serialized_module
        /// serialized_module = nil
        ///
        /// exports = module_.exports
        ///
        /// assert exports.length() == 1
        /// assert exports[0].name == "function"
        /// assert_kind_of FunctionType, exports[0].type
        /// assert exports[0].type.params == [Type::I32, Type::I64]
        /// assert exports[0].type.results == []
        /// ```
        pub fn deserialize(bytes: String) -> Self {
            x!()
        }
    }

    /// A WebAssembly instance is a stateful, executable instance of a
    /// WebAssembly [`Module`].
    ///
    /// Instance objects contain all the exported WebAssembly
    /// functions, memories, tables and globals that allow interacting
    /// with WebAssembly.
    ///
    /// Specification:
    /// <https://webassembly.github.io/spec/core/exec/runtime.html#module-instances>
    ///
    /// # Examples
    ///
    /// Example without an import object. The following creates a
    /// module with a sum exported function that sum two integers.
    ///
    /// ```ruby
    /// module_ = Module.new(
    ///   Store.new,
    ///   (<<~WAST)
    ///   (module
    ///     (type (func (param i32 i32) (result i32)))
    ///     (func (type 0)
    ///       local.get 0
    ///       local.get 1
    ///       i32.add)
    ///     (export "sum" (func 0)))
    ///   WAST
    /// )
    /// instance = Instance.new module_, nil
    ///
    /// assert instance.exports.sum.(1, 2) == 3
    /// ```
    ///
    /// Example with an import object. The following creates a module
    /// that (i) imports a sum function from the `math` namespace, and
    /// (ii) exports an `add_one` function that adds 1 to any given
    /// integer (by using the `math.sum` function).
    ///
    /// ```ruby
    /// def sum(x, y)
    ///   x + y
    /// end
    ///
    /// store = Store.new
    /// module_ = Module.new(
    ///   store,
    ///   (<<~WAST)
    ///   (module
    ///     (import "math" "sum" (func $sum (param i32 i32) (result i32)))
    ///     (func (export "add_one") (param i32) (result i32)
    ///       local.get 0
    ///       i32.const 1
    ///       call $sum))
    ///   WAST
    /// )
    ///
    /// import_object = ImportObject.new
    /// import_object.register(
    ///   "math",
    ///   {
    ///     :sum => Function.new(store, method(:sum), FunctionType.new([Type::I32, Type::I32], [Type::I32]))
    ///   }
    /// )
    ///
    /// instance = Instance.new module_, import_object
    ///
    /// assert instance.exports.add_one.(1) == 2
    /// ```
    pub struct Instance;

    impl Instance {
        /// Creates a new `Instance`.
        pub fn new(module: Module, import_object: ImportObject) -> Self {
            x!()
        }

        /// Returns all the exported entities.
        pub fn exports(&self) -> Exports {
            x!()
        }
    }

    /// Represents all the exports of an instance. It is built by [`Instance::exports`].
    ///
    /// Exports can be of kind [`Function`], [`Global`], [`Table`], or [`Memory`].
    pub struct Exports;

    impl Exports {
        /// Returns the number of exports.
        pub fn length(&self) -> Integer {
            x!()
        }

        /// Returns either a [`Function`], a [`Memory`], a [`Global`],
        /// or a [`Table`] if the name for the export exists.
        ///
        /// # Example
        ///
        /// ```ruby
        /// module_ = Module.new(
        ///   Store.new,
        ///   (<<~WAST)
        ///   (module
        ///     (func (export "func") (param i32 i64))
        ///     (global (export "glob") i32 (i32.const 7))
        ///     (table (export "tab") 0 funcref)
        ///     (memory (export "mem") 1))
        ///   WAST
        /// )
        /// instance = Instance.new module_, nil
        /// exports = instance.exports
        ///
        /// assert exports.respond_to? :func
        /// assert exports.respond_to? :glob
        /// assert exports.respond_to? :tab
        /// assert exports.respond_to? :mem
        /// assert not(exports.respond_to? :foo)
        ///
        /// assert_kind_of Function, exports.func
        /// assert_kind_of Memory, exports.mem
        /// assert_kind_of Global, exports.glob
        /// assert_kind_of Table, exports.tab
        /// ```
        pub fn method_missing(name: String) -> Any {
            x!()
        }
    }

    /// An `ImportObject` represents all of the import data used when
    /// instantiating a WebAssembly module.
    ///
    /// # Example
    ///
    /// Importing a function, `math.sum`, and call it through the
    /// exported `add_one` function:
    ///
    /// ```ruby
    /// def sum(x, y)
    ///   x + y
    /// end
    ///
    /// store = Store.new
    /// module_ = Module.new(
    ///   store,
    ///   (<<~WAST)
    ///   (module
    ///     (import "math" "sum" (func $sum (param i32 i32) (result i32)))
    ///     (func (export "add_one") (param i32) (result i32)
    ///       local.get 0
    ///       i32.const 1
    ///       call $sum))
    ///   WAST
    /// )
    ///
    /// import_object = ImportObject.new
    /// import_object.register(
    ///   "math",
    ///   {
    ///     :sum => Function.new(store, method(:sum), FunctionType.new([Type::I32, Type::I32], [Type::I32]))
    ///   }
    /// )
    ///
    /// instance = Instance.new module_, import_object
    ///
    /// assert instance.exports.add_one.(1) == 2
    /// ```
    ///
    /// Importing a memory:
    ///
    /// ```ruby
    /// store = Store.new
    /// module_ = Module.new(
    ///   store,
    ///   (<<~WAST)
    ///   (module
    ///     (import "env" "memory" (memory $memory 1))
    ///     (func (export "increment")
    ///       i32.const 0
    ///       i32.const 0
    ///       i32.load    ;; load 0
    ///       i32.const 1
    ///       i32.add     ;; add 1
    ///       i32.store   ;; store at 0
    ///       ))
    ///   WAST
    /// )
    ///
    /// memory = Memory.new store, MemoryType.new(1, nil, false)
    /// view = memory.uint8_view(0)
    ///
    /// import_object = ImportObject.new
    /// import_object.register(
    ///   "env",
    ///   {
    ///     :memory => memory,
    ///   }
    /// )
    ///
    /// instance = Instance.new module_, import_object
    ///
    /// assert view[0] == 0
    ///
    /// instance.exports.increment.()
    /// assert view[0] == 1
    ///
    /// instance.exports.increment.()
    /// assert view[0] == 2
    /// ```
    ///
    /// Importing a global:
    ///
    /// ```ruby
    /// store = Store.new
    /// module_ = Module.new(
    ///   store,
    ///   (<<~WAST)
    ///     (module
    ///       (import "env" "global" (global $global (mut i32)))
    ///       (func (export "read_g") (result i32)
    ///         global.get $global)
    ///       (func (export "write_g") (param i32)
    ///         local.get 0
    ///         global.set $global))
    ///   WAST
    /// )
    ///
    /// global = Global.new store, Value.i32(7), true
    ///
    /// import_object = ImportObject.new
    /// import_object.register(
    ///   "env",
    ///   {
    ///     :global => global
    ///   }
    /// )
    ///
    /// instance = Instance.new module_, import_object
    ///
    /// assert instance.exports.read_g.() == 7
    ///
    /// global.value = 153
    /// assert instance.exports.read_g.() == 153
    ///
    /// instance.exports.write_g.(11)
    /// assert global.value == 11
    /// ```
    ///
    /// etc.
    pub struct ImportObject;

    impl ImportObject {
        /// Creates a new `ImportObject`.
        pub fn new() -> Self {
            x!()
        }

        /// Checks whether the import object contains a specific
        /// namespace.
        pub fn contains_namespace(&self, namespace_name: String) -> Boolean {
            x!()
        }

        /// Registers a set of [`Function`], [`Memory`], [`Global`] or
        /// [`Table`] to a particular namespace.
        ///
        /// See the [`ImportObject`]'s documentation to see more
        /// examples.
        pub fn register(&self, namespace_name: String, namespace: Hash<String, Any>) {
            x!()
        }
    }

    /// Represents a WebAssembly function instance.
    ///
    /// A function instance is the runtime representation of a
    /// function. It effectively is a closure of the original function
    /// (defined in either the host or the WebAssembly module) over
    /// the runtime [`Instance`] of its originating [`Module`].
    ///
    /// The module instance is used to resolve references to other
    /// definitions during executing of the function.
    ///
    /// Specification:
    /// <https://webassembly.github.io/spec/core/exec/runtime.html#function-instances>
    ///
    /// Note that the function can be invoked/called by the host only
    /// when it is an exported function (see [`Exports`] to see an
    /// example).
    ///
    /// # Example
    ///
    /// To build a `Function`, we need its type. `Function`
    /// understands `Symbol`, `Proc`, and `Lambda`.
    ///
    /// First, a function through a symbol:
    ///
    /// ```ruby
    /// def foo(x)
    ///   x + 1
    /// end
    ///
    /// function = Function.new Store.new, method(:foo), FunctionType.new([Type::I32], [])
    /// ```
    ///
    /// Second, a function through a proc as a lambda:
    ///
    /// ```ruby
    /// function = Function.new Store.new, -> (x) { x + 1 }, FunctionType.new([Type::I32], [])
    /// ```
    ///
    /// Third, a function through a proc:
    ///
    /// ```ruby
    /// function = Function.new Store.new, Proc.new { |x| x + 1 }, FunctionType.new([Type::I32], [])
    /// ```
    pub struct Function;

    impl Function {
        /// Creates a new `Function`. The `function` can be of kind
        /// `Symbol`, `Proc` or `Lambda`.
        pub fn new(store: Store, function: Any, function_type: FunctionType) -> Self {
            x!()
        }

        /// Calls the function with arguments. It returns zero or more results.
        pub fn call(x0: Any, x1: Any, x2: Any, etc: Any) -> Any {
            x!()
        }

        /// Returns the function type.
        pub fn r#type(&self) -> FunctionType {
            x!()
        }
    }

    /// A WebAssembly memory instance.
    ///
    /// A memory instance is the runtime representation of a linear
    /// memory. It consists of a vector of bytes and an optional
    /// maximum size.
    ///
    /// The length of the vector always is a multiple of the
    /// WebAssembly page size, which is defined to be the constant
    /// 65536 – abbreviated 64Ki. Like in a memory type, the maximum
    /// size in a memory instance is given in units of this page size.
    ///
    /// A memory created by the host or in WebAssembly code will be
    /// accessible and mutable from both host and WebAssembly.
    ///
    /// Specification: <https://webassembly.github.io/spec/core/exec/runtime.html#memory-instances>
    ///
    /// # Example
    ///
    /// Creates a [`Memory`] from scratch:
    ///
    /// ```ruby
    /// store = Store.new
    /// memory_type = MemoryType.new 3, 10, true
    /// memory = Memory.new store, memory_type
    ///
    /// assert memory.size == 3
    /// ```
    ///
    /// Gets a memory from the exports of an instance:
    ///
    /// ```ruby
    /// module_ = Module.new Store.new, wasm_bytes
    /// instance = Instance.new module, nil
    ///
    /// memory = instance.exports.memory
    /// ```
    pub struct Memory;

    impl Memory {
        /// Creates a new `Memory`.
        pub fn new(store: Store, memory_type: MemoryType) -> Self {
            x!()
        }

        /// Returns the memory type.
        pub fn r#type(&self) -> MemoryType {
            x!()
        }

        /// Returns the size (in pages) of the memory.
        pub fn size(&self) -> Integer {
            x!()
        }

        /// Returns the size (in bytes) of the memory.
        pub fn data_size(&self) -> Integer {
            x!()
        }

        /// Grows memory by the specified amount of WebAssembly pages.
        ///
        /// # Example
        ///
        /// ```ruby
        /// memory = instance.exports.memory
        /// old_memory_size = memory.data_size
        ///
        /// memory.grow 1
        ///
        /// memory_size = memory.data_size
        ///
        /// assert memory_size == 1179648
        /// assert memory_size - old_memory_size == 65536
        /// ```
        pub fn grow(&self, number_of_pages: Integer) -> Integer {
            x!()
        }

        /// Creates a read-and-write view over the memory data where
        /// elements are of kind `uint8`.
        pub fn uint8_view(&self) -> Uint8View {
            x!()
        }

        /// Creates a read-and-write view over the memory data where
        /// elements are of kind `int8`.
        pub fn int8_view(&self) -> Int8View {
            x!()
        }

        /// Creates a read-and-write view over the memory data where
        /// elements are of kind `uint16`.
        pub fn uint16_view(&self) -> Uint16View {
            x!()
        }

        /// Creates a read-and-write view over the memory data where
        /// elements are of kind `int16`.
        pub fn int16_view(&self) -> Int16View {
            x!()
        }

        /// Creates a read-and-write view over the memory data where
        /// elements are of kind `uint32`.
        pub fn uint32_view(&self) -> Uint32View {
            x!()
        }

        /// Creates a read-and-write view over the memory data where
        /// elements are of kind `int32`.
        pub fn int32_view(&self) -> Int32View {
            x!()
        }
    }

    pub struct Uint8View;
    pub struct Int8View;
    pub struct Uint16View;
    pub struct Int16View;
    pub struct Uint32View;
    pub struct Int32View;

    /// Represents a WebAssembly global instance.
    ///
    /// A global instance is the runtime representation of a global
    /// variable. It consists of an individual value and a flag
    /// indicating whether it is mutable.
    ///
    /// Specification: <https://webassembly.github.io/spec/core/exec/runtime.html#global-instances>
    ///
    /// # Example
    ///
    /// ```ruby
    /// store = Store.new
    /// global = Global.new store, Value.i32(42), false
    ///
    /// assert global.value == 42
    ///
    /// type = global.type
    ///
    /// assert type.type == Type::I32
    /// assert type.mutable? == false
    /// ```
    pub struct Global;

    impl Global {
        /// Creates a new `Global`.
        pub fn new(store: Store, value: Value, mutable: Boolean) -> Self {
            x!()
        }

        /// Returns whether the global is muable.
        pub fn mutable(&self) -> Boolean {
            x!()
        }

        /// Get or set a new value to the global if mutable.
        ///
        /// # Example
        ///
        /// ```ruby
        /// store = Store.new
        /// global = Global.new store, Value.i32(42), true
        ///
        /// assert global.value == 42
        ///
        /// type = global.type
        ///
        /// assert type.type == Type::I32
        /// assert type.mutable? == true
        ///
        /// global.value = 153
        ///
        /// assert global.value == 153
        /// ```
        pub fn value(&self, value: Any) -> Any {
            x!()
        }

        /// Returns the global type.
        pub fn r#type(&self) -> GlobalType {
            x!()
        }
    }

    /// A WebAssembly table instance.
    ///
    /// The Table class is an array-like structure representing a
    /// WebAssembly table, which stores function references.
    ///
    /// A table created by the host or in WebAssembly code will be
    /// accessible and mutable from both host and WebAssembly.
    ///
    /// Specification: <https://webassembly.github.io/spec/core/exec/runtime.html#table-instances>
    pub struct Table;

    impl Table {
        /// Creates a new `Table`.
        pub fn new(store: Store, table_type: TableType, initia_value: Value) -> Self {
            x!()
        }
    }

    /// Represents a WebAssembly value of a specific type.
    ///
    /// Most of the time, the types for WebAssembly values will be
    /// inferred. When it's not possible, the `Value` class is
    /// necessary.
    pub struct Value;

    impl Value {
        /// Creates a new `Value` containing a `int32`.
        pub fn i32(value: Integer) -> Self {
            x!()
        }

        /// Creates a new `Value` containing a `int64`.
        pub fn i64(value: Integer) -> Self {
            x!()
        }

        /// Creates a new `Value` containing a `float32`.
        pub fn f32(value: Float) -> Self {
            x!()
        }

        /// Creates a new `Value` containing a `float64`.
        pub fn f64(value: Float) -> Self {
            x!()
        }
    }

    /// Wasmer's [WASI] implementation.
    ///
    /// From the user perspective, WASI is a bunch of imports. To
    /// generate the appropriated imports, you can use
    /// [`StateBuilder`](Wasi::StateBuilder) to build an
    /// [`Environment`](Wasi::Environment). This environment holds the
    /// WASI memory, and can be used to generate a valid
    /// [`ImportObject`]. This last one can be passed to [`Instance`]
    /// to instantiate a [`Module`] that needs WASI support.
    ///
    /// [WASI]: https://github.com/WebAssembly/WASI
    ///
    /// # Example
    ///
    /// ```ruby
    /// store = Store.new
    /// module_ = Module.new store, bytes
    ///
    /// # Get the WASI version.
    /// wasi_version = Wasi::get_version module_, true
    ///
    /// # Build a WASI environment for the imports.
    /// wasi_env = Wasi::StateBuilder.new("test-program")
    ///              .argument("--foo")
    ///              .environments({"ABC" => "DEF", "X" => "YZ"})
    ///              .map_directory("the_host_directory", ".")
    ///              .finalize
    ///
    /// # Generate an `ImportObject` for the WASI environment.
    /// import_object = wasi_env.generate_import_object store, wasi_version
    ///
    /// # Now we are ready to instantiate the module.
    /// instance = Instance.new module_, import_object
    ///
    /// # Here we go, let's start the program.
    /// instance.exports._start.()
    /// ```
    pub mod Wasi {
        use super::*;
        use crate::doc::Ruby::*;

        /// Represents a WASI version.
        #[allow(non_camel_case_types)]
        pub enum Version {
            LATEST_VERSION,
            SNAPSHOT0,
            SNAPSHOT1,
        }

        /// Convenient builder API for configuring WASI.
        ///
        /// Use the constructor to pass the arguments, environments,
        /// preopen directories and map directories, or use the
        /// associated methods to build the state step-by-steps.
        pub struct StateBuilder;

        impl StateBuilder {
            pub fn new(program_name: String) -> Self {
                x!()
            }

            /// Adds an argument.
            ///
            /// Arguments must not contain the nul (`0x0`) byte.
            pub fn argument(&mut self, argument: String) -> Self {
                x!()
            }

            /// Adds multiple arguments.
            ///
            /// Arguments must not contain the nul (`0x0`) byte.
            pub fn arguments(&mut self, arguments: Array<String>) -> Self {
                x!()
            }

            /// Add an environment variable pair.
            ///
            /// Environment variable keys and values must not contain
            /// the byte `=` (`0x3d`) or null (`0x0`).
            pub fn environment(&mut self, key: String, value: String) -> Self {
                x!()
            }

            /// Add environment variable pairs.
            ///
            /// Environment variable keys and values must not contain
            /// the byte `=` (`0x3d`) or null (`0x0`).
            pub fn environments(&mut self, pairs: Hash<String, String>) -> Self {
                x!()
            }

            /// Preopen a directory with a different name exposed to the WASI.
            ///
            /// This opens the given directories at the virtual root,
            /// `/`, and allows the WASI module to read and write to the
            /// given directories.
            pub fn preopen_directory(&mut self, alias: String, value: String) -> Self {
                x!()
            }

            /// Preopen directories with a different name exposed to the WASI.
            ///
            /// This opens the given directories at the virtual root,
            /// `/`, and allows the WASI module to read and write to the
            /// given directories.
            pub fn preopen_directories(&mut self, pairs: Hash<String, String>) -> Self {
                x!()
            }

            /// Preopen a directory with a different name exposed to the WASI.
            pub fn map_directory(&mut self, alias: String, value: String) -> Self {
                x!()
            }

            /// Preopen directories with a different name exposed to the WASI.
            pub fn map_directories(&mut self, pairs: Hash<String, String>) -> Self {
                x!()
            }

            /// Produces a WASI [`Environment`] based on this state builder.
            pub fn finalize(&mut self) -> Environment {
                x!()
            }
        }

        /// The environment provided to the WASI imports.
        ///
        /// To build it, use [`StateBuilder`]. See
        /// [`StateBuilder::finalize`] to learn more.
        pub struct Environment;

        impl Environment {
            /// Create an [`ImportObject`] with an existing
            /// [`Environment`]. The import object will be different
            /// according to the WASI version.
            ///
            /// Use the [`Version`] enum to use a specific WASI
            /// version, or use [`get_version`] to read the WASI
            /// version from a [`Module`].
            pub fn generate_import_object(
                &self,
                store: Store,
                wasi_version: Version,
            ) -> ImportObject {
                x!()
            }
        }

        /// Detect the version of WASI being used based on the import
        /// namespaces.
        ///
        /// A strict detection expects that all imports live in a
        /// single WASI namespace. A non-strict detection expects that
        /// at least one WASI namespace exits to detect the
        /// version. Note that the strict detection is faster than the
        /// non-strict one.
        pub fn get_version(module: Module, strict: Boolean) -> Version {
            x!()
        }
    }
}
