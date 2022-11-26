require 'erb'
TYPES = [
    ["UInt8", "u8"],
    ["Int8", "i8"],
    ["UInt16", "u16"],
    ["Int16", "i16"],
    ["UInt32", "u32"],
    ["Int32", "i32"],
    ["UInt64", "u64"],
    ["Int64", "i64"],
    ["Float32", "f32"],
    ["Float64", "f64"],
]
code = ERB.new(DATA.read, trim_mode: "-").result(binding)
Dir.chdir(__dir__) { File.write("lib.rs", code) }
__END__
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

<%- TYPES.each do |type, rust_type| -%>

struct Rs<%= type %> {
    nda: ArrayD<<%= rust_type %>>,
}

#[magnus::wrap(class = "Rumo::<%= type %>")]
struct <%= type %> {
    rc: RefCell<Rs<%= type %>>,
}

impl NArray for <%= type %> {
    fn zeros(dims: Vec<usize>) -> Self {
        let nda = ArrayD::<<%= rust_type %>>::zeros(IxDyn(&dims));
        Self { rc: RefCell::new(Rs<%= type %> { nda }) }
    }
    fn ones(dims: Vec<usize>) -> Self {
        let nda = ArrayD::<<%= rust_type %>>::ones(IxDyn(&dims));
        Self { rc: RefCell::new(Rs<%= type %> { nda }) }
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

impl <%= type %> {
    <%- if type =~ /Float/ -%>
    fn linspace(start: <%= rust_type %>, end: <%= rust_type %>, n: usize) -> Self {
        let nda = Array1::<<%= rust_type %>>::linspace(start, end, n).into_dyn();
        Self { rc: RefCell::new(Rs<%= type %> { nda }) }
    }
    fn range(start: <%= rust_type %>, end: <%= rust_type %>, step: <%= rust_type %>) -> Self {
        let nda = Array1::<<%= rust_type %>>::range(start, end, step).into_dyn();
        Self { rc: RefCell::new(Rs<%= type %> { nda }) }
    }
    <%- end -%>
    fn fill(&self, value: <%= rust_type %>) {
        self.rc.borrow_mut().nda.fill(value)
    }
    fn sum(&self) -> <%= rust_type %> {
        self.rc.borrow().nda.sum()
    }
    fn mean(&self) -> Option<<%= rust_type %>> {
        self.rc.borrow().nda.mean()
    }
    fn product(&self) -> <%= rust_type %> {
        self.rc.borrow().nda.product()
    }
    <%- if type =~ /Float/ -%>
    fn var(&self, ddof: <%= rust_type %>) -> <%= rust_type %> {
        self.rc.borrow().nda.var(ddof)
    }
    fn std(&self, ddof: <%= rust_type %>) -> <%= rust_type %> {
        self.rc.borrow().nda.std(ddof)
    }
    <%- end -%>
}
<%- end -%>

#[magnus::init]
fn init() -> Result<(), Error> {
    let module = define_module("Rumo")?;

<% TYPES.each do |type, rust_type| %>
    let class_<%= rust_type %> = module.define_class("<%= type %>", Default::default())?;
    class_<%= rust_type %>.define_singleton_method("_zeros", function!(<%= type %>::zeros, 1))?;
    class_<%= rust_type %>.define_singleton_method("_ones", function!(<%= type %>::ones, 1))?;
    <%- if type =~ /Float/ -%>
    class_<%= rust_type %>.define_singleton_method("linspace", function!(<%= type %>::linspace, 3))?;
    class_<%= rust_type %>.define_singleton_method("_range", function!(<%= type %>::range, 3))?;
    <%- end -%>
    class_<%= rust_type %>.define_method("shape", method!(<%= type %>::shape, 0))?;
    class_<%= rust_type %>.define_method("ndim", method!(<%= type %>::ndim, 0))?;
    class_<%= rust_type %>.define_method("length", method!(<%= type %>::len, 0))?;
    class_<%= rust_type %>.define_method("size", method!(<%= type %>::len, 0))?;
    class_<%= rust_type %>.define_method("fill", method!(<%= type %>::fill, 1))?;
    class_<%= rust_type %>.define_method("sum", method!(<%= type %>::sum, 0))?;
    class_<%= rust_type %>.define_method("mean", method!(<%= type %>::mean, 0))?;
    class_<%= rust_type %>.define_method("prod", method!(<%= type %>::product, 0))?;
    <%- if type =~ /Float/ -%>
    class_<%= rust_type %>.define_method("var", method!(<%= type %>::var, 1))?;
    class_<%= rust_type %>.define_method("std", method!(<%= type %>::std, 1))?;
    <%- end -%>
    class_<%= rust_type %>.define_method("inspect", method!(<%= type %>::to_string, 0))?;
<% end %>

    eval::<bool>("Rumo::SFloat = Rumo::Float32")?;
    eval::<bool>("Rumo::DFloat = Rumo::Float64")?;

    Ok(())
}
