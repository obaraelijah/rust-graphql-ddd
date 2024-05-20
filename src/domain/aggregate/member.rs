
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Member {
    pub name: String,
    pub age: usize,

}
impl Member {
    pub fn new(name: String, age: usize) -> Self {
        Member { 
            name, 
            age, 
        }
    }
}