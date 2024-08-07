#[cfg(test)]
mod tests {
    use std::ops::DerefMut;
    use derive_deref::{Deref, DerefMut};

    #[derive(Deref)]
    struct WrapperTupleDeref(i32);

    #[test]
    fn tuple_deref() {
        let w = WrapperTupleDeref(1);
        assert_eq!(*w, 1);
    }

    #[derive(DerefMut, Deref)]
    struct WrapperTupleDerefMut(i32);

    #[test]
    fn tuple_deref_mut() {
        let mut w = WrapperTupleDerefMut(1);
        *w *= 2;
        assert_eq!(*w, 2);
    }

    #[derive(DerefMut, Deref)]
    struct WrapperTupleMultipleFields(i32, i32);

    #[test]
    fn tuple_multiple_fields() {
        let mut w = WrapperTupleMultipleFields(1, 3);
        *w *= 2;
        assert_eq!(*w, 2);
        assert_eq!(*w.deref_mut(), 2);
    }

    #[derive(Deref)]
    struct WrapperStructDeref {
        #[DerefTarget]
        field: i32,
    }

    #[test]
    fn struct_deref() {
        let w = WrapperStructDeref { field: 1 };
        assert_eq!(*w, 1);
    }

    #[derive(Deref, DerefMut)]
    struct WrapperStructDiff {
        #[DerefTarget]
        #[DerefMutTarget]
        field: i32,
    }

    #[test]
    fn struct_deref_mut() {
        let mut w = WrapperStructDiff { field: 1,};
        *w *= 2;
        assert_eq!(*w.deref_mut(), 2);
    }

    #[derive(Deref, DerefMut)]
    struct WrapperStructDifferentTargets {
        #[DerefTarget]
        field: i32,
        #[DerefMutTarget]
        field_mut: i32,
    }
    
    #[test]
    fn struct_deref_mut_different_targets() {
        let mut w = WrapperStructDifferentTargets { field: 1, field_mut: 2};
        *w *= 2;
        assert_eq!(*w, 1);
        assert_eq!(*w.deref_mut(), 4);
    }
}
