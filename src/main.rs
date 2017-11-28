extern crate sequence_trie;
extern crate rand;
extern crate mine;


use mine::Repository;


fn main() {
    let mut repo = Repository::new();
    repo.insert("foo/bar", &[1,2,3,4]);
    println!("{:?}", repo.get("foo/bar"));
    repo.delete("foo/bar");
}
