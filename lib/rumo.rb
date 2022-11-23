# frozen_string_literal: true

require_relative 'rumo/version'
require_relative 'rumo/rumo'

module Rumo
  class Error < StandardError; end

  module InitWithVararg
    def new(*args)
      _new(args)
    end
  end

  class UInt8
    extend InitWithVararg
  end

  class Int8
    extend InitWithVararg
  end

  class UInt16
    extend InitWithVararg
  end

  class Int16
    extend InitWithVararg
  end

  class UInt32
    extend InitWithVararg
  end

  class Int32
    extend InitWithVararg
  end

  class UInt64
    extend InitWithVararg
  end

  class Int64
    extend InitWithVararg
  end

  class Float32
    extend InitWithVararg
  end

  class Float64
    extend InitWithVararg
  end
end
