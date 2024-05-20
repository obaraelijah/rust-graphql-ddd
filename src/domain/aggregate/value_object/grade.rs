#[derive(Copy, Debug, PartialEq, Eq,Clone)]
pub enum Grade {
    First,
    Second,
    Third,
    Fourth,
}

impl std::convert::From<Grade> for usize {
    fn from(value: Grade) -> Self {
        match value {
            Grade::First => 1,
            Grade::Second => 2,
            Grade::Third => 3,
            Grade::Fourth => 4,
        }
    }
}

impl std::convert::TryFrom<usize> for Grade {
    type Error = anyhow::Error;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        Ok(match value {
            1 => Grade::First,
            2 => Grade::Second,
            3 => Grade::Third,
            4 => Grade::Fourth,
            _ => anyhow::bail!("error"),
        })
    }
}
