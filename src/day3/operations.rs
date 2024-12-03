pub trait Operation {
    fn apply(&self, context: &mut Context);
}

pub struct Context {
    multiplication_enabled: bool,
    total: i32,
}

impl Context {
    pub fn new() -> Context {
        Context {
            multiplication_enabled: true,
            total: 0,
        }
    }

    pub fn result(&self) -> i32 {
        self.total
    }
}

pub struct Multiplication {
    pub a: i32,
    pub b: i32,
}

impl Operation for Multiplication {
    fn apply(&self, context: &mut Context) {
        if context.multiplication_enabled {
            context.total += self.result();
        }
    }
}

impl Multiplication {
    pub fn result(&self) -> i32 {
        self.a * self.b
    }
}

pub struct Configure {
    pub multiplication_enabled: bool,
}

impl Operation for Configure {
    fn apply(&self, context: &mut Context) {
        context.multiplication_enabled = self.multiplication_enabled
    }
}
