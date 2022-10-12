pub enum LineNumberState {
    NotStarted,
    On(i32),
}

impl LineNumberState {
    pub fn update(self, line_number: i32) {
        match self {
            NotStarted => On(line_number),
            On(current) => {
                if current == line_number {
                    On(current)
                }
                On(line_numbers




