use crate::render::{color::ColorSource, render_graph::BindType};
use legion::prelude::Entity;
use std::collections::HashMap;

// TODO: add ability to specify specific pipeline for uniforms
pub trait AsUniforms {
    fn get_uniform_infos(&self) -> &[UniformInfo];
    fn get_uniform_bytes(&self, name: &str) -> Option<Vec<u8>>;
    fn get_shader_defs(&self) -> Option<Vec<String>>;
    fn get_field_bind_type(&self, name: &str) -> FieldBindType;
    // TODO: support zero-copy uniforms
    // fn get_uniform_bytes_ref(&self, name: &str) -> Option<&[u8]>;
}

pub trait ShaderDefSuffixProvider {
    fn get_shader_def(&self) -> Option<&'static str>;
}

impl ShaderDefSuffixProvider for bool {
    fn get_shader_def(&self) -> Option<&'static str> {
        match *self {
            true => Some(""),
            false => None,
        }
    }
}

pub enum FieldBindType {
    Uniform,
    Texture,
}

pub trait AsFieldBindType {
    fn get_field_uniform_type(&self) -> FieldBindType;
}

impl AsFieldBindType for ColorSource {
    fn get_field_uniform_type(&self) -> FieldBindType {
        match *self {
            ColorSource::Texture(_) => FieldBindType::Texture,
            ColorSource::Color(_) => FieldBindType::Uniform,
        }
    }
}

// impl<T> AsFieldBindType for T
// where
//     T: GetBytes,
// {
//     fn get_field_uniform_type(&self) -> FieldBindType {
//         FieldBindType::Uniform
//     }
// }

pub struct UniformInfo<'a> {
    pub name: &'a str,
    pub bind_type: BindType,
}

pub struct DynamicUniformBufferInfo {
    pub indices: HashMap<usize, Entity>,
    pub offsets: HashMap<Entity, u64>,
    pub capacity: u64,
    pub count: u64,
}

impl DynamicUniformBufferInfo {
    pub fn new() -> Self {
        DynamicUniformBufferInfo {
            capacity: 0,
            count: 0,
            indices: HashMap::new(),
            offsets: HashMap::new(),
        }
    }
}