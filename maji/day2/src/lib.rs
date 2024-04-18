#![allow(warnings)]

pub trait Scalar {
    type ArrayType: Array<OwnedItem = Self>;
    type RefType<'a>: ScalarRef<'a, ScalarType = Self, ArrayType = Self::ArrayType>
    where
        Self: 'a;
    fn as_scalar_ref(&self) -> Self::RefType<'_>;
}

pub trait ScalarRef<'a>: 'a {
    type ArrayType: Array<RefItem<'a> = Self>;
    type ScalarType: Scalar<RefType<'a> = Self>;
    fn to_owned_scalar(&self) -> Self::ScalarType;
}

pub trait Array: Sized
where
    for<'a> Self::OwnedItem: Scalar<RefType<'a> = Self::RefItem<'a>>,
{
    type RefItem<'a>: ScalarRef<'a, ScalarType = Self::OwnedItem, ArrayType = Self>
    where
        Self: 'a;

    type Builder: ArrayBuilder<Target = Self>;

    type OwnedItem: Scalar<ArrayType = Self>;

    fn get(&self, index: usize) -> Option<Self::RefItem<'_>>;

    fn iter(&self) -> Arrayiterator<Self>;
}

pub trait ArrayBuilder {
    type Target: Array;

    fn with_capacity(cap: usize) -> Self;

    fn push(&mut self, value: Option<<Self::Target as Array>::RefItem<'_>>);

    fn finish(self) -> Self::Target;
}

pub trait PrimitiveType: Scalar {}

struct PrimitiveArray<T: PrimitiveType> {
    data: Vec<T>,
}

struct PrimitiveArrayBuilder<T: PrimitiveType> {
    data: Vec<T>,
}

impl<T> ArrayBuilder for PrimitiveArrayBuilder<T>
where
    T: PrimitiveType,
    T: Scalar<ArrayType = PrimitiveArray<T>>,
    for<'a> T: ScalarRef<'a, ArrayType = PrimitiveArray<T>, ScalarType = T>,
    for<'a> T: Scalar<RefType<'a> = T>,
{
    type Target = PrimitiveArray<T>;

    fn with_capacity(cap: usize) -> Self {
        todo!()
    }

    fn push(&mut self, value: Option<<Self::Target as Array>::RefItem<'_>>) {
        todo!()
    }

    fn finish(self) -> Self::Target {
        todo!()
    }
}

struct Arrayiterator<'a, A: Array> {
    array: &'a A,
}

impl<T> Array for PrimitiveArray<T>
where
    T: PrimitiveType,
    T: Scalar<ArrayType = Self>,
    for<'a> T: ScalarRef<'a, ArrayType = Self, ScalarType = T>,
    for<'a> T: Scalar<RefType<'a> = T>,
{
    type RefItem<'a>
    where
        Self: 'a,
    = <T as Scalar>::RefType<'a>;

    type Builder = PrimitiveArrayBuilder<T>;

    type OwnedItem = T;

    fn get(&self, index: usize) -> Option<Self::RefItem<'_>> {
        todo!()
    }

    fn iter(&self) -> Arrayiterator<Self> {
        todo!()
    }
}

pub struct StringArray {
    data: Vec<u8>,
}

pub struct StringArrayBuilder {
    data: Vec<u8>,
}

impl ArrayBuilder for StringArrayBuilder {
    type Target = StringArray;

    fn with_capacity(cap: usize) -> Self {
        todo!()
    }

    fn push(&mut self, value: Option<<Self::Target as Array>::RefItem<'_>>) {
        todo!()
    }

    fn finish(self) -> Self::Target {
        todo!()
    }
}

impl<'a> ScalarRef<'a> for &'a str {
    type ArrayType = StringArray;

    type ScalarType = String;

    fn to_owned_scalar(&self) -> Self::ScalarType {
        todo!()
    }
}

impl Array for StringArray {
    type RefItem<'a>
    where
        Self: 'a,
    = &'a str;

    type Builder = StringArrayBuilder;

    type OwnedItem = String;

    fn get(&self, index: usize) -> Option<Self::RefItem<'_>> {
        todo!()
    }

    fn iter(&self) -> Arrayiterator<Self> {
        todo!()
    }
}

impl Scalar for String {
    type ArrayType = StringArray;

    type RefType<'a>
    where
        Self: 'a,
    = &'a str;

    fn as_scalar_ref(&self) -> Self::RefType<'_> {
        todo!()
    }
}

impl<'a, A: Array> Iterator for Arrayiterator<'a, A> {
    type Item = Option<A::RefItem<'a>>;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

fn sql_func<'a, I: Array, O: Array>(i1: I::RefItem<'a>, i2: I::RefItem<'a>) -> O::OwnedItem {
    todo!()
}

fn eval_binary<I: Array, O: Array>(i1: I, i2: I) -> O {
    let mut builder = O::Builder::with_capacity(3);
    for (i1, i2) in i1.iter().zip(i2.iter()) {
        match (i1, i2) {
            (Some(i1), Some(i2)) => builder.push(Some(sql_func::<I, O>(i1, i2).as_scalar_ref())),
            _ => builder.push(None),
        }
    }
    builder.finish()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {}
}
