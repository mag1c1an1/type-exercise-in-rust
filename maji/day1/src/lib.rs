use std::fmt::Debug;

use bitvec::prelude::BitVec;
pub fn add(left: usize, right: usize) -> usize {
    left + right
}

/// A type that is primitive, such as `i32` and `i64`.
pub trait PrimitiveType: Copy + Send + Sync + Default + Debug + 'static {}
struct PrimitiveArray<T: PrimitiveType> {
    data: Vec<T>,
    bitmap: BitVec,
}

pub struct StringArray {
    /// The flattened data of string.
    data: Vec<u8>,

    /// Offsets of each string in the data flat array.
    offsets: Vec<usize>,

    /// The null bitmap of this array.
    bitmap: BitVec,
}

impl StringArray {
    fn get(&self, idx: usize) -> Option<&str> {
        if self.bitmap[idx] {
            let range = self.offsets[idx]..self.offsets[idx + 1];
            Some(unsafe { std::str::from_utf8_unchecked(&self.data[range]) })
        } else {
            None
        }
    }
}

impl<T: PrimitiveType> PrimitiveArray<T> {
    fn get(&self, idx: usize) -> Option<T> {
        if self.bitmap[idx] {
            Some(self.data[idx])
        } else {
            None
        }
    }

    fn len(&self) -> usize {
        self.data.len()
    }
}

pub trait Array:Sized {
    type RefItem<'a>
    where
        Self: 'a;
    type Builder: ArrayBuilder<Array = Self>;
    fn get(&self, idx: usize) -> Option<Self::RefItem<'_>>;
    fn len(&self) -> usize;
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
    /// Get iterator of this array.
    fn iter(&self) -> ArrayIterator<Self>;
}

impl<T: PrimitiveType> Array for PrimitiveArray<T> {
    type RefItem<'a> = T;
    type Builder = PrimitiveArrayBuilder<T>;

    fn get(&self, idx: usize) -> Option<Self::RefItem<'_>> {
        todo!()
    }

    fn len(&self) -> usize {
        todo!()
    }
    
    fn iter(&self) -> ArrayIterator<Self> {
        todo!()
    }
}

struct PrimitiveArrayBuilder<T: PrimitiveType> {
    /// The actual data of this array.
    data: Vec<T>,

    /// The null bitmap of this array.
    bitmap: BitVec,
}

impl<T: PrimitiveType> ArrayBuilder for PrimitiveArrayBuilder<T> {
    type Array = PrimitiveArray<T>;

    fn with_capacity(capacity: usize) -> Self {
        todo!()
    }

    fn push(&mut self, value: Option<<Self::Array as Array>::RefItem<'_>>) {
        todo!()
    }

    fn finish(self) -> Self::Array {
        todo!()
    }
}

pub struct StringArrayBuilder {
    /// The flattened data of string.
    data: Vec<u8>,

    /// Offsets of each string in the data flat array.
    offsets: Vec<usize>,

    /// The null bitmap of this array.
    bitmap: BitVec,
}

impl ArrayBuilder for StringArrayBuilder {
    type Array = StringArray;

    fn with_capacity(capacity: usize) -> Self {
        todo!()
    }

    fn push(&mut self, value: Option<<Self::Array as Array>::RefItem<'_>>) {
        todo!()
    }

    fn finish(self) -> Self::Array {
        todo!()
    }
}

impl Array for StringArray {
    type RefItem<'a> = &'a str;
    type Builder = StringArrayBuilder;

    fn get(&self, idx: usize) -> Option<Self::RefItem<'_>> {
        todo!()
    }

    fn len(&self) -> usize {
        todo!()
    }
    
    fn iter(&self) -> ArrayIterator<Self> {
        todo!()
    }
}

pub struct ArrayIterator<'a, A: Array> {
    array: &'a A,
    pos: usize,
}

impl<'a, A: Array> ArrayIterator<'a, A> {
    pub fn new(array: &'a A) -> Self {
        Self { array, pos: 0 }
    }
}

/// Generic
impl<'a, A: Array> Iterator for ArrayIterator<'a, A> {
    type Item = Option<A::RefItem<'a>>;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

pub trait ArrayBuilder {
    type Array: Array;

    /// Create a new builder with `capacity`.
    fn with_capacity(capacity: usize) -> Self;

    /// Append a value to builder.
    fn push(&mut self, value: Option<<Self::Array as Array>::RefItem<'_>>);

    /// Finish build and return a new array.
    fn finish(self) -> Self::Array;
}

fn eval_binary<I: Array, O: Array>(i1: I, i2: I) -> O {
    let mut builder = O::Builder::with_capacity(i1.len());
    for (i1, i2) in i1.iter().zip(i2.iter()) {}
    builder.finish()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
