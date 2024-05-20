 

#[derive(Copy, Debug, Clone, PartialEq, Eq)]
pub struct MemberId(usize);

impl MemberId {
    pub fn gen() -> Self {
        Self(rand::random::<usize>())
    }
}
