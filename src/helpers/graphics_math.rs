use crate::models::{Point3D, Radians};

// using f32 instead of f64 because webgl api uniform_matrix4fv_with_f32_array
// receives uniform array of f32 only
pub type Matrix4 = [f32; 4 * 4];
pub type Vec3 = [f32; 3];

/// Takes two 4-by-4 matrices, a and b, and computes the product in the order
/// that pre-composes b with a.  In other words, the matrix returned will
/// transform by b first and then a.  Note this is subtly different from just
/// multiplying the matrices together.  For given a and b, this function returns
/// the same object in both row-major and column-major mode.
pub fn multiply(a: Matrix4, b: Matrix4) -> Matrix4 {
    let mut res: Matrix4 = [0.0; 16];

    #[allow(clippy::erasing_op, clippy::identity_op)]
    {
        let b00 = b[0 * 4 + 0];
        let b01 = b[0 * 4 + 1];
        let b02 = b[0 * 4 + 2];
        let b03 = b[0 * 4 + 3];
        let b10 = b[1 * 4 + 0];
        let b11 = b[1 * 4 + 1];
        let b12 = b[1 * 4 + 2];
        let b13 = b[1 * 4 + 3];
        let b20 = b[2 * 4 + 0];
        let b21 = b[2 * 4 + 1];
        let b22 = b[2 * 4 + 2];
        let b23 = b[2 * 4 + 3];
        let b30 = b[3 * 4 + 0];
        let b31 = b[3 * 4 + 1];
        let b32 = b[3 * 4 + 2];
        let b33 = b[3 * 4 + 3];
        let a00 = a[0 * 4 + 0];
        let a01 = a[0 * 4 + 1];
        let a02 = a[0 * 4 + 2];
        let a03 = a[0 * 4 + 3];
        let a10 = a[1 * 4 + 0];
        let a11 = a[1 * 4 + 1];
        let a12 = a[1 * 4 + 2];
        let a13 = a[1 * 4 + 3];
        let a20 = a[2 * 4 + 0];
        let a21 = a[2 * 4 + 1];
        let a22 = a[2 * 4 + 2];
        let a23 = a[2 * 4 + 3];
        let a30 = a[3 * 4 + 0];
        let a31 = a[3 * 4 + 1];
        let a32 = a[3 * 4 + 2];
        let a33 = a[3 * 4 + 3];

        res[0] = b00 * a00 + b01 * a10 + b02 * a20 + b03 * a30;
        res[1] = b00 * a01 + b01 * a11 + b02 * a21 + b03 * a31;
        res[2] = b00 * a02 + b01 * a12 + b02 * a22 + b03 * a32;
        res[3] = b00 * a03 + b01 * a13 + b02 * a23 + b03 * a33;
        res[4] = b10 * a00 + b11 * a10 + b12 * a20 + b13 * a30;
        res[5] = b10 * a01 + b11 * a11 + b12 * a21 + b13 * a31;
        res[6] = b10 * a02 + b11 * a12 + b12 * a22 + b13 * a32;
        res[7] = b10 * a03 + b11 * a13 + b12 * a23 + b13 * a33;
        res[8] = b20 * a00 + b21 * a10 + b22 * a20 + b23 * a30;
        res[9] = b20 * a01 + b21 * a11 + b22 * a21 + b23 * a31;
        res[10] = b20 * a02 + b21 * a12 + b22 * a22 + b23 * a32;
        res[11] = b20 * a03 + b21 * a13 + b22 * a23 + b23 * a33;
        res[12] = b30 * a00 + b31 * a10 + b32 * a20 + b33 * a30;
        res[13] = b30 * a01 + b31 * a11 + b32 * a21 + b33 * a31;
        res[14] = b30 * a02 + b31 * a12 + b32 * a22 + b33 * a32;
        res[15] = b30 * a03 + b31 * a13 + b32 * a23 + b33 * a33;
    }

    res
}

pub fn normalize(v: Vec3) -> Vec3 {
    let mut res: Vec3 = [0.0; 3];

    let length = (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt();

    // make sure we don't divide by 0.
    static DELTA: f32 = 0.00001;
    if length > DELTA {
        res[0] = v[0] / length;
        res[1] = v[1] / length;
        res[2] = v[2] / length;
    }

    res
}

pub fn subtract_vectors(a: Vec3, b: Vec3) -> Vec3 {
    let mut res: Vec3 = [0.0; 3];

    res[0] = a[0] - b[0];
    res[1] = a[1] - b[1];
    res[2] = a[2] - b[2];

    res
}

/// Computes the cross product of 2 vectors
pub fn cross(a: Vec3, b: Vec3) -> Vec3 {
    let mut res: Vec3 = [0.0; 3];

    res[0] = a[1] * b[2] - a[2] * b[1];
    res[1] = a[2] * b[0] - a[0] * b[2];
    res[2] = a[0] * b[1] - a[1] * b[0];

    res
}

/// Creates a lookAt matrix.
///
/// This is a world matrix for a camera. In other words it will transform
/// from the origin to a place and orientation in the world. For a view
/// matrix take the inverse of this.
pub fn look_at(camera_pos: Vec3, target: Vec3, up: Vec3) -> Matrix4 {
    let mut res: Matrix4 = [0.0; 16];

    let z_axis = normalize(subtract_vectors(camera_pos, target));
    let x_axis = normalize(cross(up, z_axis));
    let y_axis = normalize(cross(z_axis, x_axis));

    res[0] = x_axis[0];
    res[1] = x_axis[1];
    res[2] = x_axis[2];
    res[3] = 0.0;
    res[4] = y_axis[0];
    res[5] = y_axis[1];
    res[6] = y_axis[2];
    res[7] = 0.0;
    res[8] = z_axis[0];
    res[9] = z_axis[1];
    res[10] = z_axis[2];
    res[11] = 0.0;
    res[12] = camera_pos[0];
    res[13] = camera_pos[1];
    res[14] = camera_pos[2];
    res[15] = 1.0;

    res
}

pub fn inverse(m: Matrix4) -> Matrix4 {
    let mut res: Matrix4 = [0.0; 16];

    #[allow(clippy::erasing_op, clippy::identity_op)]
    {
        let m00 = m[0 * 4 + 0];
        let m01 = m[0 * 4 + 1];
        let m02 = m[0 * 4 + 2];
        let m03 = m[0 * 4 + 3];
        let m10 = m[1 * 4 + 0];
        let m11 = m[1 * 4 + 1];
        let m12 = m[1 * 4 + 2];
        let m13 = m[1 * 4 + 3];
        let m20 = m[2 * 4 + 0];
        let m21 = m[2 * 4 + 1];
        let m22 = m[2 * 4 + 2];
        let m23 = m[2 * 4 + 3];
        let m30 = m[3 * 4 + 0];
        let m31 = m[3 * 4 + 1];
        let m32 = m[3 * 4 + 2];
        let m33 = m[3 * 4 + 3];
        let tmp_0 = m22 * m33;
        let tmp_1 = m32 * m23;
        let tmp_2 = m12 * m33;
        let tmp_3 = m32 * m13;
        let tmp_4 = m12 * m23;
        let tmp_5 = m22 * m13;
        let tmp_6 = m02 * m33;
        let tmp_7 = m32 * m03;
        let tmp_8 = m02 * m23;
        let tmp_9 = m22 * m03;
        let tmp_10 = m02 * m13;
        let tmp_11 = m12 * m03;
        let tmp_12 = m20 * m31;
        let tmp_13 = m30 * m21;
        let tmp_14 = m10 * m31;
        let tmp_15 = m30 * m11;
        let tmp_16 = m10 * m21;
        let tmp_17 = m20 * m11;
        let tmp_18 = m00 * m31;
        let tmp_19 = m30 * m01;
        let tmp_20 = m00 * m21;
        let tmp_21 = m20 * m01;
        let tmp_22 = m00 * m11;
        let tmp_23 = m10 * m01;

        let t0 =
            tmp_0 * m11 + tmp_3 * m21 + tmp_4 * m31 - (tmp_1 * m11 + tmp_2 * m21 + tmp_5 * m31);
        let t1 =
            tmp_1 * m01 + tmp_6 * m21 + tmp_9 * m31 - (tmp_0 * m01 + tmp_7 * m21 + tmp_8 * m31);
        let t2 =
            tmp_2 * m01 + tmp_7 * m11 + tmp_10 * m31 - (tmp_3 * m01 + tmp_6 * m11 + tmp_11 * m31);
        let t3 =
            tmp_5 * m01 + tmp_8 * m11 + tmp_11 * m21 - (tmp_4 * m01 + tmp_9 * m11 + tmp_10 * m21);

        let d = 1.0 / (m00 * t0 + m10 * t1 + m20 * t2 + m30 * t3);

        res[0] = d * t0;
        res[1] = d * t1;
        res[2] = d * t2;
        res[3] = d * t3;
        res[4] = d
            * (tmp_1 * m10 + tmp_2 * m20 + tmp_5 * m30 - (tmp_0 * m10 + tmp_3 * m20 + tmp_4 * m30));
        res[5] = d
            * (tmp_0 * m00 + tmp_7 * m20 + tmp_8 * m30 - (tmp_1 * m00 + tmp_6 * m20 + tmp_9 * m30));
        res[6] = d
            * (tmp_3 * m00 + tmp_6 * m10 + tmp_11 * m30
                - (tmp_2 * m00 + tmp_7 * m10 + tmp_10 * m30));
        res[7] = d
            * (tmp_4 * m00 + tmp_9 * m10 + tmp_10 * m20
                - (tmp_5 * m00 + tmp_8 * m10 + tmp_11 * m20));
        res[8] = d
            * (tmp_12 * m13 + tmp_15 * m23 + tmp_16 * m33
                - (tmp_13 * m13 + tmp_14 * m23 + tmp_17 * m33));
        res[9] = d
            * (tmp_13 * m03 + tmp_18 * m23 + tmp_21 * m33
                - (tmp_12 * m03 + tmp_19 * m23 + tmp_20 * m33));
        res[10] = d
            * (tmp_14 * m03 + tmp_19 * m13 + tmp_22 * m33
                - (tmp_15 * m03 + tmp_18 * m13 + tmp_23 * m33));
        res[11] = d
            * (tmp_17 * m03 + tmp_20 * m13 + tmp_23 * m23
                - (tmp_16 * m03 + tmp_21 * m13 + tmp_22 * m23));
        res[12] = d
            * (tmp_14 * m22 + tmp_17 * m32 + tmp_13 * m12
                - (tmp_16 * m32 + tmp_12 * m12 + tmp_15 * m22));
        res[13] = d
            * (tmp_20 * m32 + tmp_12 * m02 + tmp_19 * m22
                - (tmp_18 * m22 + tmp_21 * m32 + tmp_13 * m02));
        res[14] = d
            * (tmp_18 * m12 + tmp_23 * m32 + tmp_15 * m02
                - (tmp_22 * m32 + tmp_14 * m02 + tmp_19 * m12));
        res[15] = d
            * (tmp_22 * m22 + tmp_16 * m02 + tmp_21 * m12
                - (tmp_20 * m12 + tmp_23 * m22 + tmp_17 * m02));
    }

    res
}

/// Multiplies by an x rotation matrix.
///
/// This is the optimized version of `multiply(m, x_rotate(angle))`
pub fn x_rotate(m: Matrix4, angle: Radians) -> Matrix4 {
    let mut res: Matrix4 = [0.0; 16];

    let m10 = m[4];
    let m11 = m[5];
    let m12 = m[6];
    let m13 = m[7];
    let m20 = m[8];
    let m21 = m[9];
    let m22 = m[10];
    let m23 = m[11];
    let c = angle.cos() as f32;
    let s = angle.sin() as f32;

    res[4] = c * m10 + s * m20;
    res[5] = c * m11 + s * m21;
    res[6] = c * m12 + s * m22;
    res[7] = c * m13 + s * m23;
    res[8] = c * m20 - s * m10;
    res[9] = c * m21 - s * m11;
    res[10] = c * m22 - s * m12;
    res[11] = c * m23 - s * m13;

    if m != res {
        res[0] = m[0];
        res[1] = m[1];
        res[2] = m[2];
        res[3] = m[3];
        res[12] = m[12];
        res[13] = m[13];
        res[14] = m[14];
        res[15] = m[15];
    }

    res
}

/// Multiplies by an y rotation matrix.
///
/// This is the optimized version of `multiply(m, y_rotate(angle))`
pub fn y_rotate(m: Matrix4, angle: Radians) -> Matrix4 {
    let mut res: Matrix4 = [0.0; 16];

    #[allow(clippy::erasing_op, clippy::identity_op)]
    {
        let m00 = m[0 * 4 + 0];
        let m01 = m[0 * 4 + 1];
        let m02 = m[0 * 4 + 2];
        let m03 = m[0 * 4 + 3];
        let m20 = m[2 * 4 + 0];
        let m21 = m[2 * 4 + 1];
        let m22 = m[2 * 4 + 2];
        let m23 = m[2 * 4 + 3];
        let c = angle.cos() as f32;
        let s = angle.sin() as f32;

        res[0] = c * m00 - s * m20;
        res[1] = c * m01 - s * m21;
        res[2] = c * m02 - s * m22;
        res[3] = c * m03 - s * m23;
        res[8] = c * m20 + s * m00;
        res[9] = c * m21 + s * m01;
        res[10] = c * m22 + s * m02;
        res[11] = c * m23 + s * m03;

        if m != res {
            res[4] = m[4];
            res[5] = m[5];
            res[6] = m[6];
            res[7] = m[7];
            res[12] = m[12];
            res[13] = m[13];
            res[14] = m[14];
            res[15] = m[15];
        }
    }

    res
}

/// Computes a 4-by-4 perspective transformation matrix given the angular height
/// of the frustum, the aspect ratio, and the near and far clipping planes.  The
/// arguments define a frustum extending in the negative z direction.  The given
/// angle is the vertical angle of the frustum, and the horizontal angle is
/// determined to produce the given aspect ratio.  The arguments near and far are
/// the distances to the near and far clipping planes.  Note that near and far
/// are not z coordinates, but rather they are distances along the negative
/// z-axis.  The matrix generated sends the viewing frustum to the unit box.
/// We assume a unit box extending from -1 to 1 in the x and y dimensions and
/// from -1 to 1 in the z dimension.
///
/// * `field_of_view`: field of view in y axis
/// * `aspect`: aspect of viewport (width / height)
/// * `near`: near Z clipping plane
/// * `far`: far Z clipping plane
pub fn perspective(field_of_view: Radians, aspect: f32, near: f32, far: f32) -> Matrix4 {
    let mut res: Matrix4 = [0.0; 16];

    let fov = *field_of_view as f32;
    let f = (std::f32::consts::PI * 0.5 - 0.5 * fov).tan();
    let range_inv = 1.0 / (near - far);

    res[0] = f / aspect;
    res[1] = 0.0;
    res[2] = 0.0;
    res[3] = 0.0;
    res[4] = 0.0;
    res[5] = f;
    res[6] = 0.0;
    res[7] = 0.0;
    res[8] = 0.0;
    res[9] = 0.0;
    res[10] = (near + far) * range_inv;
    res[11] = -1.0;
    res[12] = 0.0;
    res[13] = 0.0;
    res[14] = near * far * range_inv * 2.0;
    res[15] = 0.0;

    res
}

pub fn get_angle_between_vectors(a: &Point3D, b: &Point3D) -> Radians {
    let dot_product = a.x * b.x + a.y * b.y + a.z * b.z;

    let a_len = (a.x * a.x + a.y * a.y + a.z * a.z).sqrt();
    let b_len = (b.x * b.x + b.y * b.y + b.z * b.z).sqrt();
    let len_mult = a_len * b_len;

    let angle_cos = dot_product / len_mult;

    Radians(angle_cos.acos())
}
