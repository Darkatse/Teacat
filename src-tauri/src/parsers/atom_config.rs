use serde::{Deserialize, Serialize};

// 定义晶体结构的返回类型
#[derive(Serialize, Clone, Debug)]
pub struct CrystalStructure {
    pub lattice_vertices: Vec<(f64, f64, f64)>, // 返回晶格顶点
    pub atoms: Vec<Atom>,
}

#[derive(Serialize, Clone, Debug)]
pub struct Atom {
    pub name: String,
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub radius: f64,
    pub color: String,
}

// 定义 AtomConfig 结构体
#[derive(Deserialize, Clone)]
pub struct AtomConfig {
    pub radius: f64,
    pub color: String,
}