use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use tauri::{path::BaseDirectory, AppHandle, Manager};
use tauri_plugin_dialog::FilePath;

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
struct AtomConfig {
    radius: f64,
    color: String,
}

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

/// 从配置文件中加载原子的半径和颜色
fn load_atom_config(app_handle: &AppHandle) -> HashMap<String, AtomConfig> {
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

/// 从 CIF 文件中的一行提取数值
fn extract_value_from_line(line: &str) -> f64 {
    let parts: Vec<&str> = line.split_whitespace().collect();
    parts[1].parse::<f64>().expect("Failed to parse value")
}

/// 计算晶格的基矢量
fn calculate_basis_vectors(
    a: f64,
    b: f64,
    c: f64,
    alpha: f64,
    beta: f64,
    gamma: f64,
) -> [(f64, f64, f64); 3] {
    let cos_alpha = alpha.to_radians().cos();
    let cos_beta = beta.to_radians().cos();
    let cos_gamma = gamma.to_radians().cos();
    let sin_gamma = gamma.to_radians().sin();

    let ax = a;
    let ay = 0.0;
    let az = 0.0;

    let bx = b * cos_gamma;
    let by = b * sin_gamma;
    let bz = 0.0;

    let cx = c * cos_beta;
    let cy = c * (cos_alpha - cos_beta * cos_gamma) / sin_gamma;
    let cz = (c * c - cx * cx - cy * cy).sqrt();

    [(ax, ay, az), (bx, by, bz), (cx, cy, cz)]
}

/// 解析并转换晶体结构的原子位置为笛卡尔坐标
fn get_cartesian_coordinates(
    a: f64,
    b: f64,
    c: f64,
    alpha: f64,
    beta: f64,
    gamma: f64,
    frac_x: f64,
    frac_y: f64,
    frac_z: f64,
) -> (f64, f64, f64) {
    // 获取基矢量
    let basis_vectors = calculate_basis_vectors(a, b, c, alpha, beta, gamma);

    // 转换为笛卡尔坐标
    let x = frac_x * basis_vectors[0].0 + frac_y * basis_vectors[1].0 + frac_z * basis_vectors[2].0;
    let y = frac_x * basis_vectors[0].1 + frac_y * basis_vectors[1].1 + frac_z * basis_vectors[2].1;
    let z = frac_x * basis_vectors[0].2 + frac_y * basis_vectors[1].2 + frac_z * basis_vectors[2].2;

    (x, y, z)
}

/// 计算晶格顶点
fn calculate_lattice_vertices(
    a: f64,
    b: f64,
    c: f64,
    alpha: f64,
    beta: f64,
    gamma: f64,
) -> Vec<(f64, f64, f64)> {
    // 获取基矢量
    let basis_vectors = calculate_basis_vectors(a, b, c, alpha, beta, gamma);

    vec![
        (0.0, 0.0, 0.0),
        basis_vectors[0], // (ax, ay, az)
        (
            basis_vectors[0].0 + basis_vectors[1].0,
            basis_vectors[0].1 + basis_vectors[1].1,
            basis_vectors[0].2 + basis_vectors[1].2,
        ),
        basis_vectors[1], // (bx, by, bz)
        basis_vectors[2], // (cx, cy, cz)
        (
            basis_vectors[0].0 + basis_vectors[2].0,
            basis_vectors[0].1 + basis_vectors[2].1,
            basis_vectors[0].2 + basis_vectors[2].2,
        ),
        (
            basis_vectors[0].0 + basis_vectors[1].0 + basis_vectors[2].0,
            basis_vectors[0].1 + basis_vectors[1].1 + basis_vectors[2].1,
            basis_vectors[0].2 + basis_vectors[1].2 + basis_vectors[2].2,
        ),
        (
            basis_vectors[1].0 + basis_vectors[2].0,
            basis_vectors[1].1 + basis_vectors[2].1,
            basis_vectors[1].2 + basis_vectors[2].2,
        ),
    ]
}

/// 计算晶格的中心点
fn get_lattice_center(vertices: &Vec<(f64, f64, f64)>) -> (f64, f64, f64) {
    let (mut x_sum, mut y_sum, mut z_sum) = (0.0, 0.0, 0.0);
    vertices.iter().for_each(|(x, y, z)| {
        x_sum += x;
        y_sum += y;
        z_sum += z;
    });
    let count = vertices.len() as f64;
    (x_sum / count, y_sum / count, z_sum / count)
}

/// 判断两个浮点数是否接近
fn is_close(value: f64, target: f64, epsilon: f64) -> bool {
    (value - target).abs() < epsilon
}

/// 复制位于晶格边界上的原子
fn replicate_boundary_atoms(atoms: &mut Vec<Atom>) {
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
