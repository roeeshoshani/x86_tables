pub type Mnemonic = &'static str;

const UNSUPPORTED_MNEMONIC: Mnemonic = "unsupported";

const OPCODE_REG_INSN_REPEAT_COUNT: usize = 8;

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

    /// a common size info for immediates that are either 16 or 32 bits.
    const SZ_IMM_16_32: Self = Self {
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
struct AxOpInfo {
    size: OpSizeInfo,
}

#[derive(Debug, Clone)]
enum OpInfo {
    Imm(ImmOpInfo),
    Reg(RegOpInfo),
    Rm(RmOpInfo),
    Ax(AxOpInfo),
}
impl OpInfo {
    const RM_8: Self = Self::Rm(RmOpInfo {
        size: OpSizeInfo::SZ_ALWAYS_8,
    });
    const RM_16_32_64_DEF_32: Self = Self::Rm(RmOpInfo {
        size: OpSizeInfo::SZ_16_32_64_DEF_32,
    });
    const R_MODRM_8: Self = Self::Reg(RegOpInfo {
        encoding: RegEncoding::Modrm,
        size: OpSizeInfo::SZ_ALWAYS_8,
    });
    const R_MODRM_16_32_64_DEF_32: Self = Self::Reg(RegOpInfo {
        encoding: RegEncoding::Modrm,
        size: OpSizeInfo::SZ_16_32_64_DEF_32,
    });
    const AL: Self = Self::Ax(AxOpInfo {
        size: OpSizeInfo::SZ_ALWAYS_8,
    });
    const AX_16_32_64_DEF_32: Self = Self::Ax(AxOpInfo {
        size: OpSizeInfo::SZ_16_32_64_DEF_32,
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
struct InsnInfo {
    mnemonic: Mnemonic,
    ops: Ops,
}

fn simple_binary_op(table: &mut Vec<InsnInfo>, mnemonic: Mnemonic) {
    table.push(InsnInfo {
        mnemonic,
        ops: &[OpInfo::RM_8, OpInfo::R_MODRM_8],
    });
    table.push(InsnInfo {
        mnemonic,
        ops: &[OpInfo::RM_16_32_64_DEF_32, OpInfo::R_MODRM_16_32_64_DEF_32],
    });
    table.push(InsnInfo {
        mnemonic,
        ops: &[OpInfo::R_MODRM_8, OpInfo::RM_8],
    });
    table.push(InsnInfo {
        mnemonic,
        ops: &[OpInfo::R_MODRM_16_32_64_DEF_32, OpInfo::RM_16_32_64_DEF_32],
    });
    table.push(InsnInfo {
        mnemonic,
        ops: &[OpInfo::AL, OpInfo::IMM_8_NO_EXT],
    });
    table.push(InsnInfo {
        mnemonic,
        ops: &[
            OpInfo::AX_16_32_64_DEF_32,
            OpInfo::Imm(ImmOpInfo {
                encoded_size: OpSizeInfo::SZ_IMM_16_32,
                extended_size: OpSizeInfo::SZ_16_32_64_DEF_32,
                extend_kind: ImmExtendKind::SignExtend,
            }),
        ],
    });
}

fn repeat(table: &mut Vec<InsnInfo>, amount: usize, entry: InsnInfo) {
    table.extend(std::iter::repeat_n(entry, amount))
}

fn unsupported(table: &mut Vec<InsnInfo>, amount: usize) {
    repeat(
        table,
        amount,
        InsnInfo {
            mnemonic: UNSUPPORTED_MNEMONIC,
            ops: &[],
        },
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
        OPCODE_REG_INSN_REPEAT_COUNT,
        InsnInfo {
            mnemonic: "inc",
            ops: &[OpInfo::Reg(RegOpInfo {
                encoding: RegEncoding::Opcode,
                size: OpSizeInfo::SZ_16_32_64_DEF_32,
            })],
        },
    );
    // 0x48 - 0x4f
    repeat(
        &mut table,
        OPCODE_REG_INSN_REPEAT_COUNT,
        InsnInfo {
            mnemonic: "dec",
            ops: &[OpInfo::Reg(RegOpInfo {
                encoding: RegEncoding::Opcode,
                size: OpSizeInfo::SZ_16_32_64_DEF_32,
            })],
        },
    );
    // 0x50 - 0x57
    repeat(
        &mut table,
        OPCODE_REG_INSN_REPEAT_COUNT,
        InsnInfo {
            mnemonic: "push",
            ops: &[OpInfo::Reg(RegOpInfo {
                encoding: RegEncoding::Opcode,
                size: OpSizeInfo::SZ_16_32_64_DEF_64,
            })],
        },
    );
    // 0x58 - 0x5f
    repeat(
        &mut table,
        OPCODE_REG_INSN_REPEAT_COUNT,
        InsnInfo {
            mnemonic: "pop",
            ops: &[OpInfo::Reg(RegOpInfo {
                encoding: RegEncoding::Opcode,
                size: OpSizeInfo::SZ_16_32_64_DEF_64,
            })],
        },
    );
    // 0x60 - 0x67
    unsupported(&mut table, 8);
    // 0x68
    table.push(InsnInfo {
        mnemonic: "push",
        ops: &[OpInfo::Imm(ImmOpInfo {
            encoded_size: OpSizeInfo::SZ_IMM_16_32,
            extended_size: OpSizeInfo::SZ_16_32_64_DEF_64,
            extend_kind: ImmExtendKind::SignExtend,
        })],
    });
    // 0x69
    table.push(InsnInfo {
        mnemonic: "imul",
        ops: &[
            OpInfo::R_MODRM_16_32_64_DEF_32,
            OpInfo::RM_16_32_64_DEF_32,
            OpInfo::Imm(ImmOpInfo {
                encoded_size: OpSizeInfo::SZ_IMM_16_32,
                extended_size: OpSizeInfo::SZ_16_32_64_DEF_32,
                extend_kind: ImmExtendKind::SignExtend,
            }),
        ],
    });

    table
}

fn main() {
    println!("Hello, world!");
}
