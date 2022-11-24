extern crate magnus;
extern crate ndarray;
use ndarray::{array, prelude::*, Array, ArrayD, ArrayView, IxDyn};

use magnus::{define_class, define_module, eval, function, method, prelude::*, Error};

trait NArray {
    fn zeros(dims: Vec<usize>) -> Self;
    fn ones(dims: Vec<usize>) -> Self;
    fn shape(&self) -> Vec<usize>;
    fn ndim(&self) -> usize;
    fn length(&self) -> usize;
    fn inspect(&self) -> String;
}

#[magnus::wrap(class = "Rumo::UInt8")]
struct UInt8 {
    nda: ArrayD<u8>,
}

impl NArray for UInt8 {
    fn zeros(dims: Vec<usize>) -> Self {
        let mut nda = ArrayD::<u8>::zeros(IxDyn(&dims));
        Self { nda }
    }
    fn ones(dims: Vec<usize>) -> Self {
        let mut nda = ArrayD::<u8>::ones(IxDyn(&dims));
        Self { nda }
    }
    fn shape(&self) -> Vec<usize> {
        self.nda.shape().to_vec()
    }
    fn ndim(&self) -> usize {
        self.nda.ndim()
    }
    fn length(&self) -> usize {
        self.nda.len()
    }
    fn inspect(&self) -> String {
        self.nda.to_string()
    }
}

impl UInt8 {
    fn sum(&self) -> u8 {
        self.nda.sum()
    }
}

#[magnus::wrap(class = "Rumo::Int8")]
struct Int8 {
    nda: ArrayD<i8>,
}

impl NArray for Int8 {
    fn zeros(dims: Vec<usize>) -> Self {
        let mut nda = ArrayD::<i8>::zeros(IxDyn(&dims));
        Self { nda }
    }
    fn ones(dims: Vec<usize>) -> Self {
        let mut nda = ArrayD::<i8>::ones(IxDyn(&dims));
        Self { nda }
    }
    fn shape(&self) -> Vec<usize> {
        self.nda.shape().to_vec()
    }
    fn ndim(&self) -> usize {
        self.nda.ndim()
    }
    fn length(&self) -> usize {
        self.nda.len()
    }
    fn inspect(&self) -> String {
        self.nda.to_string()
    }
}

impl Int8 {
    fn sum(&self) -> i8 {
        self.nda.sum()
    }
}

#[magnus::wrap(class = "Rumo::UInt16")]
struct UInt16 {
    nda: ArrayD<u16>,
}

impl NArray for UInt16 {
    fn zeros(dims: Vec<usize>) -> Self {
        let mut nda = ArrayD::<u16>::zeros(IxDyn(&dims));
        Self { nda }
    }
    fn ones(dims: Vec<usize>) -> Self {
        let mut nda = ArrayD::<u16>::ones(IxDyn(&dims));
        Self { nda }
    }
    fn shape(&self) -> Vec<usize> {
        self.nda.shape().to_vec()
    }
    fn ndim(&self) -> usize {
        self.nda.ndim()
    }
    fn length(&self) -> usize {
        self.nda.len()
    }
    fn inspect(&self) -> String {
        self.nda.to_string()
    }
}

impl UInt16 {
    fn sum(&self) -> u16 {
        self.nda.sum()
    }
}

#[magnus::wrap(class = "Rumo::Int16")]
struct Int16 {
    nda: ArrayD<i16>,
}

impl NArray for Int16 {
    fn zeros(dims: Vec<usize>) -> Self {
        let mut nda = ArrayD::<i16>::zeros(IxDyn(&dims));
        Self { nda }
    }
    fn ones(dims: Vec<usize>) -> Self {
        let mut nda = ArrayD::<i16>::ones(IxDyn(&dims));
        Self { nda }
    }
    fn shape(&self) -> Vec<usize> {
        self.nda.shape().to_vec()
    }
    fn ndim(&self) -> usize {
        self.nda.ndim()
    }
    fn length(&self) -> usize {
        self.nda.len()
    }
    fn inspect(&self) -> String {
        self.nda.to_string()
    }
}

impl Int16 {
    fn sum(&self) -> i16 {
        self.nda.sum()
    }
}

#[magnus::wrap(class = "Rumo::UInt32")]
struct UInt32 {
    nda: ArrayD<u32>,
}

impl NArray for UInt32 {
    fn zeros(dims: Vec<usize>) -> Self {
        let mut nda = ArrayD::<u32>::zeros(IxDyn(&dims));
        Self { nda }
    }
    fn ones(dims: Vec<usize>) -> Self {
        let mut nda = ArrayD::<u32>::ones(IxDyn(&dims));
        Self { nda }
    }
    fn shape(&self) -> Vec<usize> {
        self.nda.shape().to_vec()
    }
    fn ndim(&self) -> usize {
        self.nda.ndim()
    }
    fn length(&self) -> usize {
        self.nda.len()
    }
    fn inspect(&self) -> String {
        self.nda.to_string()
    }
}

impl UInt32 {
    fn sum(&self) -> u32 {
        self.nda.sum()
    }
}

#[magnus::wrap(class = "Rumo::Int32")]
struct Int32 {
    nda: ArrayD<i32>,
}

impl NArray for Int32 {
    fn zeros(dims: Vec<usize>) -> Self {
        let mut nda = ArrayD::<i32>::zeros(IxDyn(&dims));
        Self { nda }
    }
    fn ones(dims: Vec<usize>) -> Self {
        let mut nda = ArrayD::<i32>::ones(IxDyn(&dims));
        Self { nda }
    }
    fn shape(&self) -> Vec<usize> {
        self.nda.shape().to_vec()
    }
    fn ndim(&self) -> usize {
        self.nda.ndim()
    }
    fn length(&self) -> usize {
        self.nda.len()
    }
    fn inspect(&self) -> String {
        self.nda.to_string()
    }
}

impl Int32 {
    fn sum(&self) -> i32 {
        self.nda.sum()
    }
}

#[magnus::wrap(class = "Rumo::UInt64")]
struct UInt64 {
    nda: ArrayD<u64>,
}

impl NArray for UInt64 {
    fn zeros(dims: Vec<usize>) -> Self {
        let mut nda = ArrayD::<u64>::zeros(IxDyn(&dims));
        Self { nda }
    }
    fn ones(dims: Vec<usize>) -> Self {
        let mut nda = ArrayD::<u64>::ones(IxDyn(&dims));
        Self { nda }
    }
    fn shape(&self) -> Vec<usize> {
        self.nda.shape().to_vec()
    }
    fn ndim(&self) -> usize {
        self.nda.ndim()
    }
    fn length(&self) -> usize {
        self.nda.len()
    }
    fn inspect(&self) -> String {
        self.nda.to_string()
    }
}

impl UInt64 {
    fn sum(&self) -> u64 {
        self.nda.sum()
    }
}

#[magnus::wrap(class = "Rumo::Int64")]
struct Int64 {
    nda: ArrayD<i64>,
}

impl NArray for Int64 {
    fn zeros(dims: Vec<usize>) -> Self {
        let mut nda = ArrayD::<i64>::zeros(IxDyn(&dims));
        Self { nda }
    }
    fn ones(dims: Vec<usize>) -> Self {
        let mut nda = ArrayD::<i64>::ones(IxDyn(&dims));
        Self { nda }
    }
    fn shape(&self) -> Vec<usize> {
        self.nda.shape().to_vec()
    }
    fn ndim(&self) -> usize {
        self.nda.ndim()
    }
    fn length(&self) -> usize {
        self.nda.len()
    }
    fn inspect(&self) -> String {
        self.nda.to_string()
    }
}

impl Int64 {
    fn sum(&self) -> i64 {
        self.nda.sum()
    }
}

#[magnus::wrap(class = "Rumo::Float32")]
struct Float32 {
    nda: ArrayD<f32>,
}

impl NArray for Float32 {
    fn zeros(dims: Vec<usize>) -> Self {
        let mut nda = ArrayD::<f32>::zeros(IxDyn(&dims));
        Self { nda }
    }
    fn ones(dims: Vec<usize>) -> Self {
        let mut nda = ArrayD::<f32>::ones(IxDyn(&dims));
        Self { nda }
    }
    fn shape(&self) -> Vec<usize> {
        self.nda.shape().to_vec()
    }
    fn ndim(&self) -> usize {
        self.nda.ndim()
    }
    fn length(&self) -> usize {
        self.nda.len()
    }
    fn inspect(&self) -> String {
        self.nda.to_string()
    }
}

impl Float32 {
    fn sum(&self) -> f32 {
        self.nda.sum()
    }
}

#[magnus::wrap(class = "Rumo::Float64")]
struct Float64 {
    nda: ArrayD<f64>,
}

impl NArray for Float64 {
    fn zeros(dims: Vec<usize>) -> Self {
        let mut nda = ArrayD::<f64>::zeros(IxDyn(&dims));
        Self { nda }
    }
    fn ones(dims: Vec<usize>) -> Self {
        let mut nda = ArrayD::<f64>::ones(IxDyn(&dims));
        Self { nda }
    }
    fn shape(&self) -> Vec<usize> {
        self.nda.shape().to_vec()
    }
    fn ndim(&self) -> usize {
        self.nda.ndim()
    }
    fn length(&self) -> usize {
        self.nda.len()
    }
    fn inspect(&self) -> String {
        self.nda.to_string()
    }
}

impl Float64 {
    fn sum(&self) -> f64 {
        self.nda.sum()
    }
}

#[magnus::init]
fn init() -> Result<(), Error> {
    let module = define_module("Rumo")?;

    let class_u8 = module.define_class("UInt8", Default::default())?;
    class_u8.define_singleton_method("_zeros", function!(UInt8::zeros, 1))?;
    class_u8.define_singleton_method("_ones", function!(UInt8::ones, 1))?;
    class_u8.define_method("shape", method!(UInt8::shape, 0))?;
    class_u8.define_method("ndim", method!(UInt8::ndim, 0))?;
    class_u8.define_method("length", method!(UInt8::length, 0))?;
    class_u8.define_method("size", method!(UInt8::length, 0))?;
    class_u8.define_method("sum", method!(UInt8::sum, 0))?;
    class_u8.define_method("inspect", method!(UInt8::inspect, 0))?;

    let class_i8 = module.define_class("Int8", Default::default())?;
    class_i8.define_singleton_method("_zeros", function!(Int8::zeros, 1))?;
    class_i8.define_singleton_method("_ones", function!(Int8::ones, 1))?;
    class_i8.define_method("shape", method!(Int8::shape, 0))?;
    class_i8.define_method("ndim", method!(Int8::ndim, 0))?;
    class_i8.define_method("length", method!(Int8::length, 0))?;
    class_i8.define_method("size", method!(Int8::length, 0))?;
    class_i8.define_method("sum", method!(Int8::sum, 0))?;
    class_i8.define_method("inspect", method!(Int8::inspect, 0))?;

    let class_u16 = module.define_class("UInt16", Default::default())?;
    class_u16.define_singleton_method("_zeros", function!(UInt16::zeros, 1))?;
    class_u16.define_singleton_method("_ones", function!(UInt16::ones, 1))?;
    class_u16.define_method("shape", method!(UInt16::shape, 0))?;
    class_u16.define_method("ndim", method!(UInt16::ndim, 0))?;
    class_u16.define_method("length", method!(UInt16::length, 0))?;
    class_u16.define_method("size", method!(UInt16::length, 0))?;
    class_u16.define_method("sum", method!(UInt16::sum, 0))?;
    class_u16.define_method("inspect", method!(UInt16::inspect, 0))?;

    let class_i16 = module.define_class("Int16", Default::default())?;
    class_i16.define_singleton_method("_zeros", function!(Int16::zeros, 1))?;
    class_i16.define_singleton_method("_ones", function!(Int16::ones, 1))?;
    class_i16.define_method("shape", method!(Int16::shape, 0))?;
    class_i16.define_method("ndim", method!(Int16::ndim, 0))?;
    class_i16.define_method("length", method!(Int16::length, 0))?;
    class_i16.define_method("size", method!(Int16::length, 0))?;
    class_i16.define_method("sum", method!(Int16::sum, 0))?;
    class_i16.define_method("inspect", method!(Int16::inspect, 0))?;

    let class_u32 = module.define_class("UInt32", Default::default())?;
    class_u32.define_singleton_method("_zeros", function!(UInt32::zeros, 1))?;
    class_u32.define_singleton_method("_ones", function!(UInt32::ones, 1))?;
    class_u32.define_method("shape", method!(UInt32::shape, 0))?;
    class_u32.define_method("ndim", method!(UInt32::ndim, 0))?;
    class_u32.define_method("length", method!(UInt32::length, 0))?;
    class_u32.define_method("size", method!(UInt32::length, 0))?;
    class_u32.define_method("sum", method!(UInt32::sum, 0))?;
    class_u32.define_method("inspect", method!(UInt32::inspect, 0))?;

    let class_i32 = module.define_class("Int32", Default::default())?;
    class_i32.define_singleton_method("_zeros", function!(Int32::zeros, 1))?;
    class_i32.define_singleton_method("_ones", function!(Int32::ones, 1))?;
    class_i32.define_method("shape", method!(Int32::shape, 0))?;
    class_i32.define_method("ndim", method!(Int32::ndim, 0))?;
    class_i32.define_method("length", method!(Int32::length, 0))?;
    class_i32.define_method("size", method!(Int32::length, 0))?;
    class_i32.define_method("sum", method!(Int32::sum, 0))?;
    class_i32.define_method("inspect", method!(Int32::inspect, 0))?;

    let class_u64 = module.define_class("UInt64", Default::default())?;
    class_u64.define_singleton_method("_zeros", function!(UInt64::zeros, 1))?;
    class_u64.define_singleton_method("_ones", function!(UInt64::ones, 1))?;
    class_u64.define_method("shape", method!(UInt64::shape, 0))?;
    class_u64.define_method("ndim", method!(UInt64::ndim, 0))?;
    class_u64.define_method("length", method!(UInt64::length, 0))?;
    class_u64.define_method("size", method!(UInt64::length, 0))?;
    class_u64.define_method("sum", method!(UInt64::sum, 0))?;
    class_u64.define_method("inspect", method!(UInt64::inspect, 0))?;

    let class_i64 = module.define_class("Int64", Default::default())?;
    class_i64.define_singleton_method("_zeros", function!(Int64::zeros, 1))?;
    class_i64.define_singleton_method("_ones", function!(Int64::ones, 1))?;
    class_i64.define_method("shape", method!(Int64::shape, 0))?;
    class_i64.define_method("ndim", method!(Int64::ndim, 0))?;
    class_i64.define_method("length", method!(Int64::length, 0))?;
    class_i64.define_method("size", method!(Int64::length, 0))?;
    class_i64.define_method("sum", method!(Int64::sum, 0))?;
    class_i64.define_method("inspect", method!(Int64::inspect, 0))?;

    let class_f32 = module.define_class("Float32", Default::default())?;
    class_f32.define_singleton_method("_zeros", function!(Float32::zeros, 1))?;
    class_f32.define_singleton_method("_ones", function!(Float32::ones, 1))?;
    class_f32.define_method("shape", method!(Float32::shape, 0))?;
    class_f32.define_method("ndim", method!(Float32::ndim, 0))?;
    class_f32.define_method("length", method!(Float32::length, 0))?;
    class_f32.define_method("size", method!(Float32::length, 0))?;
    class_f32.define_method("sum", method!(Float32::sum, 0))?;
    class_f32.define_method("inspect", method!(Float32::inspect, 0))?;

    let class_f64 = module.define_class("Float64", Default::default())?;
    class_f64.define_singleton_method("_zeros", function!(Float64::zeros, 1))?;
    class_f64.define_singleton_method("_ones", function!(Float64::ones, 1))?;
    class_f64.define_method("shape", method!(Float64::shape, 0))?;
    class_f64.define_method("ndim", method!(Float64::ndim, 0))?;
    class_f64.define_method("length", method!(Float64::length, 0))?;
    class_f64.define_method("size", method!(Float64::length, 0))?;
    class_f64.define_method("sum", method!(Float64::sum, 0))?;
    class_f64.define_method("inspect", method!(Float64::inspect, 0))?;

    eval::<bool>("Rumo::SFloat = Rumo::Float32")?;
    eval::<bool>("Rumo::DFloat = Rumo::Float64")?;

    Ok(())
}
