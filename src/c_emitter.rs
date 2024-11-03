/// C code emitter
pub struct CEmitter {
    code: String,
}
impl CEmitter {
    pub fn new() -> Self {
        Self {
            code: String::new(),
        }
    }

    pub fn code(&self) -> &str {
        &self.code
    }

    pub fn emit_enum<'a, S, I>(&mut self, enum_name: &str, variants: I)
    where
        S: AsRef<str>,
        I: Iterator<Item = S>,
    {
        self.code.push_str("typedef enum {");
        for variant in variants {
            self.code.push_str(variant.as_ref());
            self.code.push(',');
        }
        self.code.push('}');
        self.code.push_str(enum_name);
        self.code.push(';');
    }

    pub fn begin_struct(&mut self, struct_name: &'static str) -> CStructEmitter {
        self.code.push_str("typedef struct {");
        CStructEmitter {
            emitter: self,
            struct_name,
        }
    }
}

pub struct CStructEmitter<'a> {
    emitter: &'a mut CEmitter,
    struct_name: &'static str,
}
impl<'a> CStructEmitter<'a> {
    pub fn field(self, field_name: &str, field_type: &str) -> Self {
        self.emitter.code.push_str(field_type);
        self.emitter.code.push(' ');
        self.emitter.code.push_str(field_name);
        self.emitter.code.push(';');
        self
    }
    pub fn emit(self) {
        self.emitter.code.push('}');
        self.emitter.code.push_str(self.struct_name);
        self.emitter.code.push(';');
    }
}
