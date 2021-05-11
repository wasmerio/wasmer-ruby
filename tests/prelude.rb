$LOAD_PATH.unshift File.expand_path("../../lib", __FILE__)

require "wasmer"
require "minitest/autorun"
require "minitest/reporters"

Minitest::Reporters.use! Minitest::Reporters::SpecReporter.new

ExportType = Wasmer::ExportType
Exports = Wasmer::Exports
Function = Wasmer::Function
FunctionType = Wasmer::FunctionType
Global = Wasmer::Global
GlobalType = Wasmer::GlobalType
ImportObject = Wasmer::ImportObject
ImportType = Wasmer::ImportType
Instance = Wasmer::Instance
Int16Array = Wasmer::Int16Array
Int32Array = Wasmer::Int32Array
Int8Array = Wasmer::Int8Array
Memory = Wasmer::Memory
MemoryType = Wasmer::MemoryType
Module = Wasmer::Module
Store = Wasmer::Store
Table = Wasmer::Table
TableType = Wasmer::TableType
Type = Wasmer::Type
Uint16Array = Wasmer::Uint16Array
Uint32Array = Wasmer::Uint32Array
Uint8Array = Wasmer::Uint8Array
Value = Wasmer::Value
Wasi = Wasmer::Wasi
