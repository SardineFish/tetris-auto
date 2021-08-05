#[derive(Clone, Copy)]
pub enum GameOP {
    New,
    Left(i8),
    Right(i8),
    Down(i8),
    Rotate(i8),
}

pub trait GameOPStr {
    fn to_op_string(self) -> String;    
}

impl GameOPStr for &[GameOP] {
    fn to_op_string(self) -> String {
        let mut outputs = Vec::with_capacity(self.len());
        for op in self {
            match op {
                GameOP::New => outputs.push(format!("N")),
                GameOP::Left(x) => outputs.push(format!("L{}", x)),
                GameOP::Right(x) => outputs.push(format!("R{}", x)),
                GameOP::Down(y) => outputs.push(format!("D{}", y)),
                GameOP::Rotate(rot) => outputs.push(format!("C{}", rot)),
            }
        }

        outputs.join(",")
    }
}
