use regex::Regex;
use std::fs;
use std::path::PathBuf;
use crate::parsers::atom_config::{CrystalStructure, Atom, AtomConfig};
use crate::parsers::parser_utils::{load_atom_config, replicate_boundary_atoms};

/// 解析 Quantum ESPRESSO 输出文件并返回多个晶体结构
pub fn parse_qe_output(file_path: &PathBuf) -> Vec<CrystalStructure> {
    // 正则表达式
    let cell_pattern = Regex::new(r"CELL_PARAMETERS\s+\(alat\s*=\s*([\d\.]+)\)").unwrap();
    let pos_pattern = Regex::new(r"ATOMIC_POSITIONS\s+\(([a-zA-Z]+)\)").unwrap();

    // 存储解析结果的数组
    let mut structures = Vec::new();

    // 打开文件并逐行读取
    let file_content = fs::read_to_string(file_path).expect("Unable to read Quantum ESPRESSO output file");

    let mut current_cell: Option<[[f64; 3]; 3]> = None;
    let mut current_atoms: Vec<Atom> = Vec::new();
    let mut alat = 1.0; // 初始化 alat 为 1，之后从文件读取

    for line in file_content.lines() {
        // 匹配 CELL_PARAMETERS 并读取晶格参数
        if let Some(captures) = cell_pattern.captures(line) {
            alat = captures[1]
                .parse::<f64>()
                .expect("Failed to parse alat from CELL_PARAMETERS");
            current_cell = Some(parse_cell_parameters(&file_content, line));
        }

        // 匹配 ATOMIC_POSITIONS 并读取原子数据
        if let Some(_) = pos_pattern.captures(line) {
            current_atoms = parse_atomic_positions(&file_content, line);

            // 如果当前有 cell 和 atoms，生成 CrystalStructure 并存储
            if let Some(cell) = current_cell.take() {
                let lattice_vertices = calculate_lattice_vertices(cell);
                let atoms = convert_to_cartesian(current_atoms.clone(), cell, alat); // 转换为笛卡尔坐标
                structures.push(CrystalStructure {
                    lattice_vertices,
                    atoms,
                });
            }
        }
    }

    structures
}

/// 解析 CELL_PARAMETERS 的三行数据
fn parse_cell_parameters(file_content: &str, line: &str) -> [[f64; 3]; 3] {
    let mut cell = [[0.0; 3]; 3];
    let mut lines = file_content.lines().skip_while(|l| *l != line).skip(1);

    for i in 0..3 {
        if let Some(l) = lines.next() {
            let values: Vec<f64> = l
                .split_whitespace()
                .map(|v| v.parse::<f64>().expect("Failed to parse cell parameter"))
                .collect();
            cell[i] = [values[0], values[1], values[2]];
        }
    }

    cell
}

/// 解析 ATOMIC_POSITIONS 的多行数据
fn parse_atomic_positions(file_content: &str, line: &str) -> Vec<Atom> {
    let mut atoms = Vec::new();
    let mut lines = file_content.lines().skip_while(|l| *l != line).skip(1);

    for l in lines {
        let l = l.trim();
        if l.is_empty() || l.contains("End final coordinates") {
            break;
        }

        let parts: Vec<&str> = l.split_whitespace().collect();
        let name = parts[0].to_string();
        let x = parts[1].parse::<f64>().expect("Failed to parse x coordinate");
        let y = parts[2].parse::<f64>().expect("Failed to parse y coordinate");
        let z = parts[3].parse::<f64>().expect("Failed to parse z coordinate");

        let atom = Atom {
            name,
            x,
            y,
            z,
            radius: 0.5, // 默认值，后续可以根据原子名称设置
            color: "#FFFFFF".to_string(), // 默认颜色
        };

        atoms.push(atom);
    }

    atoms
}

/// 将原子的分数坐标转换为笛卡尔坐标
fn convert_to_cartesian(atoms: Vec<Atom>, cell: [[f64; 3]; 3], alat: f64) -> Vec<Atom> {
    atoms
        .into_iter()
        .map(|mut atom| {
            let frac_x = atom.x;
            let frac_y = atom.y;
            let frac_z = atom.z;

            // 笛卡尔坐标 = 分数坐标 * 基矢量 * alat
            atom.x = frac_x * cell[0][0] * alat
                + frac_y * cell[1][0] * alat
                + frac_z * cell[2][0] * alat;
            atom.y = frac_x * cell[0][1] * alat
                + frac_y * cell[1][1] * alat
                + frac_z * cell[2][1] * alat;
            atom.z = frac_x * cell[0][2] * alat
                + frac_y * cell[1][2] * alat
                + frac_z * cell[2][2] * alat;

            atom
        })
        .collect()
}

/// 计算晶格顶点
fn calculate_lattice_vertices(cell: [[f64; 3]; 3]) -> Vec<(f64, f64, f64)> {
    let a = cell[0];
    let b = cell[1];
    let c = cell[2];

    vec![
        (0.0, 0.0, 0.0),
        (a[0], a[1], a[2]),
        (a[0] + b[0], a[1] + b[1], a[2] + b[2]),
        (b[0], b[1], b[2]),
        (c[0], c[1], c[2]),
        (a[0] + c[0], a[1] + c[1], a[2] + c[2]),
        (a[0] + b[0] + c[0], a[1] + b[1] + c[1], a[2] + b[2] + c[2]),
        (b[0] + c[0], b[1] + c[1], b[2] + c[2]),
    ]
}