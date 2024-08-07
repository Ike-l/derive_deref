#[cfg(test)]
mod tests {
    use std::ops::DerefMut;
    use small_derive_deref::{Deref, DerefMut};

    #[derive(Deref)]
    struct TupleDeref(i32);

    #[test]
    fn tuple_deref() {
        let w = TupleDeref(1);
        assert_eq!(*w, 1);
    }

    #[derive(DerefMut, Deref)]
    struct TupleDerefMut(i32);

    #[test]
    fn tuple_deref_mut() {
        let mut w = TupleDerefMut(1);
        *w *= 2;
        assert_eq!(*w, 2);
    }

    #[derive(DerefMut, Deref)]
    struct TupleMultipleFields(i32, i32);

    #[test]
    fn tuple_multiple_fields() {
        let mut w = TupleMultipleFields(1, 3);
        *w *= 2;
        assert_eq!(*w, 2);
        assert_eq!(*w.deref_mut(), 2);
        assert_eq!(w.1, 3);
    }

    #[derive(Deref)]
    struct StructDeref {
        #[DerefTarget]
        field: i32,
    }

    #[test]
    fn struct_deref() {
        let w = StructDeref { field: 1 };
        assert_eq!(*w, 1);
    }

    #[derive(Deref, DerefMut)]
    struct StructDiff {
        #[DerefTarget]
        #[DerefMutTarget]
        field: i32,
    }

    #[test]
    fn struct_deref_mut() {
        let mut w = StructDiff { field: 1,};
        *w *= 2;
        assert_eq!(*w.deref_mut(), 2);
    }

    #[derive(Deref, DerefMut)]
    struct StructDifferentTargets {
        #[DerefTarget]
        field: i32,
        #[DerefMutTarget]
        field_mut: i32,
    }
    
    #[test]
    fn struct_deref_mut_different_targets() {
        let mut w = StructDifferentTargets { field: 1, field_mut: 2};
        *w *= 2;
        assert_eq!(*w, 1);
        assert_eq!(*w.deref_mut(), 4);
    }

    #[derive(Deref)]
    struct TupleWithGenericsDeref<'a>(&'a str);

    #[test]
    fn tuple_generics_deref() {
        let w = TupleWithGenericsDeref("string");
        assert_eq!("string", *w);
    }

    #[derive(Deref, DerefMut)]
    struct TupleWithGenericsDerefMut<'a>(&'a str);

    #[test]
    fn tuple_generics_deref_mut() {
        let mut w = TupleWithGenericsDerefMut("string");
        *w = "other";
        assert_eq!("other", *w);
    }
}
