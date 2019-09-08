// Defines the mapping structures and algos between objects and container nodes
// Also defines consistent hashing
pub extern crate sha1;
pub mod sys_map {

    use std::u64;
    use sys_core_t::{VecIdT, ReplFactorT, HashKeyT, IpAddrT, NodeIdT};
    use super::sha1::Sha1;

    pub trait DBIENode {
        fn name(&self) -> String;
        fn id(&self) -> NodeIdT;
        fn hashed_id(&self) -> HashKeyT;
        fn ip_addr(&self) -> IpAddrT;
    }

    pub struct AVLNode {
        name: String,
        id: NodeIdT,
        hashed_id: HashKeyT,
        ip_addr: IpAddrT
    }

    pub struct AVLTree {
        root: Option<AVLNode>
    }

    impl DBIENode for AVLNode {
        fn id(&self) -> NodeIdT { NodeIdT::from(self.id) }
        fn ip_addr(&self) -> IpAddrT { IpAddrT::from(self.ip_addr.clone()) }
        fn name(&self) -> String { String::from(self.name.clone()) }
        fn hashed_id(&self) -> HashKeyT { HashKeyT::from(self.hashed_id) }
    }

    // TODO understand generics syntax
    // For most implementations: HashKeyType => Node
    trait DBIETree<K, V> {
        fn new() -> Self;
        fn put(&mut self, value: &V);
        // fn get(&self, key : K) -> &V;
        // fn successor(&self, key : K) -> &V;
        // fn delete(&self, key: K);
        // fn consistent_hash(&self, k: K, r: ReplFactorT) -> Vec<&V>;
    }

    impl DBIETree<VecIdT, AVLNode> for AVLTree {
        fn new() -> AVLTree {
            return AVLTree { root: None }
        }
        fn put(&mut self, value: &AVLNode) {
            if self.root.is_some() {
                //let root = self.root.get_or_insert
                // TODO insert the node
            } else {
                let id = value.id();
                self.root = Some(AVLNode {
                    id: id,
                    hashed_id: vec_id_hash(id),
                    ip_addr: value.ip_addr.clone(),
                    name: value.name.clone()
                });
            }
        }
    }

    // ideally these should be private (were made public just for testing..)
    pub fn str_hash(key: &str) -> HashKeyT {
        let hasher = Sha1::from(key);
        let digest = &hasher.digest().to_string()[0..16];
        let mut res = u64::from_str_radix(&digest, 16).ok();
        return *res.get_or_insert(0); // if hashing fails, returns 0
    }

    pub fn vec_id_hash(key: VecIdT) -> HashKeyT { // -> u64
        return str_hash(&key.to_string());
    }
}
