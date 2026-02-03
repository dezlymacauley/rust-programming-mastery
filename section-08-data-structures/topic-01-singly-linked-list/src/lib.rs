pub struct Node {
    pub  value: i32,
    pub next: Option<Box<Node>>
}

impl Node {
    pub fn new(value: i32) -> Self {
        Self {
            value: value,
            next: None
        }
    } 
}

//_____________________________________________________________________________


//_____________________________________________________________________________
