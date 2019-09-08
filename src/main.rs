mod bitmap_vector;
mod sys_map;
mod sys_core_t;
mod master;

mod tst {
    use super::sys_map::sys_map;
    pub fn hash_test() {
        println!("Testing hash function");

        for i in 1..20 {
            println!("h({:2}) = {:064b}", i, sys_map::vec_id_hash(i));
        }
    }
}

fn main() {

    println!("Starting DBIE...");
    println!("Running tests");
    crate::tst::hash_test();
    // basic testing
    //println!("{}", hash(0)); // TODO import this
}
