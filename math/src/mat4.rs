use super::mat2x3::Mat2X3;
use super::vec4::TVec4;
use num_traits::Num;

/// 4x4 matrix, stored in row major ordering.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(C)]
pub struct Mat4<T> {
    a00: T,
    a01: T,
    a02: T,
    a03: T,

    a10: T,
    a11: T,
    a12: T,
    a13: T,

    a20: T,
    a21: T,
    a22: T,
    a23: T,

    a30: T,
    a31: T,
    a32: T,
    a33: T,
}

impl<T> Mat4<T>
where
    T: Num + Copy + Clone + std::fmt::Debug,
{
    pub fn as_slice(&self) -> &[T] {
        unsafe { std::slice::from_raw_parts(&self.a00 as *const _, 16) }
    }

    pub fn as_mut_slice(&mut self) -> &mut [T] {
        unsafe { std::slice::from_raw_parts_mut(&mut self.a00 as *mut _, 16) }
    }

    pub fn as_ptr(&self) -> *const T {
        &self.a00 as *const _
    }

    pub fn as_mut_ptr(&mut self) -> *mut T {
        &mut self.a00 as *mut _
    }

    pub fn transpose(&self) -> Self {
        Self {
            a00: self.a00,
            a01: self.a10,
            a02: self.a20,
            a03: self.a30,

            a10: self.a01,
            a11: self.a11,
            a12: self.a21,
            a13: self.a31,

            a20: self.a02,
            a21: self.a12,
            a22: self.a22,
            a23: self.a32,

            a30: self.a03,
            a31: self.a13,
            a32: self.a23,
            a33: self.a33,
        }
    }
}

pub mod consts {
    use super::Mat4;
    use num_traits::Num;

    pub fn null<T>() -> Mat4<T>
    where
        T: Num + Copy + Clone + std::fmt::Debug,
    {
        Mat4 {
            a00: T::zero(),
            a01: T::zero(),
            a02: T::zero(),
            a03: T::zero(),

            a10: T::zero(),
            a11: T::zero(),
            a12: T::zero(),
            a13: T::zero(),

            a20: T::zero(),
            a21: T::zero(),
            a22: T::zero(),
            a23: T::zero(),

            a30: T::zero(),
            a31: T::zero(),
            a32: T::zero(),
            a33: T::zero(),
        }
    }

    pub fn identity<T>() -> Mat4<T>
    where
        T: Num + Copy + Clone + std::fmt::Debug,
    {
        Mat4 {
            a00: T::one(),
            a11: T::one(),
            a22: T::one(),
            a33: T::one(),
            ..null()
        }
    }
}

impl<T> std::iter::FromIterator<T> for Mat4<T>
where
    T: Num + Copy + Clone + std::fmt::Debug,
{
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut m = consts::null();
        m.as_mut_slice()
            .iter_mut()
            .zip(iter)
            .for_each(|(dst, src)| *dst = src);

        m
    }
}

impl<T, U: AsRef<[T]>> std::convert::From<U> for Mat4<T>
where
    T: Num + Copy + Clone + std::fmt::Debug,
{
    fn from(s: U) -> Self {
        let mut m = consts::null();
        m.as_mut_slice()
            .iter_mut()
            .zip(s.as_ref().iter())
            .for_each(|(dst, src)| *dst = *src);
        m
    }
}

impl<T> std::convert::From<Mat2X3<T>> for Mat4<T>
where
    T: Num + Copy + Clone + std::fmt::Debug,
{
    fn from(m: Mat2X3<T>) -> Self {
        Self {
            a00: m.a00,
            a01: m.a01,
            a02: T::zero(),
            a03: m.a02,

            a10: m.a10,
            a11: m.a11,
            a12: T::zero(),
            a13: m.a12,

            ..consts::identity()
        }
    }
}

impl<T> std::ops::Index<usize> for Mat4<T>
where
    T: Num + Copy + Clone + std::fmt::Debug,
{
    type Output = TVec4<T>;

    fn index(&self, idx: usize) -> &Self::Output {
        debug_assert!(idx < 4);

        unsafe { &*(self.as_ptr().add(idx * 4) as *const TVec4<T>) }
    }
}

impl<T> std::ops::IndexMut<usize> for Mat4<T>
where
    T: Num + Copy + Clone + std::fmt::Debug,
{
    fn index_mut(&mut self, idx: usize) -> &mut Self::Output {
        debug_assert!(idx < 4);

        unsafe { &mut *(self.as_mut_ptr().add(idx * 4) as *mut TVec4<T>) }
    }
}

impl<T> std::ops::Mul for Mat4<T>
where
    T: Num + Copy + Clone + std::fmt::Debug + std::ops::AddAssign + std::ops::Mul,
{
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut res = consts::null();

        (0..4).for_each(|row| {
            (0..4).for_each(|col| {
                (0..4).for_each(|k| {
                    res[row][col] += self[row][k] * rhs[k][col];
                });
            });
        });

        res
    }
}

pub type Mat4F32 = Mat4<f32>;
pub type Mat4I32 = Mat4<i32>;

#[cfg(test)]
mod tests {
    use super::super::vec4::*;
    use super::*;

    #[test]
    fn test_index_ops() {
        use std::convert::From;
        use std::iter::FromIterator;

        let m = Mat4::from_iter(0..16);

        assert_eq!(m[0], Vec4I32::new(0, 1, 2, 3));
        assert_eq!(m[1], Vec4I32::new(4, 5, 6, 7));
        assert_eq!(m[2], Vec4I32::new(8, 9, 10, 11));
        assert_eq!(m[3], Vec4I32::new(12, 13, 14, 15));

        let mut m = Mat4::from_iter(0..16);
        m[0].as_mut_slice().iter_mut().for_each(|x| *x *= 2);
        assert_eq!(m[0], Vec4I32::from([0, 2, 4, 6]));
    }

    #[test]
    fn test_multiplication() {
        use std::iter::FromIterator;
        let m0 = Mat4::from_iter(1..=16);
        let m1 = Mat4::from_iter(17..=17 + 15);

        let res = m0 * m1;
        assert_eq!(
            res,
            Mat4::from([
                250, 260, 270, 280, 618, 644, 670, 696, 986, 1028, 1070, 1112, 1354, 1412, 1470,
                1528
            ])
        );
    }

    #[test]
    fn test_transpose() {
        use std::iter::FromIterator;
        let m = Mat4::from_iter(0..16);
        let m1 = m.transpose();

        assert_eq!(
            m1,
            Mat4::from([0, 4, 8, 12, 1, 5, 9, 13, 2, 6, 10, 14, 3, 7, 11, 15])
        );
    }
}
