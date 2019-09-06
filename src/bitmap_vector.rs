
pub mod bitmap_vector {
    use std::mem;
    use crate::sys_core_t::Word;

    fn word_len() -> usize {
        return mem::size_of::<Word>() * 8;
    }

    trait BitmapVector {
        fn from_uncompressed(vector: Vec<Word>) -> Self;
        fn and(&self, other: &Self) -> Self;
        fn or(&self, other: &Self) -> Self;
        fn value(&self) -> &Vec<Word>;
        fn clone(&self) -> Self;
    }

    // struct WAHMetadata {
    //
    // }

    struct WAHVector {
        value: Vec<Word> // TODO convert to an array?
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

        fn from_uncompressed(vector: Vec<Word>) -> WAHVector {
            // TODO compress vector
            return WAHVector { value: vector };
        }

        // TODO generic binary 'op' function

        fn and(&self, other: &WAHVector) -> WAHVector {
            return WAHVector { value: self.value.clone() }; // TODO
        }

        fn or(&self, other: &WAHVector) -> WAHVector {
            return WAHVector { value: self.value.clone() }; // TODO
        }

        fn value(&self) -> &Vec<Word> {
            return &self.value;
        }

        fn clone(&self) -> WAHVector {
            return WAHVector { value: self.value.clone() };
        }
    }
}
