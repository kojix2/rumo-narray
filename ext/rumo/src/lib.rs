// This file is generated by ERB. Do not edit.

extern crate magnus;
extern crate ndarray;
use ndarray::{Array1, ArrayD, IxDyn};
use magnus::{define_module, eval, function, method, prelude::*, Error};
use std::cell::RefCell;

trait NArray {
    fn zeros(dims: Vec<usize>) -> Self;
    fn ones(dims: Vec<usize>) -> Self;
    fn shape(&self) -> Vec<usize>;
    fn ndim(&self) -> usize;
    fn len(&self) -> usize;
    fn to_string(&self) -> String;
}


struct RsUInt8 {
    nda: ArrayD<u8>,
}

#[magnus::wrap(class = "Rumo::UInt8")]
struct UInt8 {
    rc: RefCell<RsUInt8>,
}

impl NArray for UInt8 {
    fn zeros(dims: Vec<usize>) -> Self {
        let nda = ArrayD::<u8>::zeros(IxDyn(&dims));
        Self { rc: RefCell::new(RsUInt8 { nda }) }
    }
    fn ones(dims: Vec<usize>) -> Self {
        let nda = ArrayD::<u8>::ones(IxDyn(&dims));
        Self { rc: RefCell::new(RsUInt8 { nda }) }
    }
    fn shape(&self) -> Vec<usize> {
        self.rc.borrow().nda.shape().to_vec()
    }
    fn ndim(&self) -> usize {
        self.rc.borrow().nda.ndim()
    }
    fn len(&self) -> usize {
        self.rc.borrow().nda.len()
    }
    fn to_string(&self) -> String {
        self.rc.borrow().nda.to_string()
    }
}

impl UInt8 {
    fn fill(&self, value: u8) {
        self.rc.borrow_mut().nda.fill(value)
    }
    fn into_shape(&self, shape: Vec<usize>) -> Self {
        let nda = self.rc.borrow().nda.clone().into_shape(IxDyn(&shape)).unwrap();
        Self { rc: RefCell::new(RsUInt8 { nda }) }
    }
    fn sum(&self) -> u8 {
        self.rc.borrow().nda.sum()
    }
    fn mean(&self) -> Option<u8> {
        self.rc.borrow().nda.mean()
    }
    fn product(&self) -> u8 {
        self.rc.borrow().nda.product()
    }
}

struct RsInt8 {
    nda: ArrayD<i8>,
}

#[magnus::wrap(class = "Rumo::Int8")]
struct Int8 {
    rc: RefCell<RsInt8>,
}

impl NArray for Int8 {
    fn zeros(dims: Vec<usize>) -> Self {
        let nda = ArrayD::<i8>::zeros(IxDyn(&dims));
        Self { rc: RefCell::new(RsInt8 { nda }) }
    }
    fn ones(dims: Vec<usize>) -> Self {
        let nda = ArrayD::<i8>::ones(IxDyn(&dims));
        Self { rc: RefCell::new(RsInt8 { nda }) }
    }
    fn shape(&self) -> Vec<usize> {
        self.rc.borrow().nda.shape().to_vec()
    }
    fn ndim(&self) -> usize {
        self.rc.borrow().nda.ndim()
    }
    fn len(&self) -> usize {
        self.rc.borrow().nda.len()
    }
    fn to_string(&self) -> String {
        self.rc.borrow().nda.to_string()
    }
}

impl Int8 {
    fn fill(&self, value: i8) {
        self.rc.borrow_mut().nda.fill(value)
    }
    fn into_shape(&self, shape: Vec<usize>) -> Self {
        let nda = self.rc.borrow().nda.clone().into_shape(IxDyn(&shape)).unwrap();
        Self { rc: RefCell::new(RsInt8 { nda }) }
    }
    fn sum(&self) -> i8 {
        self.rc.borrow().nda.sum()
    }
    fn mean(&self) -> Option<i8> {
        self.rc.borrow().nda.mean()
    }
    fn product(&self) -> i8 {
        self.rc.borrow().nda.product()
    }
}

struct RsUInt16 {
    nda: ArrayD<u16>,
}

#[magnus::wrap(class = "Rumo::UInt16")]
struct UInt16 {
    rc: RefCell<RsUInt16>,
}

impl NArray for UInt16 {
    fn zeros(dims: Vec<usize>) -> Self {
        let nda = ArrayD::<u16>::zeros(IxDyn(&dims));
        Self { rc: RefCell::new(RsUInt16 { nda }) }
    }
    fn ones(dims: Vec<usize>) -> Self {
        let nda = ArrayD::<u16>::ones(IxDyn(&dims));
        Self { rc: RefCell::new(RsUInt16 { nda }) }
    }
    fn shape(&self) -> Vec<usize> {
        self.rc.borrow().nda.shape().to_vec()
    }
    fn ndim(&self) -> usize {
        self.rc.borrow().nda.ndim()
    }
    fn len(&self) -> usize {
        self.rc.borrow().nda.len()
    }
    fn to_string(&self) -> String {
        self.rc.borrow().nda.to_string()
    }
}

impl UInt16 {
    fn fill(&self, value: u16) {
        self.rc.borrow_mut().nda.fill(value)
    }
    fn into_shape(&self, shape: Vec<usize>) -> Self {
        let nda = self.rc.borrow().nda.clone().into_shape(IxDyn(&shape)).unwrap();
        Self { rc: RefCell::new(RsUInt16 { nda }) }
    }
    fn sum(&self) -> u16 {
        self.rc.borrow().nda.sum()
    }
    fn mean(&self) -> Option<u16> {
        self.rc.borrow().nda.mean()
    }
    fn product(&self) -> u16 {
        self.rc.borrow().nda.product()
    }
}

struct RsInt16 {
    nda: ArrayD<i16>,
}

#[magnus::wrap(class = "Rumo::Int16")]
struct Int16 {
    rc: RefCell<RsInt16>,
}

impl NArray for Int16 {
    fn zeros(dims: Vec<usize>) -> Self {
        let nda = ArrayD::<i16>::zeros(IxDyn(&dims));
        Self { rc: RefCell::new(RsInt16 { nda }) }
    }
    fn ones(dims: Vec<usize>) -> Self {
        let nda = ArrayD::<i16>::ones(IxDyn(&dims));
        Self { rc: RefCell::new(RsInt16 { nda }) }
    }
    fn shape(&self) -> Vec<usize> {
        self.rc.borrow().nda.shape().to_vec()
    }
    fn ndim(&self) -> usize {
        self.rc.borrow().nda.ndim()
    }
    fn len(&self) -> usize {
        self.rc.borrow().nda.len()
    }
    fn to_string(&self) -> String {
        self.rc.borrow().nda.to_string()
    }
}

impl Int16 {
    fn fill(&self, value: i16) {
        self.rc.borrow_mut().nda.fill(value)
    }
    fn into_shape(&self, shape: Vec<usize>) -> Self {
        let nda = self.rc.borrow().nda.clone().into_shape(IxDyn(&shape)).unwrap();
        Self { rc: RefCell::new(RsInt16 { nda }) }
    }
    fn sum(&self) -> i16 {
        self.rc.borrow().nda.sum()
    }
    fn mean(&self) -> Option<i16> {
        self.rc.borrow().nda.mean()
    }
    fn product(&self) -> i16 {
        self.rc.borrow().nda.product()
    }
}

struct RsUInt32 {
    nda: ArrayD<u32>,
}

#[magnus::wrap(class = "Rumo::UInt32")]
struct UInt32 {
    rc: RefCell<RsUInt32>,
}

impl NArray for UInt32 {
    fn zeros(dims: Vec<usize>) -> Self {
        let nda = ArrayD::<u32>::zeros(IxDyn(&dims));
        Self { rc: RefCell::new(RsUInt32 { nda }) }
    }
    fn ones(dims: Vec<usize>) -> Self {
        let nda = ArrayD::<u32>::ones(IxDyn(&dims));
        Self { rc: RefCell::new(RsUInt32 { nda }) }
    }
    fn shape(&self) -> Vec<usize> {
        self.rc.borrow().nda.shape().to_vec()
    }
    fn ndim(&self) -> usize {
        self.rc.borrow().nda.ndim()
    }
    fn len(&self) -> usize {
        self.rc.borrow().nda.len()
    }
    fn to_string(&self) -> String {
        self.rc.borrow().nda.to_string()
    }
}

impl UInt32 {
    fn fill(&self, value: u32) {
        self.rc.borrow_mut().nda.fill(value)
    }
    fn into_shape(&self, shape: Vec<usize>) -> Self {
        let nda = self.rc.borrow().nda.clone().into_shape(IxDyn(&shape)).unwrap();
        Self { rc: RefCell::new(RsUInt32 { nda }) }
    }
    fn sum(&self) -> u32 {
        self.rc.borrow().nda.sum()
    }
    fn mean(&self) -> Option<u32> {
        self.rc.borrow().nda.mean()
    }
    fn product(&self) -> u32 {
        self.rc.borrow().nda.product()
    }
}

struct RsInt32 {
    nda: ArrayD<i32>,
}

#[magnus::wrap(class = "Rumo::Int32")]
struct Int32 {
    rc: RefCell<RsInt32>,
}

impl NArray for Int32 {
    fn zeros(dims: Vec<usize>) -> Self {
        let nda = ArrayD::<i32>::zeros(IxDyn(&dims));
        Self { rc: RefCell::new(RsInt32 { nda }) }
    }
    fn ones(dims: Vec<usize>) -> Self {
        let nda = ArrayD::<i32>::ones(IxDyn(&dims));
        Self { rc: RefCell::new(RsInt32 { nda }) }
    }
    fn shape(&self) -> Vec<usize> {
        self.rc.borrow().nda.shape().to_vec()
    }
    fn ndim(&self) -> usize {
        self.rc.borrow().nda.ndim()
    }
    fn len(&self) -> usize {
        self.rc.borrow().nda.len()
    }
    fn to_string(&self) -> String {
        self.rc.borrow().nda.to_string()
    }
}

impl Int32 {
    fn fill(&self, value: i32) {
        self.rc.borrow_mut().nda.fill(value)
    }
    fn into_shape(&self, shape: Vec<usize>) -> Self {
        let nda = self.rc.borrow().nda.clone().into_shape(IxDyn(&shape)).unwrap();
        Self { rc: RefCell::new(RsInt32 { nda }) }
    }
    fn sum(&self) -> i32 {
        self.rc.borrow().nda.sum()
    }
    fn mean(&self) -> Option<i32> {
        self.rc.borrow().nda.mean()
    }
    fn product(&self) -> i32 {
        self.rc.borrow().nda.product()
    }
}

struct RsUInt64 {
    nda: ArrayD<u64>,
}

#[magnus::wrap(class = "Rumo::UInt64")]
struct UInt64 {
    rc: RefCell<RsUInt64>,
}

impl NArray for UInt64 {
    fn zeros(dims: Vec<usize>) -> Self {
        let nda = ArrayD::<u64>::zeros(IxDyn(&dims));
        Self { rc: RefCell::new(RsUInt64 { nda }) }
    }
    fn ones(dims: Vec<usize>) -> Self {
        let nda = ArrayD::<u64>::ones(IxDyn(&dims));
        Self { rc: RefCell::new(RsUInt64 { nda }) }
    }
    fn shape(&self) -> Vec<usize> {
        self.rc.borrow().nda.shape().to_vec()
    }
    fn ndim(&self) -> usize {
        self.rc.borrow().nda.ndim()
    }
    fn len(&self) -> usize {
        self.rc.borrow().nda.len()
    }
    fn to_string(&self) -> String {
        self.rc.borrow().nda.to_string()
    }
}

impl UInt64 {
    fn fill(&self, value: u64) {
        self.rc.borrow_mut().nda.fill(value)
    }
    fn into_shape(&self, shape: Vec<usize>) -> Self {
        let nda = self.rc.borrow().nda.clone().into_shape(IxDyn(&shape)).unwrap();
        Self { rc: RefCell::new(RsUInt64 { nda }) }
    }
    fn sum(&self) -> u64 {
        self.rc.borrow().nda.sum()
    }
    fn mean(&self) -> Option<u64> {
        self.rc.borrow().nda.mean()
    }
    fn product(&self) -> u64 {
        self.rc.borrow().nda.product()
    }
}

struct RsInt64 {
    nda: ArrayD<i64>,
}

#[magnus::wrap(class = "Rumo::Int64")]
struct Int64 {
    rc: RefCell<RsInt64>,
}

impl NArray for Int64 {
    fn zeros(dims: Vec<usize>) -> Self {
        let nda = ArrayD::<i64>::zeros(IxDyn(&dims));
        Self { rc: RefCell::new(RsInt64 { nda }) }
    }
    fn ones(dims: Vec<usize>) -> Self {
        let nda = ArrayD::<i64>::ones(IxDyn(&dims));
        Self { rc: RefCell::new(RsInt64 { nda }) }
    }
    fn shape(&self) -> Vec<usize> {
        self.rc.borrow().nda.shape().to_vec()
    }
    fn ndim(&self) -> usize {
        self.rc.borrow().nda.ndim()
    }
    fn len(&self) -> usize {
        self.rc.borrow().nda.len()
    }
    fn to_string(&self) -> String {
        self.rc.borrow().nda.to_string()
    }
}

impl Int64 {
    fn fill(&self, value: i64) {
        self.rc.borrow_mut().nda.fill(value)
    }
    fn into_shape(&self, shape: Vec<usize>) -> Self {
        let nda = self.rc.borrow().nda.clone().into_shape(IxDyn(&shape)).unwrap();
        Self { rc: RefCell::new(RsInt64 { nda }) }
    }
    fn sum(&self) -> i64 {
        self.rc.borrow().nda.sum()
    }
    fn mean(&self) -> Option<i64> {
        self.rc.borrow().nda.mean()
    }
    fn product(&self) -> i64 {
        self.rc.borrow().nda.product()
    }
}

struct RsFloat32 {
    nda: ArrayD<f32>,
}

#[magnus::wrap(class = "Rumo::Float32")]
struct Float32 {
    rc: RefCell<RsFloat32>,
}

impl NArray for Float32 {
    fn zeros(dims: Vec<usize>) -> Self {
        let nda = ArrayD::<f32>::zeros(IxDyn(&dims));
        Self { rc: RefCell::new(RsFloat32 { nda }) }
    }
    fn ones(dims: Vec<usize>) -> Self {
        let nda = ArrayD::<f32>::ones(IxDyn(&dims));
        Self { rc: RefCell::new(RsFloat32 { nda }) }
    }
    fn shape(&self) -> Vec<usize> {
        self.rc.borrow().nda.shape().to_vec()
    }
    fn ndim(&self) -> usize {
        self.rc.borrow().nda.ndim()
    }
    fn len(&self) -> usize {
        self.rc.borrow().nda.len()
    }
    fn to_string(&self) -> String {
        self.rc.borrow().nda.to_string()
    }
}

impl Float32 {
    fn linspace(start: f32, end: f32, n: usize) -> Self {
        let nda = Array1::<f32>::linspace(start, end, n).into_dyn();
        Self { rc: RefCell::new(RsFloat32 { nda }) }
    }
    fn range(start: f32, end: f32, step: f32) -> Self {
        let nda = Array1::<f32>::range(start, end, step).into_dyn();
        Self { rc: RefCell::new(RsFloat32 { nda }) }
    }
    fn fill(&self, value: f32) {
        self.rc.borrow_mut().nda.fill(value)
    }
    fn into_shape(&self, shape: Vec<usize>) -> Self {
        let nda = self.rc.borrow().nda.clone().into_shape(IxDyn(&shape)).unwrap();
        Self { rc: RefCell::new(RsFloat32 { nda }) }
    }
    fn sum(&self) -> f32 {
        self.rc.borrow().nda.sum()
    }
    fn mean(&self) -> Option<f32> {
        self.rc.borrow().nda.mean()
    }
    fn product(&self) -> f32 {
        self.rc.borrow().nda.product()
    }
    fn var(&self, ddof: f32) -> f32 {
        self.rc.borrow().nda.var(ddof)
    }
    fn std(&self, ddof: f32) -> f32 {
        self.rc.borrow().nda.std(ddof)
    }
}

struct RsFloat64 {
    nda: ArrayD<f64>,
}

#[magnus::wrap(class = "Rumo::Float64")]
struct Float64 {
    rc: RefCell<RsFloat64>,
}

impl NArray for Float64 {
    fn zeros(dims: Vec<usize>) -> Self {
        let nda = ArrayD::<f64>::zeros(IxDyn(&dims));
        Self { rc: RefCell::new(RsFloat64 { nda }) }
    }
    fn ones(dims: Vec<usize>) -> Self {
        let nda = ArrayD::<f64>::ones(IxDyn(&dims));
        Self { rc: RefCell::new(RsFloat64 { nda }) }
    }
    fn shape(&self) -> Vec<usize> {
        self.rc.borrow().nda.shape().to_vec()
    }
    fn ndim(&self) -> usize {
        self.rc.borrow().nda.ndim()
    }
    fn len(&self) -> usize {
        self.rc.borrow().nda.len()
    }
    fn to_string(&self) -> String {
        self.rc.borrow().nda.to_string()
    }
}

impl Float64 {
    fn linspace(start: f64, end: f64, n: usize) -> Self {
        let nda = Array1::<f64>::linspace(start, end, n).into_dyn();
        Self { rc: RefCell::new(RsFloat64 { nda }) }
    }
    fn range(start: f64, end: f64, step: f64) -> Self {
        let nda = Array1::<f64>::range(start, end, step).into_dyn();
        Self { rc: RefCell::new(RsFloat64 { nda }) }
    }
    fn fill(&self, value: f64) {
        self.rc.borrow_mut().nda.fill(value)
    }
    fn into_shape(&self, shape: Vec<usize>) -> Self {
        let nda = self.rc.borrow().nda.clone().into_shape(IxDyn(&shape)).unwrap();
        Self { rc: RefCell::new(RsFloat64 { nda }) }
    }
    fn sum(&self) -> f64 {
        self.rc.borrow().nda.sum()
    }
    fn mean(&self) -> Option<f64> {
        self.rc.borrow().nda.mean()
    }
    fn product(&self) -> f64 {
        self.rc.borrow().nda.product()
    }
    fn var(&self, ddof: f64) -> f64 {
        self.rc.borrow().nda.var(ddof)
    }
    fn std(&self, ddof: f64) -> f64 {
        self.rc.borrow().nda.std(ddof)
    }
}

#[magnus::init]
fn init() -> Result<(), Error> {
    let module = define_module("Rumo")?;


    let class_u8 = module.define_class("UInt8", Default::default())?;
    class_u8.define_singleton_method("_zeros", function!(UInt8::zeros, 1))?;
    class_u8.define_singleton_method("_ones", function!(UInt8::ones, 1))?;
    class_u8.define_method("fill", method!(UInt8::fill, 1))?;
    class_u8.define_method("shape", method!(UInt8::shape, 0))?;
    class_u8.define_method("ndim", method!(UInt8::ndim, 0))?;
    class_u8.define_method("length", method!(UInt8::len, 0))?;
    class_u8.define_method("size", method!(UInt8::len, 0))?;
    class_u8.define_method("_reshape", method!(UInt8::into_shape, 1))?;
    class_u8.define_method("sum", method!(UInt8::sum, 0))?;
    class_u8.define_method("mean", method!(UInt8::mean, 0))?;
    class_u8.define_method("prod", method!(UInt8::product, 0))?;
    class_u8.define_method("inspect", method!(UInt8::to_string, 0))?;

    let class_i8 = module.define_class("Int8", Default::default())?;
    class_i8.define_singleton_method("_zeros", function!(Int8::zeros, 1))?;
    class_i8.define_singleton_method("_ones", function!(Int8::ones, 1))?;
    class_i8.define_method("fill", method!(Int8::fill, 1))?;
    class_i8.define_method("shape", method!(Int8::shape, 0))?;
    class_i8.define_method("ndim", method!(Int8::ndim, 0))?;
    class_i8.define_method("length", method!(Int8::len, 0))?;
    class_i8.define_method("size", method!(Int8::len, 0))?;
    class_i8.define_method("_reshape", method!(Int8::into_shape, 1))?;
    class_i8.define_method("sum", method!(Int8::sum, 0))?;
    class_i8.define_method("mean", method!(Int8::mean, 0))?;
    class_i8.define_method("prod", method!(Int8::product, 0))?;
    class_i8.define_method("inspect", method!(Int8::to_string, 0))?;

    let class_u16 = module.define_class("UInt16", Default::default())?;
    class_u16.define_singleton_method("_zeros", function!(UInt16::zeros, 1))?;
    class_u16.define_singleton_method("_ones", function!(UInt16::ones, 1))?;
    class_u16.define_method("fill", method!(UInt16::fill, 1))?;
    class_u16.define_method("shape", method!(UInt16::shape, 0))?;
    class_u16.define_method("ndim", method!(UInt16::ndim, 0))?;
    class_u16.define_method("length", method!(UInt16::len, 0))?;
    class_u16.define_method("size", method!(UInt16::len, 0))?;
    class_u16.define_method("_reshape", method!(UInt16::into_shape, 1))?;
    class_u16.define_method("sum", method!(UInt16::sum, 0))?;
    class_u16.define_method("mean", method!(UInt16::mean, 0))?;
    class_u16.define_method("prod", method!(UInt16::product, 0))?;
    class_u16.define_method("inspect", method!(UInt16::to_string, 0))?;

    let class_i16 = module.define_class("Int16", Default::default())?;
    class_i16.define_singleton_method("_zeros", function!(Int16::zeros, 1))?;
    class_i16.define_singleton_method("_ones", function!(Int16::ones, 1))?;
    class_i16.define_method("fill", method!(Int16::fill, 1))?;
    class_i16.define_method("shape", method!(Int16::shape, 0))?;
    class_i16.define_method("ndim", method!(Int16::ndim, 0))?;
    class_i16.define_method("length", method!(Int16::len, 0))?;
    class_i16.define_method("size", method!(Int16::len, 0))?;
    class_i16.define_method("_reshape", method!(Int16::into_shape, 1))?;
    class_i16.define_method("sum", method!(Int16::sum, 0))?;
    class_i16.define_method("mean", method!(Int16::mean, 0))?;
    class_i16.define_method("prod", method!(Int16::product, 0))?;
    class_i16.define_method("inspect", method!(Int16::to_string, 0))?;

    let class_u32 = module.define_class("UInt32", Default::default())?;
    class_u32.define_singleton_method("_zeros", function!(UInt32::zeros, 1))?;
    class_u32.define_singleton_method("_ones", function!(UInt32::ones, 1))?;
    class_u32.define_method("fill", method!(UInt32::fill, 1))?;
    class_u32.define_method("shape", method!(UInt32::shape, 0))?;
    class_u32.define_method("ndim", method!(UInt32::ndim, 0))?;
    class_u32.define_method("length", method!(UInt32::len, 0))?;
    class_u32.define_method("size", method!(UInt32::len, 0))?;
    class_u32.define_method("_reshape", method!(UInt32::into_shape, 1))?;
    class_u32.define_method("sum", method!(UInt32::sum, 0))?;
    class_u32.define_method("mean", method!(UInt32::mean, 0))?;
    class_u32.define_method("prod", method!(UInt32::product, 0))?;
    class_u32.define_method("inspect", method!(UInt32::to_string, 0))?;

    let class_i32 = module.define_class("Int32", Default::default())?;
    class_i32.define_singleton_method("_zeros", function!(Int32::zeros, 1))?;
    class_i32.define_singleton_method("_ones", function!(Int32::ones, 1))?;
    class_i32.define_method("fill", method!(Int32::fill, 1))?;
    class_i32.define_method("shape", method!(Int32::shape, 0))?;
    class_i32.define_method("ndim", method!(Int32::ndim, 0))?;
    class_i32.define_method("length", method!(Int32::len, 0))?;
    class_i32.define_method("size", method!(Int32::len, 0))?;
    class_i32.define_method("_reshape", method!(Int32::into_shape, 1))?;
    class_i32.define_method("sum", method!(Int32::sum, 0))?;
    class_i32.define_method("mean", method!(Int32::mean, 0))?;
    class_i32.define_method("prod", method!(Int32::product, 0))?;
    class_i32.define_method("inspect", method!(Int32::to_string, 0))?;

    let class_u64 = module.define_class("UInt64", Default::default())?;
    class_u64.define_singleton_method("_zeros", function!(UInt64::zeros, 1))?;
    class_u64.define_singleton_method("_ones", function!(UInt64::ones, 1))?;
    class_u64.define_method("fill", method!(UInt64::fill, 1))?;
    class_u64.define_method("shape", method!(UInt64::shape, 0))?;
    class_u64.define_method("ndim", method!(UInt64::ndim, 0))?;
    class_u64.define_method("length", method!(UInt64::len, 0))?;
    class_u64.define_method("size", method!(UInt64::len, 0))?;
    class_u64.define_method("_reshape", method!(UInt64::into_shape, 1))?;
    class_u64.define_method("sum", method!(UInt64::sum, 0))?;
    class_u64.define_method("mean", method!(UInt64::mean, 0))?;
    class_u64.define_method("prod", method!(UInt64::product, 0))?;
    class_u64.define_method("inspect", method!(UInt64::to_string, 0))?;

    let class_i64 = module.define_class("Int64", Default::default())?;
    class_i64.define_singleton_method("_zeros", function!(Int64::zeros, 1))?;
    class_i64.define_singleton_method("_ones", function!(Int64::ones, 1))?;
    class_i64.define_method("fill", method!(Int64::fill, 1))?;
    class_i64.define_method("shape", method!(Int64::shape, 0))?;
    class_i64.define_method("ndim", method!(Int64::ndim, 0))?;
    class_i64.define_method("length", method!(Int64::len, 0))?;
    class_i64.define_method("size", method!(Int64::len, 0))?;
    class_i64.define_method("_reshape", method!(Int64::into_shape, 1))?;
    class_i64.define_method("sum", method!(Int64::sum, 0))?;
    class_i64.define_method("mean", method!(Int64::mean, 0))?;
    class_i64.define_method("prod", method!(Int64::product, 0))?;
    class_i64.define_method("inspect", method!(Int64::to_string, 0))?;

    let class_f32 = module.define_class("Float32", Default::default())?;
    class_f32.define_singleton_method("_zeros", function!(Float32::zeros, 1))?;
    class_f32.define_singleton_method("_ones", function!(Float32::ones, 1))?;
    class_f32.define_singleton_method("linspace", function!(Float32::linspace, 3))?;
    class_f32.define_singleton_method("_range", function!(Float32::range, 3))?;
    class_f32.define_method("fill", method!(Float32::fill, 1))?;
    class_f32.define_method("shape", method!(Float32::shape, 0))?;
    class_f32.define_method("ndim", method!(Float32::ndim, 0))?;
    class_f32.define_method("length", method!(Float32::len, 0))?;
    class_f32.define_method("size", method!(Float32::len, 0))?;
    class_f32.define_method("_reshape", method!(Float32::into_shape, 1))?;
    class_f32.define_method("sum", method!(Float32::sum, 0))?;
    class_f32.define_method("mean", method!(Float32::mean, 0))?;
    class_f32.define_method("prod", method!(Float32::product, 0))?;
    class_f32.define_method("var", method!(Float32::var, 1))?;
    class_f32.define_method("std", method!(Float32::std, 1))?;
    class_f32.define_method("inspect", method!(Float32::to_string, 0))?;

    let class_f64 = module.define_class("Float64", Default::default())?;
    class_f64.define_singleton_method("_zeros", function!(Float64::zeros, 1))?;
    class_f64.define_singleton_method("_ones", function!(Float64::ones, 1))?;
    class_f64.define_singleton_method("linspace", function!(Float64::linspace, 3))?;
    class_f64.define_singleton_method("_range", function!(Float64::range, 3))?;
    class_f64.define_method("fill", method!(Float64::fill, 1))?;
    class_f64.define_method("shape", method!(Float64::shape, 0))?;
    class_f64.define_method("ndim", method!(Float64::ndim, 0))?;
    class_f64.define_method("length", method!(Float64::len, 0))?;
    class_f64.define_method("size", method!(Float64::len, 0))?;
    class_f64.define_method("_reshape", method!(Float64::into_shape, 1))?;
    class_f64.define_method("sum", method!(Float64::sum, 0))?;
    class_f64.define_method("mean", method!(Float64::mean, 0))?;
    class_f64.define_method("prod", method!(Float64::product, 0))?;
    class_f64.define_method("var", method!(Float64::var, 1))?;
    class_f64.define_method("std", method!(Float64::std, 1))?;
    class_f64.define_method("inspect", method!(Float64::to_string, 0))?;


    eval::<bool>("Rumo::SFloat = Rumo::Float32")?;
    eval::<bool>("Rumo::DFloat = Rumo::Float64")?;

    Ok(())
}
