extern crate magnus;
extern crate ndarray;
use ndarray::{array, prelude::*, Array, ArrayD, ArrayView, IxDyn};

use magnus::{define_class, define_module, eval, function, method, prelude::*, Error};

#[magnus::wrap(class = "Rumo::UInt8")]
struct UInt8 {
    nda: ArrayD<u8>,
}

impl UInt8 {
    fn new(dims: Vec<usize>) -> Self {
        let mut nda = ArrayD::<u8>::zeros(IxDyn(&dims));
        Self { nda }
    }
}

#[magnus::wrap(class = "Rumo::Int8")]
struct Int8 {
    nda: ArrayD<i8>,
}

impl Int8 {
    fn new(dims: Vec<usize>) -> Self {
        let mut nda = ArrayD::<i8>::zeros(IxDyn(&dims));
        Self { nda }
    }
}

#[magnus::wrap(class = "Rumo::UInt16")]
struct UInt16 {
    nda: ArrayD<u16>,
}

impl UInt16 {
    fn new(dims: Vec<usize>) -> Self {
        let mut nda = ArrayD::<u16>::zeros(IxDyn(&dims));
        Self { nda }
    }
}

#[magnus::wrap(class = "Rumo::Int16")]
struct Int16 {
    nda: ArrayD<i16>,
}

impl Int16 {
    fn new(dims: Vec<usize>) -> Self {
        let mut nda = ArrayD::<i16>::zeros(IxDyn(&dims));
        Self { nda }
    }
}

#[magnus::wrap(class = "Rumo::UInt32")]
struct UInt32 {
    nda: ArrayD<u32>,
}

impl UInt32 {
    fn new(dims: Vec<usize>) -> Self {
        let mut nda = ArrayD::<u32>::zeros(IxDyn(&dims));
        Self { nda }
    }
}

#[magnus::wrap(class = "Rumo::Int32")]
struct Int32 {
    nda: ArrayD<i32>,
}

impl Int32 {
    fn new(dims: Vec<usize>) -> Self {
        let mut nda = ArrayD::<i32>::zeros(IxDyn(&dims));
        Self { nda }
    }
}

#[magnus::wrap(class = "Rumo::UInt64")]
struct UInt64 {
    nda: ArrayD<u64>,
}

impl UInt64 {
    fn new(dims: Vec<usize>) -> Self {
        let mut nda = ArrayD::<u64>::zeros(IxDyn(&dims));
        Self { nda }
    }
}

#[magnus::wrap(class = "Rumo::Int64")]
struct Int64 {
    nda: ArrayD<i64>,
}

impl Int64 {
    fn new(dims: Vec<usize>) -> Self {
        let mut nda = ArrayD::<i64>::zeros(IxDyn(&dims));
        Self { nda }
    }
}

#[magnus::wrap(class = "Rumo::Float32")]
struct Float32 {
    nda: ArrayD<f32>,
}

impl Float32 {
    fn new(dims: Vec<usize>) -> Self {
        let mut nda = ArrayD::<f32>::zeros(IxDyn(&dims));
        Self { nda }
    }
}

#[magnus::wrap(class = "Rumo::Float64")]
struct Float64 {
    nda: ArrayD<f64>,
}

impl Float64 {
    fn new(dims: Vec<usize>) -> Self {
        let mut nda = ArrayD::<f64>::zeros(IxDyn(&dims));
        Self { nda }
    }
}

#[magnus::init]
fn init() -> Result<(), Error> {
    let module = define_module("Rumo")?;
    let class_u8 = module.define_class("UInt8", Default::default())?;
    class_u8.define_singleton_method("new", function!(UInt8::new, 1))?;

    let class_i8 = module.define_class("Int8", Default::default())?;
    class_i8.define_singleton_method("new", function!(Int8::new, 1))?;

    let class_u16 = module.define_class("UInt16", Default::default())?;
    class_u16.define_singleton_method("new", function!(UInt16::new, 1))?;

    let class_i16 = module.define_class("Int16", Default::default())?;
    class_i16.define_singleton_method("new", function!(Int16::new, 1))?;

    let class_u32 = module.define_class("UInt32", Default::default())?;
    class_u32.define_singleton_method("new", function!(UInt32::new, 1))?;

    let class_i32 = module.define_class("Int32", Default::default())?;
    class_i32.define_singleton_method("new", function!(Int32::new, 1))?;

    let class_u64 = module.define_class("UInt64", Default::default())?;
    class_u64.define_singleton_method("new", function!(UInt64::new, 1))?;

    let class_i64 = module.define_class("Int64", Default::default())?;
    class_i64.define_singleton_method("new", function!(Int64::new, 1))?;

    let class_f32 = module.define_class("Float32", Default::default())?;
    class_f32.define_singleton_method("new", function!(Float32::new, 1))?;

    let class_f64 = module.define_class("Float64", Default::default())?;
    class_f64.define_singleton_method("new", function!(Float64::new, 1))?;
    
    eval::<bool>("Rumo::SFloat = Rumo::Float32")?;
    eval::<bool>("Rumo::DFloat = Rumo::Float64")?;
    
    Ok(())
}
