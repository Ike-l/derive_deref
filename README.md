# small_derive_deref
This crate adds ```#[derive(Deref)]``` ```#[derive(DerefMut]```.<br>

It works for:
* Structs with one field
* Structs with multiple fields
  * Need ```#[DerefTarget]``` / ```#[DerefMutTarget]```.
* Tuples with one field
* Tuples with multiple fields
  * Will use the first field

## Known Limitations
* The type of the fields for ```#[DerefTarget]``` and ```#[DerefMutTarget]``` must be the same

## Examples
```rust
use std::ops::DerefMut;
use small_derive_deref::{Deref, DerefMut};

 #[derive(Deref, DerefMut)]
 struct WrapperStructDifferentTargetsMultipleGenerics<'a, T> {
     #[DerefTarget]
     field: &'a str,
     #[DerefMutTarget]
     field_mut: &'a str,
     foo: T
 }
 
 let mut w = WrapperStructDifferentTargetsMultipleGenerics { field: "not rust", field_mut: "rust", foo: "foo"};
 assert_eq!(*w, "not rust");
 *w = "rUst";
 assert_eq!(*w.deref_mut(), "rUst");
 
 
 #[derive(Deref, DerefMut)]
 struct WrapperTuple(i32, i32);
 
 let mut w = WrapperTuple(1, 3);
 *w *= 2;
 assert_eq!(*w, 2);
 assert_eq!(*w.deref_mut(), 2);
```
## License
MIT or Apache-2.0
