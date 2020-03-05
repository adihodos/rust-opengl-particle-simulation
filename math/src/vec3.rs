use num_traits::{Float, Num};
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(C)]
pub struct TVec3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T> TVec3<T>
where
    T: Copy + Clone + Num + std::fmt::Debug,
{
    pub fn new(x: T, y: T, z: T) -> Self {
        TVec3 { x, y, z }
    }

    pub fn same(val: T) -> Self {
        Self::new(val, val, val)
    }

    pub fn as_slice(&self) -> &[T] {
        unsafe { std::slice::from_raw_parts(&self.x as *const _, 3) }
    }

    pub fn as_mut_slice(&mut self) -> &mut [T] {
        unsafe { std::slice::from_raw_parts_mut(&mut self.x as *mut _, 3) }
    }

    pub fn as_ptr(&self) -> *const T {
        &self.x as *const _
    }

    pub fn as_mut_ptr(&mut self) -> *mut T {
        &mut self.x as *mut _
    }

    pub fn length_squared(&self) -> T {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn length(&self) -> T
    where
        T: Float + Copy + Clone + std::fmt::Debug,
    {
        self.length_squared().sqrt()
    }
}

pub mod consts {
    use super::TVec3;
    use num_traits::Num;

    pub fn null<T>() -> TVec3<T>
    where
        T: Copy + Clone + Num + std::fmt::Debug,
    {
        TVec3 {
            x: T::zero(),
            y: T::zero(),
            z: T::zero(),
        }
    }

    pub fn unit_x<T>() -> TVec3<T>
    where
        T: Num + Copy + Clone + std::fmt::Debug,
    {
        TVec3 {
            x: T::one(),
            y: T::zero(),
            z: T::zero(),
        }
    }

    pub fn unit_y<T>() -> TVec3<T>
    where
        T: Num + Copy + Clone + std::fmt::Debug,
    {
        TVec3 {
            x: T::zero(),
            y: T::one(),
            z: T::zero(),
        }
    }

    pub fn unit_z<T>() -> TVec3<T>
    where
        T: Num + Copy + Clone + std::fmt::Debug,
    {
        TVec3 {
            x: T::zero(),
            y: T::zero(),
            z: T::one(),
        }
    }
}

impl<T, U: AsRef<[T]>> std::convert::From<U> for TVec3<T>
where
    T: Copy + Clone + Num + std::fmt::Debug,
{
    fn from(s: U) -> Self {
        let mut v = consts::null();
        v.as_mut_slice()
            .iter_mut()
            .zip(s.as_ref().iter())
            .for_each(|(dst, src)| *dst = *src);
        v
    }
}

impl<T> std::ops::Index<usize> for TVec3<T>
where
    T: Copy + Clone + Num + std::fmt::Debug,
{
    type Output = T;
    fn index(&self, idx: usize) -> &Self::Output {
        debug_assert!(idx < 3);
        &self.as_slice()[idx]
    }
}

impl<T> std::ops::IndexMut<usize> for TVec3<T>
where
    T: Copy + Clone + Num + std::fmt::Debug,
{
    fn index_mut(&mut self, idx: usize) -> &mut Self::Output {
        debug_assert!(idx < 3);
        &mut self.as_mut_slice()[idx]
    }
}
