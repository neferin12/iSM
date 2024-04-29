#[derive(Debug, Clone)]
pub struct Points {
    pub first_selection: u16,
    pub second_selection: u16,
    pub third_selection: u16,
    pub no_selection: u16,
}

impl Points {
    pub fn default() -> Points {
        return Points {
            first_selection: 0,
            second_selection: 5,
            third_selection: 10,
            no_selection: 30,
        };
    }
}