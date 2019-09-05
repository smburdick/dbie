
mod bitmap_vector {
    use std::mem;

    type Word = u64;

    struct BitmapVector {
        value: Vec<Word> // TODO could make this an array
    }

    impl BitmapVector {
        fn clone(&self) -> Self {
            return Self {
                value: self.value.clone()
            };
        }
        fn word_length(&self) -> usize {
            return mem::size_of::<Word>() * 8;
        }
    }

    trait AlgorithmicBitmapVector {
        fn from_uncompressed(vector: BitmapVector) -> Self;
        fn and(&self, other: &Self) -> Self;
        //fn or(&self, other: &BitmapVector) -> BitmapVector;
        //fn clone(&self) -> Self;
    }

    struct WAHMetadata {

    }

    struct WAHVector {
        value: BitmapVector
        // TODO documentation on vectors
        // /**
        //
        // 2 types of words: literal, fill
        // Most sig bit = flag
        // Literal = (flag, litval) when flag = 0
        //     rem 63 bits are uncompressed bit seq
        // Fill = (flag, val, len) when flag = 1
        //     bit with val repeats for len bits when uncompressed
        //
        // */
    }

    impl AlgorithmicBitmapVector for WAHVector {
        // WAHVector::from_uncompressed
        fn from_uncompressed(vector: BitmapVector) -> WAHVector {
            // TODO compress vector
            return WAHVector { value: vector };
        }

        // TODO generic binary 'op' function

        fn and(&self, other: &WAHVector) -> WAHVector {
            return WAHVector { value: self.value.clone() }; // TODO
        }
    }
}
