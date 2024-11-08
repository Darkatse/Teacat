use std::fs;
use std::collections::HashMap;
use tauri::{path::BaseDirectory, AppHandle, Manager};
use crate::parsers::atom_config::{Atom, AtomConfig};

/// 从配置文件中加载原子的半径和颜色
pub fn load_atom_config(app_handle: &AppHandle) -> HashMap<String, AtomConfig> {
    let config_path = app_handle
        .path()
        .resolve("resources/atom_config.json", BaseDirectory::Resource)
        .expect("Failed to resolve atom config file path");

    // 读取配置文件内容
    // reference:
    // Atom Radius: https://crystalmaker.com/support/tutorials/atomic-radii/index.html
    // Color: https://jmol.sourceforge.net/jscolors/
    let config_content = fs::read_to_string(config_path).expect("Unable to read atom config file");
    let config: HashMap<String, AtomConfig> =
        serde_json::from_str(&config_content).expect("Invalid JSON format in atom config");
    config
}

/// 判断两个浮点数是否接近
fn is_close(value: f64, target: f64, epsilon: f64) -> bool {
    (value - target).abs() < epsilon
}

/// 复制位于晶格边界上的原子
pub fn replicate_boundary_atoms(atoms: &mut Vec<Atom>) {
    let mut new_atoms = Vec::new();
    let epsilon = 0.01;

    for atom in atoms.iter() {
        let mut replicas = vec![(0.0, 0.0, 0.0)]; // 记录原子的复制偏移向量

        // 如果原子位于 x 边界上，复制到相邻晶格
        if is_close(atom.x, 0.0, epsilon) {
            replicas.push((1.0, 0.0, 0.0));
        } else if is_close(atom.x, 1.0, epsilon) {
            replicas.push((-1.0, 0.0, 0.0));
        }

        // 如果原子位于 y 边界上，复制到相邻晶格
        if is_close(atom.y, 0.0, epsilon) {
            replicas.push((0.0, 1.0, 0.0));
        } else if is_close(atom.y, 1.0, epsilon) {
            replicas.push((0.0, -1.0, 0.0));
        }

        // 如果原子位于 z 边界上，复制到相邻晶格
        if is_close(atom.z, 0.0, epsilon) {
            replicas.push((0.0, 0.0, 1.0));
        } else if is_close(atom.z, 1.0, epsilon) {
            replicas.push((0.0, 0.0, -1.0));
        }

        // 根据复制偏移向量，复制原子
        for (dx, dy, dz) in replicas {
            let mut new_atom = atom.clone();
            new_atom.x = atom.x + dx;
            new_atom.y = atom.y + dy;
            new_atom.z = atom.z + dz;
            new_atoms.push(new_atom);
        }
    }

    // 将新复制的原子添加到原子列表中
    atoms.extend(new_atoms);
}