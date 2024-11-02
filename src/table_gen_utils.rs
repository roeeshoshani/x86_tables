use crate::table_types::*;

pub fn simple_binary_op(table: &mut Vec<InsnInfo>, mnemonic: Mnemonic) {
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

pub fn repeat(table: &mut Vec<InsnInfo>, amount: usize, entry: InsnInfo) {
    table.extend(std::iter::repeat_n(entry, amount))
}

pub fn unsupported(table: &mut Vec<InsnInfo>, amount: usize) {
    repeat(
        table,
        amount,
        InsnInfo::Regular(RegularInsnInfo::UNSUPPORTED),
    )
}
