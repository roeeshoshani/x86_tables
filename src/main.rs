pub type Mnemonic = &'static str;

const MNEMONIC_UNSUPPORTED: Mnemonic = "unsupported";

const SIMPLE_BINOP_MNEMONICS: [Mnemonic; 8] =
    ["add", "or", "adc", "sbb", "and", "sub", "xor", "cmd"];

#[derive(Debug, Clone)]
enum OpSize {
    S8,
    S16,
    S32,
    S64,
}

#[derive(Debug, Clone)]
struct OpSizeInfo {
    with_operand_size_override: OpSize,
    mode_32: OpSize,
    mode_64: OpSize,
    mode_64_with_rex_w: OpSize,
}
impl OpSizeInfo {
    /// operand size is always 8 bits
    const SZ_ALWAYS_8: Self = Self {
        with_operand_size_override: OpSize::S8,
        mode_32: OpSize::S8,
        mode_64: OpSize::S8,
        mode_64_with_rex_w: OpSize::S8,
    };

    /// the default operand size for instructions that default to 32-bit operands.
    const SZ_16_32_64_DEF_32: Self = Self {
        with_operand_size_override: OpSize::S16,
        mode_32: OpSize::S32,
        mode_64: OpSize::S32,
        mode_64_with_rex_w: OpSize::S64,
    };

    /// the default operand size for instructions that default to 64-bit operands.
    const SZ_16_32_64_DEF_64: Self = Self {
        with_operand_size_override: OpSize::S16,
        mode_32: OpSize::S32,
        mode_64: OpSize::S64,
        mode_64_with_rex_w: OpSize::S64,
    };

    /// a common size info for immediate encodings that are either 16 or 32 bits.
    const SZ_IMM_ENCODING_16_32: Self = Self {
        with_operand_size_override: OpSize::S16,
        mode_32: OpSize::S32,
        mode_64: OpSize::S32,
        mode_64_with_rex_w: OpSize::S32,
    };
}

#[derive(Debug, Clone)]
enum ImmExtendKind {
    SignExtend,
    ZeroExtend,
}

#[derive(Debug, Clone)]
struct ImmOpInfo {
    encoded_size: OpSizeInfo,
    extended_size: OpSizeInfo,
    extend_kind: ImmExtendKind,
}

#[derive(Debug, Clone)]
struct MemOffsetOpInfo {
    mem_operand_size: OpSizeInfo,
}

#[derive(Debug, Clone)]
enum RegEncoding {
    Modrm,
    Opcode,
}

#[derive(Debug, Clone)]
struct RegOpInfo {
    encoding: RegEncoding,
    size: OpSizeInfo,
}

#[derive(Debug, Clone)]
struct RmOpInfo {
    size: OpSizeInfo,
}

#[derive(Debug, Clone)]
struct SpecificRegOpInfo {
    reg_64_bit_name: &'static str,
    size: OpSizeInfo,
}

#[derive(Debug, Clone)]
struct RelOpInfo {
    size: OpSizeInfo,
}

#[derive(Debug, Clone)]
enum OpInfo {
    /// immediate operand
    Imm(ImmOpInfo),

    /// register operand
    Reg(RegOpInfo),

    /// rm operand
    Rm(OpSizeInfo),

    /// specific register which is enforced by the opcode
    SpecificReg(SpecificRegOpInfo),

    /// relative offset used for relative jumps
    Rel(OpSizeInfo),

    /// memory access by absolute address, for example `mov rcx, [0x1234]`
    MemOffset(MemOffsetOpInfo),

    /// an implicit operand which is not actually specified in the instruction, only its size it relevant.
    Implicit(OpSizeInfo),

    Cond,
}
impl OpInfo {
    const RM_8: Self = Self::Rm(OpSizeInfo::SZ_ALWAYS_8);
    const RM_16_32_64_DEF_32: Self = Self::Rm(OpSizeInfo::SZ_16_32_64_DEF_32);
    const RM_16_32_64_DEF_64: Self = Self::Rm(OpSizeInfo::SZ_16_32_64_DEF_64);
    const R_MODRM_8: Self = Self::Reg(RegOpInfo {
        encoding: RegEncoding::Modrm,
        size: OpSizeInfo::SZ_ALWAYS_8,
    });
    const R_MODRM_16_32_64_DEF_32: Self = Self::Reg(RegOpInfo {
        encoding: RegEncoding::Modrm,
        size: OpSizeInfo::SZ_16_32_64_DEF_32,
    });
    const R_OPCODE_16_32_64_DEF_32: Self = Self::Reg(RegOpInfo {
        encoding: RegEncoding::Opcode,
        size: OpSizeInfo::SZ_16_32_64_DEF_32,
    });
    const R_OPCODE_16_32_64_DEF_64: Self = Self::Reg(RegOpInfo {
        encoding: RegEncoding::Opcode,
        size: OpSizeInfo::SZ_16_32_64_DEF_64,
    });
    const AL: Self = Self::SpecificReg(SpecificRegOpInfo {
        size: OpSizeInfo::SZ_ALWAYS_8,
        reg_64_bit_name: "rax",
    });
    const AX_16_32_64_DEF_32: Self = Self::SpecificReg(SpecificRegOpInfo {
        size: OpSizeInfo::SZ_16_32_64_DEF_32,
        reg_64_bit_name: "rax",
    });
    const DX_16_32_64_DEF_32: Self = Self::SpecificReg(SpecificRegOpInfo {
        size: OpSizeInfo::SZ_16_32_64_DEF_32,
        reg_64_bit_name: "rdx",
    });

    /// an 8-bit immediate which should not be sign/zero extended.
    const IMM_8_NO_EXT: Self = Self::Imm(ImmOpInfo {
        encoded_size: OpSizeInfo::SZ_ALWAYS_8,
        extended_size: OpSizeInfo::SZ_ALWAYS_8,
        // doesn't matter
        extend_kind: ImmExtendKind::SignExtend,
    });
}

type Ops = &'static [OpInfo];

#[derive(Debug, Clone)]
struct RegularInsnInfo {
    mnemonic: Mnemonic,
    ops: Ops,
}
impl RegularInsnInfo {
    const UNSUPPORTED: Self = Self {
        mnemonic: MNEMONIC_UNSUPPORTED,
        ops: &[],
    };
}

#[derive(Debug, Clone)]
struct ModrmRegOpcodeExtInsnInfo {
    by_reg_value: [RegularInsnInfo; 8],
}
impl ModrmRegOpcodeExtInsnInfo {
    fn new_with_same_operands(ops: Ops, mnemonics: [Mnemonic; 8]) -> Self {
        Self {
            by_reg_value: std::array::from_fn(|i| RegularInsnInfo {
                mnemonic: mnemonics[i],
                ops,
            }),
        }
    }
}

#[derive(Debug, Clone)]
enum InsnInfo {
    Regular(RegularInsnInfo),
    ModrmRegOpcodeExt(ModrmRegOpcodeExtInsnInfo),
}

fn simple_binary_op(table: &mut Vec<InsnInfo>, mnemonic: Mnemonic) {
    table.push(InsnInfo::Regular(RegularInsnInfo {
        mnemonic,
        ops: &[OpInfo::RM_8, OpInfo::R_MODRM_8],
    }));
    table.push(InsnInfo::Regular(RegularInsnInfo {
        mnemonic,
        ops: &[OpInfo::RM_16_32_64_DEF_32, OpInfo::R_MODRM_16_32_64_DEF_32],
    }));
    table.push(InsnInfo::Regular(RegularInsnInfo {
        mnemonic,
        ops: &[OpInfo::R_MODRM_8, OpInfo::RM_8],
    }));
    table.push(InsnInfo::Regular(RegularInsnInfo {
        mnemonic,
        ops: &[OpInfo::R_MODRM_16_32_64_DEF_32, OpInfo::RM_16_32_64_DEF_32],
    }));
    table.push(InsnInfo::Regular(RegularInsnInfo {
        mnemonic,
        ops: &[OpInfo::AL, OpInfo::IMM_8_NO_EXT],
    }));
    table.push(InsnInfo::Regular(RegularInsnInfo {
        mnemonic,
        ops: &[
            OpInfo::AX_16_32_64_DEF_32,
            OpInfo::Imm(ImmOpInfo {
                encoded_size: OpSizeInfo::SZ_IMM_ENCODING_16_32,
                extended_size: OpSizeInfo::SZ_16_32_64_DEF_32,
                extend_kind: ImmExtendKind::SignExtend,
            }),
        ],
    }));
}

fn repeat(table: &mut Vec<InsnInfo>, amount: usize, entry: InsnInfo) {
    table.extend(std::iter::repeat_n(entry, amount))
}

fn unsupported(table: &mut Vec<InsnInfo>, amount: usize) {
    repeat(
        table,
        amount,
        InsnInfo::Regular(RegularInsnInfo::UNSUPPORTED),
    )
}

fn table() -> Vec<InsnInfo> {
    let mut table = Vec::new();

    // 0x00 - 0x05
    simple_binary_op(&mut table, "add");
    // 0x06 - 0x07
    unsupported(&mut table, 2);
    // 0x08 - 0x0d
    simple_binary_op(&mut table, "or");
    // 0x0e - 0x0f
    unsupported(&mut table, 2);
    // 0x10 - 0x15
    simple_binary_op(&mut table, "adc");
    // 0x16 - 0x17
    unsupported(&mut table, 2);
    // 0x18 - 0x1d
    simple_binary_op(&mut table, "sbb");
    // 0x1e - 0x1f
    unsupported(&mut table, 2);
    // 0x20 - 0x25
    simple_binary_op(&mut table, "and");
    // 0x26 - 0x27
    unsupported(&mut table, 2);
    // 0x28 - 0x2d
    simple_binary_op(&mut table, "sub");
    // 0x2e - 0x2f
    unsupported(&mut table, 2);
    // 0x30 - 0x35
    simple_binary_op(&mut table, "xor");
    // 0x26 - 0x27
    unsupported(&mut table, 2);
    // 0x38 - 0x3d
    simple_binary_op(&mut table, "cmp");
    // 0x3e - 0x3f
    unsupported(&mut table, 2);
    // 0x40 - 0x47
    repeat(
        &mut table,
        8,
        InsnInfo::Regular(RegularInsnInfo {
            mnemonic: "inc",
            ops: &[OpInfo::R_OPCODE_16_32_64_DEF_32],
        }),
    );
    // 0x48 - 0x4f
    repeat(
        &mut table,
        8,
        InsnInfo::Regular(RegularInsnInfo {
            mnemonic: "dec",
            ops: &[OpInfo::R_OPCODE_16_32_64_DEF_32],
        }),
    );
    // 0x50 - 0x57
    repeat(
        &mut table,
        8,
        InsnInfo::Regular(RegularInsnInfo {
            mnemonic: "push",
            ops: &[OpInfo::R_OPCODE_16_32_64_DEF_64],
        }),
    );
    // 0x58 - 0x5f
    repeat(
        &mut table,
        8,
        InsnInfo::Regular(RegularInsnInfo {
            mnemonic: "pop",
            ops: &[OpInfo::R_OPCODE_16_32_64_DEF_64],
        }),
    );
    // 0x60 - 0x67
    unsupported(&mut table, 8);
    // 0x68
    table.push(InsnInfo::Regular(RegularInsnInfo {
        mnemonic: "push",
        ops: &[OpInfo::Imm(ImmOpInfo {
            encoded_size: OpSizeInfo::SZ_IMM_ENCODING_16_32,
            extended_size: OpSizeInfo::SZ_16_32_64_DEF_64,
            extend_kind: ImmExtendKind::SignExtend,
        })],
    }));
    // 0x69
    table.push(InsnInfo::Regular(RegularInsnInfo {
        mnemonic: "imul",
        ops: &[
            OpInfo::R_MODRM_16_32_64_DEF_32,
            OpInfo::RM_16_32_64_DEF_32,
            OpInfo::Imm(ImmOpInfo {
                encoded_size: OpSizeInfo::SZ_IMM_ENCODING_16_32,
                extended_size: OpSizeInfo::SZ_16_32_64_DEF_32,
                extend_kind: ImmExtendKind::SignExtend,
            }),
        ],
    }));
    // 0x6a
    table.push(InsnInfo::Regular(RegularInsnInfo {
        mnemonic: "push",
        ops: &[OpInfo::Imm(ImmOpInfo {
            encoded_size: OpSizeInfo::SZ_ALWAYS_8,
            extended_size: OpSizeInfo::SZ_16_32_64_DEF_64,
            extend_kind: ImmExtendKind::SignExtend,
        })],
    }));
    // 0x6b
    table.push(InsnInfo::Regular(RegularInsnInfo {
        mnemonic: "imul",
        ops: &[
            OpInfo::R_MODRM_16_32_64_DEF_32,
            OpInfo::RM_16_32_64_DEF_32,
            OpInfo::Imm(ImmOpInfo {
                encoded_size: OpSizeInfo::SZ_ALWAYS_8,
                extended_size: OpSizeInfo::SZ_16_32_64_DEF_32,
                extend_kind: ImmExtendKind::SignExtend,
            }),
        ],
    }));
    // 0x6c - 0x6f
    unsupported(&mut table, 4);
    // 0x70 - 0x7f
    repeat(
        &mut table,
        16,
        InsnInfo::Regular(RegularInsnInfo {
            mnemonic: "jcc",
            ops: &[OpInfo::Cond, OpInfo::Rel(OpSizeInfo::SZ_ALWAYS_8)],
        }),
    );
    // 0x80
    table.push(InsnInfo::ModrmRegOpcodeExt(
        ModrmRegOpcodeExtInsnInfo::new_with_same_operands(
            &[OpInfo::RM_8, OpInfo::IMM_8_NO_EXT],
            SIMPLE_BINOP_MNEMONICS,
        ),
    ));
    // 0x81
    table.push(InsnInfo::ModrmRegOpcodeExt(
        ModrmRegOpcodeExtInsnInfo::new_with_same_operands(
            &[
                OpInfo::RM_16_32_64_DEF_32,
                OpInfo::Imm(ImmOpInfo {
                    encoded_size: OpSizeInfo::SZ_IMM_ENCODING_16_32,
                    extended_size: OpSizeInfo::SZ_16_32_64_DEF_32,
                    extend_kind: ImmExtendKind::SignExtend,
                }),
            ],
            SIMPLE_BINOP_MNEMONICS,
        ),
    ));
    // 0x82
    unsupported(&mut table, 1);
    // 0x83
    table.push(InsnInfo::ModrmRegOpcodeExt(
        ModrmRegOpcodeExtInsnInfo::new_with_same_operands(
            &[
                OpInfo::RM_16_32_64_DEF_32,
                OpInfo::Imm(ImmOpInfo {
                    encoded_size: OpSizeInfo::SZ_ALWAYS_8,
                    extended_size: OpSizeInfo::SZ_16_32_64_DEF_32,
                    extend_kind: ImmExtendKind::SignExtend,
                }),
            ],
            SIMPLE_BINOP_MNEMONICS,
        ),
    ));
    // 0x84
    table.push(InsnInfo::Regular(RegularInsnInfo {
        mnemonic: "test",
        ops: &[OpInfo::RM_8, OpInfo::R_MODRM_8],
    }));
    // 0x85
    table.push(InsnInfo::Regular(RegularInsnInfo {
        mnemonic: "test",
        ops: &[OpInfo::RM_16_32_64_DEF_32, OpInfo::R_MODRM_16_32_64_DEF_32],
    }));
    // 0x86
    table.push(InsnInfo::Regular(RegularInsnInfo {
        mnemonic: "xchg",
        ops: &[OpInfo::RM_8, OpInfo::R_MODRM_8],
    }));
    // 0x87
    table.push(InsnInfo::Regular(RegularInsnInfo {
        mnemonic: "xchg",
        ops: &[OpInfo::RM_16_32_64_DEF_32, OpInfo::R_MODRM_16_32_64_DEF_32],
    }));
    // 0x88
    table.push(InsnInfo::Regular(RegularInsnInfo {
        mnemonic: "mov",
        ops: &[OpInfo::RM_8, OpInfo::R_MODRM_8],
    }));
    // 0x89
    table.push(InsnInfo::Regular(RegularInsnInfo {
        mnemonic: "mov",
        ops: &[OpInfo::RM_16_32_64_DEF_32, OpInfo::R_MODRM_16_32_64_DEF_32],
    }));
    // 0x8a
    table.push(InsnInfo::Regular(RegularInsnInfo {
        mnemonic: "mov",
        ops: &[OpInfo::R_MODRM_8, OpInfo::RM_8],
    }));
    // 0x8b
    table.push(InsnInfo::Regular(RegularInsnInfo {
        mnemonic: "mov",
        ops: &[OpInfo::R_MODRM_16_32_64_DEF_32, OpInfo::RM_16_32_64_DEF_32],
    }));
    // 0x8c
    unsupported(&mut table, 1);
    // 0x8d
    table.push(InsnInfo::Regular(RegularInsnInfo {
        mnemonic: "lea",
        ops: &[OpInfo::R_MODRM_16_32_64_DEF_32, OpInfo::RM_16_32_64_DEF_32],
    }));
    // 0x8e
    unsupported(&mut table, 1);
    // 0x8f
    table.push(InsnInfo::ModrmRegOpcodeExt(ModrmRegOpcodeExtInsnInfo {
        by_reg_value: [
            RegularInsnInfo {
                mnemonic: "pop",
                ops: &[OpInfo::RM_16_32_64_DEF_64],
            },
            RegularInsnInfo::UNSUPPORTED,
            RegularInsnInfo::UNSUPPORTED,
            RegularInsnInfo::UNSUPPORTED,
            RegularInsnInfo::UNSUPPORTED,
            RegularInsnInfo::UNSUPPORTED,
            RegularInsnInfo::UNSUPPORTED,
            RegularInsnInfo::UNSUPPORTED,
        ],
    }));
    // 0x90
    table.push(InsnInfo::Regular(RegularInsnInfo {
        mnemonic: "nop",
        ops: &[],
    }));
    // 0x91 - 0x97
    repeat(
        &mut table,
        8,
        InsnInfo::Regular(RegularInsnInfo {
            mnemonic: "xchg",
            ops: &[OpInfo::AX_16_32_64_DEF_32, OpInfo::R_OPCODE_16_32_64_DEF_32],
        }),
    );
    //0x98
    table.push(InsnInfo::Regular(RegularInsnInfo {
        mnemonic: "movsz", // this is actually cbw, but this makes life simpler when lifting it
        ops: &[
            OpInfo::AX_16_32_64_DEF_32,
            OpInfo::SpecificReg(SpecificRegOpInfo {
                reg_64_bit_name: "rax",
                size: OpSizeInfo {
                    with_operand_size_override: OpSize::S8,
                    mode_32: OpSize::S16,
                    mode_64: OpSize::S16,
                    mode_64_with_rex_w: OpSize::S32,
                },
            }),
        ],
    }));
    // 0x99
    table.push(InsnInfo::Regular(RegularInsnInfo {
        mnemonic: "cwd", // this is cwd/cdq/cqo
        ops: &[OpInfo::DX_16_32_64_DEF_32, OpInfo::AX_16_32_64_DEF_32],
    }));
    // 0x9a - 0x9f
    unsupported(&mut table, 6);
    // 0xa0
    table.push(InsnInfo::Regular(RegularInsnInfo {
        mnemonic: "mov",
        ops: &[
            OpInfo::AL,
            OpInfo::MemOffset(MemOffsetOpInfo {
                mem_operand_size: OpSizeInfo::SZ_ALWAYS_8,
            }),
        ],
    }));
    // 0xa1
    table.push(InsnInfo::Regular(RegularInsnInfo {
        mnemonic: "mov",
        ops: &[
            OpInfo::AX_16_32_64_DEF_32,
            OpInfo::MemOffset(MemOffsetOpInfo {
                mem_operand_size: OpSizeInfo::SZ_16_32_64_DEF_32,
            }),
        ],
    }));
    // 0xa2
    table.push(InsnInfo::Regular(RegularInsnInfo {
        mnemonic: "mov",
        ops: &[
            OpInfo::MemOffset(MemOffsetOpInfo {
                mem_operand_size: OpSizeInfo::SZ_ALWAYS_8,
            }),
            OpInfo::AL,
        ],
    }));
    // 0xa3
    table.push(InsnInfo::Regular(RegularInsnInfo {
        mnemonic: "mov",
        ops: &[
            OpInfo::MemOffset(MemOffsetOpInfo {
                mem_operand_size: OpSizeInfo::SZ_16_32_64_DEF_32,
            }),
            OpInfo::AX_16_32_64_DEF_32,
        ],
    }));
    // 0xa4
    table.push(InsnInfo::Regular(RegularInsnInfo {
        mnemonic: "movs",
        ops: &[OpInfo::Implicit(OpSizeInfo::SZ_ALWAYS_8)],
    }));
    // 0xa5
    table.push(InsnInfo::Regular(RegularInsnInfo {
        mnemonic: "movs",
        ops: &[OpInfo::Implicit(OpSizeInfo::SZ_16_32_64_DEF_32)],
    }));
    // 0xa6
    table.push(InsnInfo::Regular(RegularInsnInfo {
        mnemonic: "cmps",
        ops: &[OpInfo::Implicit(OpSizeInfo::SZ_ALWAYS_8)],
    }));
    // 0xa7
    table.push(InsnInfo::Regular(RegularInsnInfo {
        mnemonic: "cmps",
        ops: &[OpInfo::Implicit(OpSizeInfo::SZ_16_32_64_DEF_32)],
    }));
    // 0xa8
    table.push(InsnInfo::Regular(RegularInsnInfo {
        mnemonic: "test",
        ops: &[OpInfo::AL, OpInfo::IMM_8_NO_EXT],
    }));
    // 0xa9
    table.push(InsnInfo::Regular(RegularInsnInfo {
        mnemonic: "test",
        ops: &[
            OpInfo::AX_16_32_64_DEF_32,
            OpInfo::Imm(ImmOpInfo {
                encoded_size: OpSizeInfo::SZ_IMM_ENCODING_16_32,
                extended_size: OpSizeInfo::SZ_16_32_64_DEF_32,
                extend_kind: ImmExtendKind::SignExtend,
            }),
        ],
    }));
    // 0xaa
    table.push(InsnInfo::Regular(RegularInsnInfo {
        mnemonic: "stos",
        ops: &[OpInfo::Implicit(OpSizeInfo::SZ_ALWAYS_8)],
    }));
    // 0xab
    table.push(InsnInfo::Regular(RegularInsnInfo {
        mnemonic: "stos",
        ops: &[OpInfo::Implicit(OpSizeInfo::SZ_16_32_64_DEF_32)],
    }));
    // 0xac
    table.push(InsnInfo::Regular(RegularInsnInfo {
        mnemonic: "lods",
        ops: &[OpInfo::Implicit(OpSizeInfo::SZ_ALWAYS_8)],
    }));
    // 0xad
    table.push(InsnInfo::Regular(RegularInsnInfo {
        mnemonic: "lods",
        ops: &[OpInfo::Implicit(OpSizeInfo::SZ_16_32_64_DEF_32)],
    }));
    // 0xae
    table.push(InsnInfo::Regular(RegularInsnInfo {
        mnemonic: "scas",
        ops: &[OpInfo::Implicit(OpSizeInfo::SZ_ALWAYS_8)],
    }));
    // 0xaf
    table.push(InsnInfo::Regular(RegularInsnInfo {
        mnemonic: "scas",
        ops: &[OpInfo::Implicit(OpSizeInfo::SZ_16_32_64_DEF_32)],
    }));

    table
}

fn main() {
    let t = table();
    println!("{:?}", t);
}
