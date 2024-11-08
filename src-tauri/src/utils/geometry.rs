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
pub fn get_cartesian_coordinates(
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
pub fn calculate_lattice_vertices(
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
pub fn get_lattice_center(vertices: &Vec<(f64, f64, f64)>) -> (f64, f64, f64) {
    let (mut x_sum, mut y_sum, mut z_sum) = (0.0, 0.0, 0.0);
    vertices.iter().for_each(|(x, y, z)| {
        x_sum += x;
        y_sum += y;
        z_sum += z;
    });
    let count = vertices.len() as f64;
    (x_sum / count, y_sum / count, z_sum / count)
}