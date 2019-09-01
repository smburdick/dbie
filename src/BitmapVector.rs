struct BitmapVector {
    value: Vec<u64>
}

impl BitmapVector {
    fn clone(&self) -> Self {
        return Self {
            value: self.value.clone()
        };
    }
}

trait AlgorithmicBitmapVector {
    fn new_from_uncompressed(vector: BitmapVector) -> Self;
    fn and(&self, other: &Self) -> Self;
    //fn or(&self, other: &BitmapVector) -> BitmapVector;
    //fn clone(&self) -> Self;
}

struct WAHVector {
    value: BitmapVector
}

impl AlgorithmicBitmapVector for WAHVector {
    fn new_from_uncompressed(vector: BitmapVector) -> WAHVector {
        // TODO compress vector
        return WAHVector { value: vector };
    }
    fn and(&self, other: &WAHVector) -> WAHVector {
        return WAHVector { value: self.value.clone() }; // TODO
    }
}

fn main() {}
