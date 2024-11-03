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

fn min_bits_required_for_field(values_amount: usize) -> usize {
    // round up log2
    (values_amount - 1).ilog2() as usize + 1
}

fn min_int_type_required_for_field(values_amount: usize) -> &'static str {
    if values_amount <= (1 << 8) {
        "uint8_t"
    } else if values_amount <= (1 << 16) {
        "uint16_t"
    } else if values_amount <= (1 << 32) {
        "uint32_t"
    } else {
        "uint64_t"
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
    pub fn bit_field(self, field_name: &str, field_type: &str, bits_amount: usize) -> Self {
        self.emitter.code.push_str(field_type);
        self.emitter.code.push(' ');
        self.emitter.code.push_str(field_name);
        self.emitter.code.push(':');
        self.emitter.code.push_str(&bits_amount.to_string());
        self.emitter.code.push(';');
        self
    }
    pub fn bit_field_min_size(self, field_name: &str, values_amount: usize) -> Self {
        self.bit_field(
            field_name,
            min_int_type_required_for_field(values_amount),
            min_bits_required_for_field(values_amount),
        )
    }
    pub fn emit(self) {
        self.emitter.code.push('}');
        self.emitter.code.push_str(self.struct_name);
        self.emitter.code.push(';');
    }
}
