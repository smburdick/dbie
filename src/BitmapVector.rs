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



/**
 *

 */
trait AlgorithmicBitmapVector {
    //fn new
    fn and(&self, other: &Self) -> Self;
    //fn or(&self, other: &BitmapVector) -> BitmapVector;
    //fn compress(&self);
}

struct WAHVector {
    value: BitmapVector
}

impl AlgorithmicBitmapVector for WAHVector {
    fn and(&self, other: &WAHVector) -> WAHVector {
        return WAHVector { value: self.value.clone() }; // TODO
    }
}

fn main() {}
