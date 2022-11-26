# frozen_string_literal: true

require_relative 'version'
require_relative 'rumo'

module Rumo
  class Error < StandardError; end

  module InitWithVararg
    def self.extended(obj)
      obj.private_class_method :_zeros
      obj.private_class_method :_ones
    end

    def new(*args)
      _zeros(args)
    end

    def zeros(*args)
      _zeros(args)
    end

    def ones(*args)
      _ones(args)
    end

    def reshape(*args)
      _reshape(args)
    end
  end

  # FIXME: This is a workaround
  module ReturnSelf
    def fill(...)
      super(...)
      self
    end
  end

  class UInt8
    extend InitWithVararg
    prepend ReturnSelf
  end

  class Int8
    extend InitWithVararg
    prepend ReturnSelf
  end

  class UInt16
    extend InitWithVararg
    prepend ReturnSelf
  end

  class Int16
    extend InitWithVararg
    prepend ReturnSelf
  end

  class UInt32
    extend InitWithVararg
    prepend ReturnSelf
  end

  class Int32
    extend InitWithVararg
    prepend ReturnSelf
  end

  class UInt64
    extend InitWithVararg
    prepend ReturnSelf
  end

  class Int64
    extend InitWithVararg
    prepend ReturnSelf
  end

  class Float32
    extend InitWithVararg
    prepend ReturnSelf
  end

  class Float64
    extend InitWithVararg
    prepend ReturnSelf
  end
end
