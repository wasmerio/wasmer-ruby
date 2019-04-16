require 'wasmer/version'
require 'rutie'

module Wasmer
  Rutie.new(:wasmer).init 'Init_wasmer', __dir__
end
