# frozen_string_literal: true

require 'test_helper'

class RumoTest < Test::Unit::TestCase
  test 'VERSION' do
    assert do
      ::Rumo.const_defined?(:VERSION)
    end
  end

  test 'new0' do
    assert_instance_of(::Rumo::UInt8, Rumo::UInt8.new(0))
    assert_instance_of(::Rumo::Int8, Rumo::Int8.new(0))
    assert_instance_of(::Rumo::UInt16, Rumo::UInt16.new(0))
    assert_instance_of(::Rumo::Int16, Rumo::Int16.new(0))
    assert_instance_of(::Rumo::UInt32, Rumo::UInt32.new(0))
    assert_instance_of(::Rumo::Int32, Rumo::Int32.new(0))
    assert_instance_of(::Rumo::UInt64, Rumo::UInt64.new(0))
    assert_instance_of(::Rumo::Int64, Rumo::Int64.new(0))
    assert_instance_of(::Rumo::Float32, Rumo::Float32.new(0))
    assert_instance_of(::Rumo::Float32, Rumo::SFloat.new(0))
    assert_instance_of(::Rumo::Float64, Rumo::Float64.new(0))
    assert_instance_of(::Rumo::Float64, Rumo::DFloat.new(0))
  end

  test 'new1' do
    assert_instance_of(::Rumo::UInt8, Rumo::UInt8.new(1))
    assert_instance_of(::Rumo::Int8, Rumo::Int8.new(1))
    assert_instance_of(::Rumo::UInt16, Rumo::UInt16.new(1))
    assert_instance_of(::Rumo::Int16, Rumo::Int16.new(1))
    assert_instance_of(::Rumo::UInt32, Rumo::UInt32.new(1))
    assert_instance_of(::Rumo::Int32, Rumo::Int32.new(1))
    assert_instance_of(::Rumo::UInt64, Rumo::UInt64.new(1))
    assert_instance_of(::Rumo::Int64, Rumo::Int64.new(1))
    assert_instance_of(::Rumo::Float32, Rumo::Float32.new(1))
    assert_instance_of(::Rumo::Float32, Rumo::SFloat.new(1))
    assert_instance_of(::Rumo::Float64, Rumo::Float64.new(1))
    assert_instance_of(::Rumo::Float64, Rumo::DFloat.new(1))
  end

  test 'new2' do
    assert_instance_of(::Rumo::UInt8, Rumo::UInt8.new(1, 2))
    assert_instance_of(::Rumo::Int8, Rumo::Int8.new(1, 2))
    assert_instance_of(::Rumo::UInt16, Rumo::UInt16.new(1, 2))
    assert_instance_of(::Rumo::Int16, Rumo::Int16.new(1, 2))
    assert_instance_of(::Rumo::UInt32, Rumo::UInt32.new(1, 2))
    assert_instance_of(::Rumo::Int32, Rumo::Int32.new(1, 2))
    assert_instance_of(::Rumo::UInt64, Rumo::UInt64.new(1, 2))
    assert_instance_of(::Rumo::Int64, Rumo::Int64.new(1, 2))
    assert_instance_of(::Rumo::Float32, Rumo::Float32.new(1, 2))
    assert_instance_of(::Rumo::Float32, Rumo::SFloat.new(1, 2))
    assert_instance_of(::Rumo::Float64, Rumo::Float64.new(1, 2))
    assert_instance_of(::Rumo::Float64, Rumo::DFloat.new(1, 2))
  end

  test 'new3' do
    assert_instance_of(::Rumo::UInt8, Rumo::UInt8.new(1, 2, 3))
    assert_instance_of(::Rumo::Int8, Rumo::Int8.new(1, 2, 3))
    assert_instance_of(::Rumo::UInt16, Rumo::UInt16.new(1, 2, 3))
    assert_instance_of(::Rumo::Int16, Rumo::Int16.new(1, 2, 3))
    assert_instance_of(::Rumo::UInt32, Rumo::UInt32.new(1, 2, 3))
    assert_instance_of(::Rumo::Int32, Rumo::Int32.new(1, 2, 3))
    assert_instance_of(::Rumo::UInt64, Rumo::UInt64.new(1, 2, 3))
    assert_instance_of(::Rumo::Int64, Rumo::Int64.new(1, 2, 3))
    assert_instance_of(::Rumo::Float32, Rumo::Float32.new(1, 2, 3))
    assert_instance_of(::Rumo::Float32, Rumo::SFloat.new(1, 2, 3))
    assert_instance_of(::Rumo::Float64, Rumo::Float64.new(1, 2, 3))
    assert_instance_of(::Rumo::Float64, Rumo::DFloat.new(1, 2, 3))
  end

  test 'new4' do
    assert_instance_of(::Rumo::UInt8, Rumo::UInt8.new(1, 2, 3, 4))
    assert_instance_of(::Rumo::Int8, Rumo::Int8.new(1, 2, 3, 4))
    assert_instance_of(::Rumo::UInt16, Rumo::UInt16.new(1, 2, 3, 4))
    assert_instance_of(::Rumo::Int16, Rumo::Int16.new(1, 2, 3, 4))
    assert_instance_of(::Rumo::UInt32, Rumo::UInt32.new(1, 2, 3, 4))
    assert_instance_of(::Rumo::Int32, Rumo::Int32.new(1, 2, 3, 4))
    assert_instance_of(::Rumo::UInt64, Rumo::UInt64.new(1, 2, 3, 4))
    assert_instance_of(::Rumo::Int64, Rumo::Int64.new(1, 2, 3, 4))
    assert_instance_of(::Rumo::Float32, Rumo::Float32.new(1, 2, 3, 4))
    assert_instance_of(::Rumo::Float32, Rumo::SFloat.new(1, 2, 3, 4))
    assert_instance_of(::Rumo::Float64, Rumo::Float64.new(1, 2, 3, 4))
    assert_instance_of(::Rumo::Float64, Rumo::DFloat.new(1, 2, 3, 4))
  end

  test 'shape0' do
    assert_equal([0], Rumo::UInt8.new(0).shape)
    assert_equal([0], Rumo::Int8.new(0).shape)
    assert_equal([0], Rumo::UInt16.new(0).shape)
    assert_equal([0], Rumo::Int16.new(0).shape)
    assert_equal([0], Rumo::UInt32.new(0).shape)
    assert_equal([0], Rumo::Int32.new(0).shape)
    assert_equal([0], Rumo::UInt64.new(0).shape)
    assert_equal([0], Rumo::Int64.new(0).shape)
    assert_equal([0], Rumo::Float32.new(0).shape)
    assert_equal([0], Rumo::SFloat.new(0).shape)
    assert_equal([0], Rumo::Float64.new(0).shape)
    assert_equal([0], Rumo::DFloat.new(0).shape)
  end

  test 'shape1' do
    assert_equal([1], Rumo::UInt8.new(1).shape)
    assert_equal([1], Rumo::Int8.new(1).shape)
    assert_equal([1], Rumo::UInt16.new(1).shape)
    assert_equal([1], Rumo::Int16.new(1).shape)
    assert_equal([1], Rumo::UInt32.new(1).shape)
    assert_equal([1], Rumo::Int32.new(1).shape)
    assert_equal([1], Rumo::UInt64.new(1).shape)
    assert_equal([1], Rumo::Int64.new(1).shape)
    assert_equal([1], Rumo::Float32.new(1).shape)
    assert_equal([1], Rumo::SFloat.new(1).shape)
    assert_equal([1], Rumo::Float64.new(1).shape)
    assert_equal([1], Rumo::DFloat.new(1).shape)
  end

  test 'shape2' do
    assert_equal([1, 2], Rumo::UInt8.new(1, 2).shape)
    assert_equal([1, 2], Rumo::Int8.new(1, 2).shape)
    assert_equal([1, 2], Rumo::UInt16.new(1, 2).shape)
    assert_equal([1, 2], Rumo::Int16.new(1, 2).shape)
    assert_equal([1, 2], Rumo::UInt32.new(1, 2).shape)
    assert_equal([1, 2], Rumo::Int32.new(1, 2).shape)
    assert_equal([1, 2], Rumo::UInt64.new(1, 2).shape)
    assert_equal([1, 2], Rumo::Int64.new(1, 2).shape)
    assert_equal([1, 2], Rumo::Float32.new(1, 2).shape)
    assert_equal([1, 2], Rumo::SFloat.new(1, 2).shape)
    assert_equal([1, 2], Rumo::Float64.new(1, 2).shape)
    assert_equal([1, 2], Rumo::DFloat.new(1, 2).shape)
  end

  test 'shape3' do
    assert_equal([1, 2, 3], Rumo::UInt8.new(1, 2, 3).shape)
    assert_equal([1, 2, 3], Rumo::Int8.new(1, 2, 3).shape)
    assert_equal([1, 2, 3], Rumo::UInt16.new(1, 2, 3).shape)
    assert_equal([1, 2, 3], Rumo::Int16.new(1, 2, 3).shape)
    assert_equal([1, 2, 3], Rumo::UInt32.new(1, 2, 3).shape)
    assert_equal([1, 2, 3], Rumo::Int32.new(1, 2, 3).shape)
    assert_equal([1, 2, 3], Rumo::UInt64.new(1, 2, 3).shape)
    assert_equal([1, 2, 3], Rumo::Int64.new(1, 2, 3).shape)
    assert_equal([1, 2, 3], Rumo::Float32.new(1, 2, 3).shape)
    assert_equal([1, 2, 3], Rumo::SFloat.new(1, 2, 3).shape)
    assert_equal([1, 2, 3], Rumo::Float64.new(1, 2, 3).shape)
    assert_equal([1, 2, 3], Rumo::DFloat.new(1, 2, 3).shape)
  end

  test 'ndim0' do
    assert_equal(1, Rumo::UInt8.new(0).ndim)
    assert_equal(1, Rumo::Int8.new(0).ndim)
    assert_equal(1, Rumo::UInt16.new(0).ndim)
    assert_equal(1, Rumo::Int16.new(0).ndim)
    assert_equal(1, Rumo::UInt32.new(0).ndim)
    assert_equal(1, Rumo::Int32.new(0).ndim)
    assert_equal(1, Rumo::UInt64.new(0).ndim)
    assert_equal(1, Rumo::Int64.new(0).ndim)
    assert_equal(1, Rumo::Float32.new(0).ndim)
    assert_equal(1, Rumo::SFloat.new(0).ndim)
    assert_equal(1, Rumo::Float64.new(0).ndim)
    assert_equal(1, Rumo::DFloat.new(0).ndim)
  end

  test 'ndim1' do
    assert_equal(1, Rumo::UInt8.new(1).ndim)
    assert_equal(1, Rumo::Int8.new(1).ndim)
    assert_equal(1, Rumo::UInt16.new(1).ndim)
    assert_equal(1, Rumo::Int16.new(1).ndim)
    assert_equal(1, Rumo::UInt32.new(1).ndim)
    assert_equal(1, Rumo::Int32.new(1).ndim)
    assert_equal(1, Rumo::UInt64.new(1).ndim)
    assert_equal(1, Rumo::Int64.new(1).ndim)
    assert_equal(1, Rumo::Float32.new(1).ndim)
    assert_equal(1, Rumo::SFloat.new(1).ndim)
    assert_equal(1, Rumo::Float64.new(1).ndim)
    assert_equal(1, Rumo::DFloat.new(1).ndim)
  end

  test 'ndim2' do
    assert_equal(2, Rumo::UInt8.new(1, 2).ndim)
    assert_equal(2, Rumo::Int8.new(1, 2).ndim)
    assert_equal(2, Rumo::UInt16.new(1, 2).ndim)
    assert_equal(2, Rumo::Int16.new(1, 2).ndim)
    assert_equal(2, Rumo::UInt32.new(1, 2).ndim)
    assert_equal(2, Rumo::Int32.new(1, 2).ndim)
    assert_equal(2, Rumo::UInt64.new(1, 2).ndim)
    assert_equal(2, Rumo::Int64.new(1, 2).ndim)
    assert_equal(2, Rumo::Float32.new(1, 2).ndim)
    assert_equal(2, Rumo::SFloat.new(1, 2).ndim)
    assert_equal(2, Rumo::Float64.new(1, 2).ndim)
    assert_equal(2, Rumo::DFloat.new(1, 2).ndim)
  end

  test 'ndim3' do
    assert_equal(3, Rumo::UInt8.new(1, 2, 3).ndim)
    assert_equal(3, Rumo::Int8.new(1, 2, 3).ndim)
    assert_equal(3, Rumo::UInt16.new(1, 2, 3).ndim)
    assert_equal(3, Rumo::Int16.new(1, 2, 3).ndim)
    assert_equal(3, Rumo::UInt32.new(1, 2, 3).ndim)
    assert_equal(3, Rumo::Int32.new(1, 2, 3).ndim)
    assert_equal(3, Rumo::UInt64.new(1, 2, 3).ndim)
    assert_equal(3, Rumo::Int64.new(1, 2, 3).ndim)
    assert_equal(3, Rumo::Float32.new(1, 2, 3).ndim)
    assert_equal(3, Rumo::SFloat.new(1, 2, 3).ndim)
    assert_equal(3, Rumo::Float64.new(1, 2, 3).ndim)
    assert_equal(3, Rumo::DFloat.new(1, 2, 3).ndim)
  end
end
