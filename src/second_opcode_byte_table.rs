use crate::{table_gen_utils::*, table_types::*};

pub fn gen_second_opcode_byte_table() -> Vec<InsnInfo> {
    let mut table = Vec::new();

    // 0x00 - 0x1d
    assert_eq!(table.len(), 0x00);
    unsupported(&mut table, 0x1e);
    // 0x1e
    assert_eq!(table.len(), 0x1e);
    table.push(InsnInfo::ModrmRegOpcodeExt(ModrmRegOpcodeExtInsnInfo {
        by_reg_value: [
            // 0
            RegularInsnInfo::UNSUPPORTED,
            // 1
            RegularInsnInfo::UNSUPPORTED,
            // 2
            RegularInsnInfo::UNSUPPORTED,
            // 3
            RegularInsnInfo::UNSUPPORTED,
            // 4
            RegularInsnInfo::UNSUPPORTED,
            // 5
            RegularInsnInfo::UNSUPPORTED,
            // 6
            RegularInsnInfo::UNSUPPORTED,
            // 7
            RegularInsnInfo {
                mnemonic: "endbr",
                ops: &[],
            },
        ],
    }));
    // 0x1f
    assert_eq!(table.len(), 0x1f);
    table.push(InsnInfo::Regular(RegularInsnInfo {
        mnemonic: "nop",
        ops: &[OpInfo::Rm(OpSizeInfo {
            with_operand_size_override: OpSize::S16,
            mode_32: OpSize::S32,
            mode_64: OpSize::S32,
            mode_64_with_rex_w: OpSize::S32,
        })],
    }));
    // 0x20 - 0x3f
    assert_eq!(table.len(), 0x20);
    unsupported(&mut table, 0x20);
    // 0x40 - 0x4f
    assert_eq!(table.len(), 0x40);
    repeat(
        &mut table,
        16,
        InsnInfo::Regular(RegularInsnInfo {
            mnemonic: "cmovcc",
            ops: &[
                OpInfo::Cond,
                OpInfo::R_MODRM_16_32_64_DEF_32,
                OpInfo::RM_16_32_64_DEF_32,
            ],
        }),
    );
    // 0x50 - 0x7f
    assert_eq!(table.len(), 0x50);
    unsupported(&mut table, 0x30);
    // 0x80 - 0x8f
    assert_eq!(table.len(), 0x80);
    repeat(
        &mut table,
        16,
        InsnInfo::Regular(RegularInsnInfo {
            mnemonic: "jcc",
            ops: &[OpInfo::Cond, OpInfo::REL_32],
        }),
    );
    // 0x90 - 0x9f
    assert_eq!(table.len(), 0x90);
    repeat(
        &mut table,
        16,
        InsnInfo::Regular(RegularInsnInfo {
            mnemonic: "setcc",
            ops: &[OpInfo::Cond, OpInfo::RM_8],
        }),
    );
    // 0xa0 - 0xa2
    assert_eq!(table.len(), 0xa0);
    unsupported(&mut table, 3);
    // 0xa3
    assert_eq!(table.len(), 0xa3);
    table.push(InsnInfo::Regular(RegularInsnInfo {
        mnemonic: "bt",
        ops: &[OpInfo::RM_16_32_64_DEF_32, OpInfo::R_MODRM_16_32_64_DEF_32],
    }));
    // 0xa4
    assert_eq!(table.len(), 0xa4);
    table.push(InsnInfo::Regular(RegularInsnInfo {
        mnemonic: "shld",
        ops: &[
            OpInfo::RM_16_32_64_DEF_32,
            OpInfo::R_MODRM_16_32_64_DEF_32,
            OpInfo::Imm(ImmOpInfo {
                encoded_size: OpSizeInfo::SZ_ALWAYS_8,
                extended_size: OpSizeInfo::SZ_16_32_64_DEF_32,
                extend_kind: ImmExtendKind::ZeroExtend,
            }),
        ],
    }));
    // 0xa5
    assert_eq!(table.len(), 0xa5);
    table.push(InsnInfo::Regular(RegularInsnInfo {
        mnemonic: "shld",
        ops: &[
            OpInfo::RM_16_32_64_DEF_32,
            OpInfo::R_MODRM_16_32_64_DEF_32,
            OpInfo::ZextSpecificReg(ZextSpecificRegOpInfo {
                reg: SpecificReg::Rcx,
                size: OpSizeInfo::SZ_ALWAYS_8,
                extended_size: OpSizeInfo::SZ_16_32_64_DEF_32,
            }),
        ],
    }));
    // 0xa6 - 0xab
    assert_eq!(table.len(), 0xa6);
    unsupported(&mut table, 6);
    // 0xac
    assert_eq!(table.len(), 0xac);
    table.push(InsnInfo::Regular(RegularInsnInfo {
        mnemonic: "shrd",
        ops: &[
            OpInfo::RM_16_32_64_DEF_32,
            OpInfo::R_MODRM_16_32_64_DEF_32,
            OpInfo::Imm(ImmOpInfo {
                encoded_size: OpSizeInfo::SZ_ALWAYS_8,
                extended_size: OpSizeInfo::SZ_16_32_64_DEF_32,
                extend_kind: ImmExtendKind::ZeroExtend,
            }),
        ],
    }));
    // 0xad
    assert_eq!(table.len(), 0xad);
    table.push(InsnInfo::Regular(RegularInsnInfo {
        mnemonic: "shrd",
        ops: &[
            OpInfo::RM_16_32_64_DEF_32,
            OpInfo::R_MODRM_16_32_64_DEF_32,
            OpInfo::ZextSpecificReg(ZextSpecificRegOpInfo {
                reg: SpecificReg::Rcx,
                size: OpSizeInfo::SZ_ALWAYS_8,
                extended_size: OpSizeInfo::SZ_16_32_64_DEF_32,
            }),
        ],
    }));
    // 0xae
    assert_eq!(table.len(), 0xae);
    unsupported(&mut table, 1);
    // 0xaf
    assert_eq!(table.len(), 0xaf);
    table.push(InsnInfo::Regular(RegularInsnInfo {
        mnemonic: "imul",
        ops: &[OpInfo::R_MODRM_16_32_64_DEF_32, OpInfo::RM_16_32_64_DEF_32],
    }));
    // 0xb0 - 0xb5
    assert_eq!(table.len(), 0xb0);
    unsupported(&mut table, 6);
    // 0xb6
    assert_eq!(table.len(), 0xb6);
    table.push(InsnInfo::Regular(RegularInsnInfo {
        mnemonic: "movzx",
        ops: &[OpInfo::R_MODRM_16_32_64_DEF_32, OpInfo::RM_8],
    }));
    // 0xb7
    assert_eq!(table.len(), 0xb7);
    table.push(InsnInfo::Regular(RegularInsnInfo {
        mnemonic: "movzx",
        ops: &[
            OpInfo::R_MODRM_16_32_64_DEF_32,
            OpInfo::Rm(OpSizeInfo::SZ_ALWAYS_16),
        ],
    }));
    // 0xb8 - 0xbd
    assert_eq!(table.len(), 0xb8);
    unsupported(&mut table, 6);
    // 0xbe
    assert_eq!(table.len(), 0xbe);
    table.push(InsnInfo::Regular(RegularInsnInfo {
        mnemonic: "movsx",
        ops: &[OpInfo::R_MODRM_16_32_64_DEF_32, OpInfo::RM_8],
    }));
    // 0xbf
    assert_eq!(table.len(), 0xbf);
    table.push(InsnInfo::Regular(RegularInsnInfo {
        mnemonic: "movsx",
        ops: &[
            OpInfo::R_MODRM_16_32_64_DEF_32,
            OpInfo::Rm(OpSizeInfo::SZ_ALWAYS_16),
        ],
    }));
    // 0xc0 - 0xff
    assert_eq!(table.len(), 0xc0);
    unsupported(&mut table, 0x40);

    assert_eq!(table.len(), 0x100);

    table
}
