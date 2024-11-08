use regex::Regex;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use tauri::AppHandle;
use tauri_plugin_dialog::FilePath;
use crate::parsers::atom_config::{CrystalStructure, Atom, AtomConfig};
use crate::utils::geometry::{calculate_lattice_vertices,get_cartesian_coordinates,get_lattice_center};
use crate::parsers::parser_utils::{load_atom_config, replicate_boundary_atoms};


/// 从文件中解析晶体结构
pub fn parse_cif(app_handle: &AppHandle, file_path: &FilePath) -> CrystalStructure {
    // 读取 CIF 文件内容
    let file_content = match file_path {
        FilePath::Path(path_str) => {
            fs::read_to_string(PathBuf::from(path_str)).expect("Unable to read file")
        }
        _ => panic!("Invalid file path"),
    };

    // 初始化晶体参数
    let mut a: f64 = 0.0;
    let mut b: f64 = 0.0;
    let mut c: f64 = 0.0;
    let mut alpha: f64 = 0.0;
    let mut beta: f64 = 0.0;
    let mut gamma: f64 = 0.0;
    let mut atoms: Vec<Atom> = Vec::new();

    // 正则表达式匹配原子信息的行
    let atom_line_regex = Regex::new(r"^\s*[A-Za-z]+\s+[A-Za-z]+\d").unwrap();

    // 配置文件路径（假设在程序根目录下）
    let atom_config = load_atom_config(app_handle);

    // 解析 CIF 文件的每一行
    for line in file_content.lines() {
        // 提取晶格参数
        if line.starts_with("_cell_length_a") {
            a = extract_value_from_line(line);
        } else if line.starts_with("_cell_length_b") {
            b = extract_value_from_line(line);
        } else if line.starts_with("_cell_length_c") {
            c = extract_value_from_line(line);
        } else if line.starts_with("_cell_angle_alpha") {
            alpha = extract_value_from_line(line);
        } else if line.starts_with("_cell_angle_beta") {
            beta = extract_value_from_line(line);
        } else if line.starts_with("_cell_angle_gamma") {
            gamma = extract_value_from_line(line);
        }

        // 提取原子信息
        if atom_line_regex.is_match(line) {
            let atom = parse_atom_line(line, &atom_config);
            atoms.push(atom);
        }
    }

    // 复制位于边界的原子
    replicate_boundary_atoms(&mut atoms);

    // 将所有原子坐标转换为笛卡尔坐标
    atoms.iter_mut().for_each(|atom| {
        (atom.x, atom.y, atom.z) =
            get_cartesian_coordinates(a, b, c, alpha, beta, gamma, atom.x, atom.y, atom.z);
    });

    // 计算晶格顶点
    let mut lattice_vertices = calculate_lattice_vertices(a, b, c, alpha, beta, gamma);

    // 计算晶格中心
    let center = get_lattice_center(&lattice_vertices);

    // 平移原子，使晶格的中心位于 (0,0,0)
    atoms.iter_mut().for_each(|atom| {
        atom.x -= center.0;
        atom.y -= center.1;
        atom.z -= center.2;
    });

    // 平移晶格顶点，使晶格的中心位于 (0,0,0)
    lattice_vertices.iter_mut().for_each(|vertex| {
        vertex.0 -= center.0;
        vertex.1 -= center.1;
        vertex.2 -= center.2;
    });

    // 返回解析后的晶体结构
    CrystalStructure {
        lattice_vertices,
        atoms,
    }
}


/// 从 CIF 文件中的一行提取数值
fn extract_value_from_line(line: &str) -> f64 {
    let parts: Vec<&str> = line.split_whitespace().collect();
    parts[1].parse::<f64>().expect("Failed to parse value")
}


/// 解析原子行并返回 Atom 结构
fn parse_atom_line(line: &str, atom_config: &HashMap<String, AtomConfig>) -> Atom {
    let parts: Vec<&str> = line.split_whitespace().collect();
    let name = parts[0].to_string();
    let frac_x = parts[3]
        .parse::<f64>()
        .expect("Failed to parse x coordinate");
    let frac_y = parts[4]
        .parse::<f64>()
        .expect("Failed to parse y coordinate");
    let frac_z = parts[5]
        .parse::<f64>()
        .expect("Failed to parse z coordinate");

    // 查找原子的绘制半径和颜色
    let _config = atom_config.get(&name).cloned().unwrap_or(AtomConfig {
        radius: 0.35,
        color: "#505050".to_string(),
    }); // 如果没有找到，使用默认值

    Atom {
        name,
        x: frac_x,
        y: frac_y,
        z: frac_z,
        radius: _config.radius,
        color: _config.color,
    }
}
