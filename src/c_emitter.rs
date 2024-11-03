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

    pub fn emit_system_include(&mut self, header_file_name: &str) {
        self.code.push_str("#include <");
        self.code.push_str(header_file_name);
        self.code.push_str(">\n");
    }

    pub fn emit_enum<'a, S, I>(&mut self, enum_name: &str, variants: I)
    where
        S: AsRef<str>,
        I: IntoIterator<Item = S>,
    {
        self.code.push_str("typedef enum {");
        for variant in variants {
            self.code.push_str(variant.as_ref());
            self.code.push(',');
        }
        self.code.push('}');
        self.code.push_str(enum_name);
        self.code.push_str(";\n");
    }

    pub fn begin_struct(&mut self, struct_name: &'static str) -> CStructEmitter {
        self.code.push_str("typedef struct {");
        CStructEmitter {
            emitter: self,
            struct_name,
        }
    }

    pub fn begin_table(&mut self, struct_name: &str, table_name: &str) -> CTableEmitter {
        self.code.push_str("const ");
        self.code.push_str(struct_name);
        self.code.push(' ');
        self.code.push_str(table_name);
        self.code.push_str("[] = {");
        CTableEmitter { emitter: self }
    }
}

pub fn min_bits_required_for_field(values_amount: usize) -> usize {
    // round up log2
    (values_amount - 1).ilog2() as usize + 1
}

pub fn min_int_type_required_for_field(values_amount: usize) -> &'static str {
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
        self.emitter.code.push_str(";\n");
    }
}

pub struct CTableEmitter<'a> {
    emitter: &'a mut CEmitter,
}
impl<'a> CTableEmitter<'a> {
    pub fn begin_entry(&mut self) -> CTableEntryEmitter {
        self.emitter.code.push('{');
        CTableEntryEmitter {
            emitter: self.emitter,
        }
    }
    pub fn emit(self) {
        self.emitter.code.push_str("};\n");
    }
}

pub struct CTableEntryEmitter<'a> {
    emitter: &'a mut CEmitter,
}
impl<'a> CTableEntryEmitter<'a> {
    pub fn field(self, field_name: &str, value: &str) -> Self {
        self.emitter.code.push('.');
        self.emitter.code.push_str(field_name);
        self.emitter.code.push('=');
        self.emitter.code.push_str(value);
        self.emitter.code.push(',');
        self
    }
    pub fn emit(self) {
        self.emitter.code.push_str("},\n");
    }
}
