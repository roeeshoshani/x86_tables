use std::{cmp::max, collections::HashSet, hash::Hash};

use c_emitter::{gen_bit_field_min_size, CEmitter};
use delve::VariantNames;
use either::Either;
use first_opcode_byte_table::gen_first_opcode_byte_table;
use table_types::*;
use to_snake_case::ToSnakeCase;

mod c_emitter;
mod first_opcode_byte_table;
mod table_gen_utils;
mod table_types;

const CODEGEN_TYPES: &str = include_str!("codegen_types.h");
const MNEMONIC_MODRM_REG_OPCODE_EXT: &str = "modrm_reg_opcode_ext";

fn table_all_regular_insn_infos<'a>(
    table: &'a [InsnInfo],
) -> impl Iterator<Item = &'a RegularInsnInfo> + 'a {
    table
        .iter()
        .map(|insn_info| match insn_info {
            InsnInfo::Regular(info) => Either::Left(std::iter::once(info)),
            InsnInfo::ModrmRegOpcodeExt(modrm_reg_opcode_ext_info) => {
                Either::Right(modrm_reg_opcode_ext_info.by_reg_value.iter())
            }
        })
        .flatten()
}

fn table_all_mnemonics<'a>(table: &'a [InsnInfo]) -> impl Iterator<Item = Mnemonic> + 'a {
    table_all_regular_insn_infos(table).map(|insn_info| insn_info.mnemonic)
}

fn table_all_ops<'a>(table: &'a [InsnInfo]) -> impl Iterator<Item = Ops> + 'a {
    table_all_regular_insn_infos(table).map(|insn_info| insn_info.ops)
}

fn table_all_modrm_reg_opcode_ext_tables<'a>(
    table: &'a [InsnInfo],
) -> impl Iterator<Item = &'a ModrmRegOpcodeExtInsnInfo> + 'a {
    table.iter().filter_map(|insn_info| match insn_info {
        InsnInfo::Regular(_) => None,
        InsnInfo::ModrmRegOpcodeExt(inner_table) => Some(inner_table),
    })
}

fn mnemonic_to_c_variant_name(mnemonic: Mnemonic) -> String {
    format!("MNEMONIC_{}", mnemonic.to_uppercase())
}

fn op_kind_to_c_variant_name(op_kind_variant_name: &str) -> String {
    format!(
        "OP_KIND_{}",
        op_kind_variant_name.to_snake_case().to_uppercase()
    )
}

fn imm_ext_kind_to_c_variant_name(imm_ext_kind_variant_name: &str) -> String {
    format!(
        "IMM_EXT_{}",
        imm_ext_kind_variant_name.to_snake_case().to_uppercase()
    )
}

fn reg_encoding_to_c_variant_name(reg_encoding_variant_name: &str) -> String {
    format!(
        "REG_ENC_{}",
        reg_encoding_variant_name.to_snake_case().to_uppercase()
    )
}

fn specific_reg_to_c_variant_name(specific_reg_variant_name: &str) -> String {
    format!(
        "SPECIFIC_REG_{}",
        specific_reg_variant_name.to_snake_case().to_uppercase()
    )
}

fn specific_imm_to_c_variant_name(specific_imm_value_variant_name: &str) -> String {
    format!(
        "SPECIFIC_IMM_{}",
        specific_imm_value_variant_name
            .to_snake_case()
            .to_uppercase()
    )
}

fn iter_collect_unique<T: Eq + Hash, I: Iterator<Item = T>>(iter: I) -> Vec<T> {
    let set: HashSet<T> = iter.collect();
    set.into_iter().collect()
}

fn find_index<'a, T: Eq + 'a, C>(item: T, collection: C) -> usize
where
    C: IntoIterator<Item = &'a T>,
{
    collection.into_iter().position(|x| *x == item).unwrap()
}

fn main() {
    let mut emitter = CEmitter::new();
    let first_opcode_byte_table = gen_first_opcode_byte_table();

    emitter.emit_system_include("stdint.h");

    let mut mnemonics = iter_collect_unique(table_all_mnemonics(&first_opcode_byte_table));
    // a psuedo mnemonic used to represent the fact that this instruction required further identification using the reg field
    // of the modrm field.
    mnemonics.push(MNEMONIC_MODRM_REG_OPCODE_EXT);

    let ops_infos = iter_collect_unique(table_all_ops(&first_opcode_byte_table));
    let laid_out_ops_infos = ops_infos.iter().map(|x| x.iter()).flatten();
    let laid_out_ops_infos_len = laid_out_ops_infos.clone().count();
    let insn_max_ops = ops_infos.iter().map(|cur_ops| cur_ops.len()).max().unwrap();

    let op_infos = iter_collect_unique(laid_out_ops_infos);
    let op_size_infos = iter_collect_unique(
        op_infos
            .iter()
            .map(|op_info| match op_info {
                OpInfo::Imm(imm) => vec![imm.encoded_size.clone(), imm.extended_size.clone()],
                OpInfo::SpecificImm(imm) => vec![imm.operand_size.clone()],
                OpInfo::Reg(reg) => vec![reg.size.clone()],
                OpInfo::Rm(size) => vec![size.clone()],
                OpInfo::SpecificReg(reg) => vec![reg.size.clone()],
                OpInfo::ZextSpecificReg(reg) => vec![reg.size.clone(), reg.extended_size.clone()],
                OpInfo::Rel(size) => vec![size.clone()],
                OpInfo::MemOffset(moffset) => vec![moffset.mem_operand_size.clone()],
                OpInfo::Implicit(size) => vec![size.clone()],
                OpInfo::Cond => vec![],
            })
            .flatten(),
    );

    let modrm_reg_opcode_ext_tables = iter_collect_unique(
        table_all_modrm_reg_opcode_ext_tables(&first_opcode_byte_table).cloned(),
    );

    emitter.emit_enum(
        "mnemonic_t",
        mnemonics.iter().map(|x| mnemonic_to_c_variant_name(*x)),
    );

    emitter
        .begin_struct("insn_info_t")
        .bit_field_min_size("mnemonic", mnemonics.len())
        .bit_field_min_size(
            "first_op_index",
            max(laid_out_ops_infos_len, modrm_reg_opcode_ext_tables.len()),
        )
        .bit_field_min_size("ops_amount", insn_max_ops + 1)
        .emit();

    emitter.emit_enum(
        "op_kind_t",
        OpInfo::VARIANT_NAMES
            .into_iter()
            .map(|x| op_kind_to_c_variant_name(x)),
    );

    emitter.emit_enum(
        "imm_ext_kind_t",
        ImmExtendKind::VARIANT_NAMES
            .into_iter()
            .map(|x| imm_ext_kind_to_c_variant_name(x)),
    );

    emitter.emit_enum(
        "reg_encoding_t",
        RegEncoding::VARIANT_NAMES
            .into_iter()
            .map(|x| reg_encoding_to_c_variant_name(x)),
    );

    emitter.emit_enum(
        "specific_reg_t",
        SpecificReg::VARIANT_NAMES
            .into_iter()
            .map(|x| specific_reg_to_c_variant_name(x)),
    );

    emitter.emit_enum(
        "specific_imm_t",
        SpecificImm::VARIANT_NAMES
            .into_iter()
            .map(|x| specific_imm_to_c_variant_name(x)),
    );

    let mut op_info_union_emitter =
        emitter.begin_tagged_union("op_info_t", OpInfo::VARIANT_NAMES.len());
    op_info_union_emitter
        .begin_struct_variant("imm")
        .bit_field_min_size("encoded_size_info_index", op_size_infos.len())
        .bit_field_min_size("extended_size_info_index", op_size_infos.len())
        .bit_field_min_size("extend_kind", ImmExtendKind::VARIANT_NAMES.len())
        .emit();
    op_info_union_emitter
        .begin_struct_variant("specific_imm")
        .bit_field_min_size("operand_size_info_index", op_size_infos.len())
        .bit_field_min_size("value", SpecificImm::VARIANT_NAMES.len())
        .emit();
    op_info_union_emitter
        .begin_struct_variant("reg")
        .bit_field_min_size("size_info_index", op_size_infos.len())
        .bit_field("encoding", "uint8_t", RegEncoding::VARIANT_NAMES.len())
        .emit();
    op_info_union_emitter
        .begin_struct_variant("rm")
        .bit_field_min_size("size_info_index", op_size_infos.len())
        .emit();
    op_info_union_emitter
        .begin_struct_variant("specific_reg")
        .bit_field_min_size("size_info_index", op_size_infos.len())
        .bit_field_min_size("reg", SpecificReg::VARIANT_NAMES.len())
        .emit();
    op_info_union_emitter
        .begin_struct_variant("zext_specific_reg")
        .bit_field_min_size("size_info_index", op_size_infos.len())
        .bit_field_min_size("extended_size_info_index", op_size_infos.len())
        .bit_field_min_size("reg", SpecificReg::VARIANT_NAMES.len())
        .emit();
    op_info_union_emitter
        .begin_struct_variant("rel")
        .bit_field_min_size("size_info_index", op_size_infos.len())
        .emit();
    op_info_union_emitter
        .begin_struct_variant("mem_offset")
        .bit_field_min_size("mem_operand_size_info_index", op_size_infos.len())
        .emit();
    op_info_union_emitter
        .begin_struct_variant("implicit")
        .bit_field_min_size("size_info_index", op_size_infos.len())
        .emit();
    op_info_union_emitter.begin_struct_variant("cond").emit();
    op_info_union_emitter.emit();

    let mut first_opcde_byte_table_emitter =
        emitter.begin_table("insn_info_t", "first_opcode_byte");
    for insn_info in first_opcode_byte_table {
        let (mnemonic, ops_index) = match insn_info {
            InsnInfo::Regular(info) => (info.mnemonic, find_index(info.ops, &ops_infos)),
            InsnInfo::ModrmRegOpcodeExt(modrm_reg_table) => (
                MNEMONIC_MODRM_REG_OPCODE_EXT,
                find_index(modrm_reg_table, &modrm_reg_opcode_ext_tables),
            ),
        };
        first_opcde_byte_table_emitter
            .begin_entry()
            .field("mnemonic", &mnemonic_to_c_variant_name(mnemonic))
            .field("ops_index", &ops_index.to_string())
            .emit()
    }
    first_opcde_byte_table_emitter.emit();

    // let mut laid_out_ops_table_emitter =emitter.begin_table("", )

    println!("{}", emitter.code());
}
