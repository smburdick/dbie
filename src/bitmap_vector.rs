
pub mod bitmap_vector {
    use std::mem;
    use crate::sys_core_t::{Word, VecIdT};

    fn word_len() -> usize {
        return mem::size_of::<Word>() * 8;
    }

    pub trait BitmapVector {
        fn from_uncompressed(vector: Vec<Word>, id: VecIdT) -> Self; // should vector take ownership of value reference?
        fn and(&self, other: &Self) -> Self;
        fn or(&self, other: &Self) -> Self;
        //fn value(&self) -> &Vec<Word>; // TODO consider changing to array
        fn id(&self) -> VecIdT;
        fn clone(&self) -> Self;
    }

    pub struct WAHVector {
        value: Vec<Word>, // TODO convert to an array somehow...?
        id: VecIdT
        // TODO documentation on vectors
        // /**
        //
        // 2 types of Words: literal, fill
        // Most sig bit = flag
        // Literal = (flag, litval) when flag = 0
        //     rem 63 bits are uncompressed bit seq
        // Fill = (flag, val, len) when flag = 1
        //     bit with val repeats for len bits when uncompressed
        //
        // */
    }

    impl BitmapVector for WAHVector {

        fn from_uncompressed(vector: Vec<Word>, id: VecIdT) -> WAHVector {
            // TODO compress vector
            return WAHVector {
                value: vector,
                id: id
            };
        }

        // TODO generic binary 'op' function

        fn and(&self, other: &WAHVector) -> WAHVector {
            return self.clone(); // TODO
        }

        fn or(&self, other: &WAHVector) -> WAHVector {
            return self.clone(); // TODO
        }

        fn id(&self) -> VecIdT {
            return VecIdT::from(self.id);
        }

        fn clone(&self) -> WAHVector {
            return WAHVector {
                value: self.value.clone(), // TODO use from instead
                id: self.id
            };
        }
    }
}
