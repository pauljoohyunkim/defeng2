
#[derive(Debug)]
pub enum CVType {
    ConsonantFormer,
    ConsonantLatter,
    Vowel
}

#[derive(Debug)]
pub struct CVTree {
    pub node: CVType,
    pub depth: u16,
    pub children: [Option<Box<CVTree>>; 2]
}

impl CVTree {
    pub fn new(depth: u16, initial: CVType) -> Self {
        if depth == 0 {
            Self {
                node: initial,
                depth: 0,
                children: [None, None]
            }
        } else {
            match initial {
                CVType::ConsonantFormer => Self {
                    node: initial,
                    depth: depth,
                    children: [
                        Some(Box::new(CVTree::new(depth-1, CVType::Vowel))),
                        None
                    ]
                },
                CVType::ConsonantLatter => Self {
                    node: initial,
                    depth: depth,
                    children: [
                        Some(Box::new(CVTree::new(depth-1, CVType::ConsonantFormer))),
                        None
                    ]
                },
                CVType::Vowel => Self {
                    node: initial,
                    depth: depth,
                    children: [
                        Some(Box::new(CVTree::new(depth-1, CVType::ConsonantFormer))),
                        Some(Box::new(CVTree::new(depth-1, CVType::ConsonantLatter)))
                    ]
                }
            }
        }
    }
}
