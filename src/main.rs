pub type Mnemonic = &'static str;

const UNSUPPORTED_MNEMONIC: Mnemonic = "unsupported";

enum OpSize {
    S8,
    S16,
    S32,
    S64,
}

struct OpSizeInfo {
    mode_32: OpSize,
    mode_64: OpSize,
    mode_64_with_rex_w: OpSize,
    with_operand_size_override: OpSize,
}
impl OpSizeInfo {
    /// operand size is always 8 bits
    const SZ_ALWAYS_8: Self = Self {
        mode_32: OpSize::S8,
        mode_64: OpSize::S8,
        mode_64_with_rex_w: OpSize::S8,
        with_operand_size_override: OpSize::S8,
    };

    /// operand size may be 16, 32 or 64 bits.
    /// the default is 32 bits in both 32 and 64 bit mode. operand size override makes it 16 bits. REX.W makes it 64 bits.
    const SZ_16_32_64_DEF_32: Self = Self {
        mode_32: OpSize::S8,
        mode_64: OpSize::S8,
        mode_64_with_rex_w: OpSize::S8,
        with_operand_size_override: OpSize::S8,
    };
}

enum ImmExtendKind {
    SignExtend,
    ZeroExtend,
}

struct ImmOpInfo {
    encoded_size: OpSizeInfo,
    extended_size: OpSizeInfo,
    extend_kind: ImmExtendKind,
}

enum RegEncoding {
    Modrm,
    Opcode,
}

struct RegOpInfo {
    encoding: RegEncoding,
    size: OpSizeInfo,
}

struct RmOpInfo {
    size: OpSizeInfo,
}

struct AxOpInfo {
    size: OpSizeInfo,
}

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
                encoded_size: OpSizeInfo {
                    mode_32: OpSize::S32,
                    mode_64: OpSize::S32,
                    mode_64_with_rex_w: OpSize::S32,
                    with_operand_size_override: OpSize::S16,
                },
                extended_size: OpSizeInfo {
                    mode_32: OpSize::S32,
                    mode_64: OpSize::S32,
                    mode_64_with_rex_w: OpSize::S64,
                    with_operand_size_override: OpSize::S16,
                },
                extend_kind: ImmExtendKind::SignExtend,
            }),
        ],
    });
}

fn unsupported(table: &mut Vec<InsnInfo>, amount: usize) {
    for _ in 0..amount {
        table.push(InsnInfo {
            mnemonic: UNSUPPORTED_MNEMONIC,
            ops: &[],
        })
    }
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

    table
}

fn main() {
    println!("Hello, world!");
}
