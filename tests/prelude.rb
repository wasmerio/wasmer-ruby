$LOAD_PATH.unshift File.expand_path("../../lib", __FILE__)

require "wasmer"
require "minitest/autorun"
require 'color_pound_spec_reporter'

Minitest::Reporters.use! [ColorPoundSpecReporter.new]   

ExportType = Wasmer::ExportType
Exports = Wasmer::Exports
Function = Wasmer::Function
FunctionType = Wasmer::FunctionType
GlobalType = Wasmer::GlobalType
ImportType = Wasmer::ImportType
Instance = Wasmer::Instance
Memory = Wasmer::Memory
MemoryType = Wasmer::MemoryType
Module = Wasmer::Module
Store = Wasmer::Store
TableType = Wasmer::TableType
Type = Wasmer::Type
Value = Wasmer::Value
