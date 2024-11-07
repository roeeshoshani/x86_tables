use std::{cmp::max, path::PathBuf};

use c_emitter::{min_int_type_required_for_field, CEmitter};
use clap::Parser;
use delve::VariantNames;
use either::Either;
use first_opcode_byte_table::gen_first_opcode_byte_table;
use table_types::*;
use to_snake_case::ToSnakeCase;

mod c_emitter;
mod first_opcode_byte_table;
mod table_gen_utils;
mod table_types;

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

fn op_size_to_c_variant_name(op_size: OpSize) -> String {
    format!("OP_SIZE_{}", op_size as u32)
}

fn vec_dedup<T: Eq>(vec: &mut Vec<T>) {
    let mut i = 0;
    while i < vec.len() {
        // find and remove duplicates
        let mut j = i + 1;
        while j < vec.len() {
            if vec[i] == vec[j] {
                // found duplicate, remove it
                vec.swap_remove(j);
            } else {
                j += 1;
            }
        }

        i += 1;
    }
}

fn iter_collect_unique<T: Eq, I: Iterator<Item = T>>(iter: I) -> Vec<T> {
    let mut result: Vec<T> = iter.collect();
    vec_dedup(&mut result);
    result
}

fn find_index<'a, T: Eq + 'a, C>(item: &T, collection: C) -> usize
where
    C: IntoIterator<Item = &'a T>,
{
    collection.into_iter().position(|x| x == item).unwrap()
}

fn find_first_op_index(ops_info: Ops, uniq_ops_infos: &[Ops]) -> usize {
    uniq_ops_infos
        .iter()
        .take_while(|x| **x != ops_info)
        .map(|x| x.len())
        .sum()
}

struct GeneratedCode {
    types_file: CEmitter,
    tables_file: CEmitter,
}
fn generate_code() -> GeneratedCode {
    let mut types_file = CEmitter::new();
    let mut tables_file = CEmitter::new();

    types_file.pragma_once();
    types_file.include_system("stdint.h");

    let first_opcode_byte_table = gen_first_opcode_byte_table();

    let mut uniq_mnemonics = iter_collect_unique(table_all_mnemonics(&first_opcode_byte_table));
    // a psuedo mnemonic used to represent the fact that this instruction required further identification using the reg field
    // of the modrm field.
    uniq_mnemonics.push(MNEMONIC_MODRM_REG_OPCODE_EXT);

    let uniq_ops_infos = iter_collect_unique(table_all_ops(&first_opcode_byte_table));
    let laid_out_ops_infos = uniq_ops_infos.iter().map(|x| x.iter()).flatten();
    let laid_out_ops_infos_len = laid_out_ops_infos.clone().count();
    let insn_max_ops = uniq_ops_infos
        .iter()
        .map(|cur_ops| cur_ops.len())
        .max()
        .unwrap();

    types_file.define("X86_TABLES_INSN_MAX_OPS", &insn_max_ops.to_string());

    let uniq_op_infos = iter_collect_unique(laid_out_ops_infos.cloned());
    let uniq_op_size_infos = iter_collect_unique(
        uniq_op_infos
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

    let uniq_modrm_reg_opcode_ext_tables = iter_collect_unique(
        table_all_modrm_reg_opcode_ext_tables(&first_opcode_byte_table).cloned(),
    );

    types_file.emit_enum(
        "mnemonic_t",
        uniq_mnemonics
            .iter()
            .map(|x| mnemonic_to_c_variant_name(*x)),
    );

    let mut insn_info_union = types_file.begin_union("insn_info_t");
    insn_info_union.bit_field("mnemonic", uniq_mnemonics.len());
    insn_info_union
        .begin_embedded_struct("regular")
        .bit_field("mnemonic", uniq_mnemonics.len())
        .bit_field(
            "first_op_index",
            max(
                laid_out_ops_infos_len,
                uniq_modrm_reg_opcode_ext_tables.len(),
            ),
        )
        .bit_field("ops_amount", insn_max_ops + 1)
        .emit();
    insn_info_union
        .begin_embedded_struct("modrm_reg_opcode_ext")
        .bit_field("mnemonic", uniq_mnemonics.len())
        .bit_field(
            "modrm_reg_table_index",
            uniq_modrm_reg_opcode_ext_tables.len(),
        )
        .emit();
    insn_info_union.emit();

    types_file.emit_enum(
        "op_size_t",
        OpSize::VARIANT_NAMES
            .iter()
            .map(|x| format!("OP_SIZE_{}", x.strip_prefix("S").unwrap())),
    );

    types_file
        .begin_struct("op_size_info_t")
        .bit_field("with_operand_size_override", OpSize::VARIANT_NAMES.len())
        .bit_field("mode_32", OpSize::VARIANT_NAMES.len())
        .bit_field("mode_64", OpSize::VARIANT_NAMES.len())
        .bit_field("mode_64_with_rex_w", OpSize::VARIANT_NAMES.len())
        .emit();

    types_file.emit_enum(
        "op_kind_t",
        OpInfo::VARIANT_NAMES
            .into_iter()
            .map(|x| op_kind_to_c_variant_name(x)),
    );

    types_file.emit_enum(
        "imm_ext_kind_t",
        ImmExtendKind::VARIANT_NAMES
            .into_iter()
            .map(|x| imm_ext_kind_to_c_variant_name(x)),
    );

    types_file.emit_enum(
        "reg_encoding_t",
        RegEncoding::VARIANT_NAMES
            .into_iter()
            .map(|x| reg_encoding_to_c_variant_name(x)),
    );

    types_file.emit_enum(
        "specific_reg_t",
        SpecificReg::VARIANT_NAMES
            .into_iter()
            .map(|x| specific_reg_to_c_variant_name(x)),
    );

    types_file.emit_enum(
        "specific_imm_t",
        SpecificImm::VARIANT_NAMES
            .into_iter()
            .map(|x| specific_imm_to_c_variant_name(x)),
    );

    let mut op_info_union = types_file.begin_tagged_union("op_info_t", OpInfo::VARIANT_NAMES.len());
    op_info_union
        .begin_struct_variant("imm")
        .bit_field("encoded_size_info_index", uniq_op_size_infos.len())
        .bit_field("extended_size_info_index", uniq_op_size_infos.len())
        .bit_field("extend_kind", ImmExtendKind::VARIANT_NAMES.len())
        .emit();
    op_info_union
        .begin_struct_variant("specific_imm")
        .bit_field("operand_size_info_index", uniq_op_size_infos.len())
        .bit_field("value", SpecificImm::VARIANT_NAMES.len())
        .emit();
    op_info_union
        .begin_struct_variant("reg")
        .bit_field("size_info_index", uniq_op_size_infos.len())
        .bit_field("encoding", RegEncoding::VARIANT_NAMES.len())
        .emit();
    op_info_union
        .begin_struct_variant("rm")
        .bit_field("size_info_index", uniq_op_size_infos.len())
        .emit();
    op_info_union
        .begin_struct_variant("specific_reg")
        .bit_field("size_info_index", uniq_op_size_infos.len())
        .bit_field("reg", SpecificReg::VARIANT_NAMES.len())
        .emit();
    op_info_union
        .begin_struct_variant("zext_specific_reg")
        .bit_field("size_info_index", uniq_op_size_infos.len())
        .bit_field("extended_size_info_index", uniq_op_size_infos.len())
        .bit_field("reg", SpecificReg::VARIANT_NAMES.len())
        .emit();
    op_info_union
        .begin_struct_variant("rel")
        .bit_field("size_info_index", uniq_op_size_infos.len())
        .emit();
    op_info_union
        .begin_struct_variant("mem_offset")
        .bit_field("mem_operand_size_info_index", uniq_op_size_infos.len())
        .emit();
    op_info_union
        .begin_struct_variant("implicit")
        .bit_field("size_info_index", uniq_op_size_infos.len())
        .emit();
    op_info_union.begin_struct_variant("cond").emit();
    op_info_union.emit();

    let mut op_size_info_table = tables_file.begin_table("op_size_info_t", "op_size_infos_table");
    for op_size_info in &uniq_op_size_infos {
        op_size_info_table
            .begin_entry()
            .field(
                "with_operand_size_override",
                &op_size_to_c_variant_name(op_size_info.with_operand_size_override),
            )
            .field("mode_32", &op_size_to_c_variant_name(op_size_info.mode_32))
            .field("mode_64", &op_size_to_c_variant_name(op_size_info.mode_64))
            .field(
                "mode_64_with_rex_w",
                &op_size_to_c_variant_name(op_size_info.mode_64_with_rex_w),
            )
            .emit();
    }
    op_size_info_table.emit();

    let mut op_info_table = tables_file.begin_table("op_info_t", "op_infos_table");
    for op_info in &uniq_op_infos {
        let mut entry = op_info_table.begin_entry();

        let op_kind_c_variant = op_kind_to_c_variant_name(op_info.into());

        match op_info {
            OpInfo::Imm(imm) => entry
                .begin_struct_field("imm")
                .field("kind", &op_kind_c_variant)
                .field_int(
                    "encoded_size_info_index",
                    find_index(&imm.encoded_size, &uniq_op_size_infos),
                )
                .field_int(
                    "extended_size_info_index",
                    find_index(&imm.encoded_size, &uniq_op_size_infos),
                )
                .field(
                    "extend_kind",
                    &imm_ext_kind_to_c_variant_name((&imm.extend_kind).into()),
                )
                .emit(),
            OpInfo::SpecificImm(specific_imm) => entry
                .begin_struct_field("specific_imm")
                .field("kind", &op_kind_c_variant)
                .field_int(
                    "operand_size_info_index",
                    find_index(&specific_imm.operand_size, &uniq_op_size_infos),
                )
                .field(
                    "value",
                    &specific_imm_to_c_variant_name((&specific_imm.value).into()),
                )
                .emit(),
            OpInfo::Reg(reg) => entry
                .begin_struct_field("reg")
                .field("kind", &op_kind_c_variant)
                .field_int(
                    "size_info_index",
                    find_index(&reg.size, &uniq_op_size_infos),
                )
                .field(
                    "encoding",
                    &reg_encoding_to_c_variant_name((&reg.encoding).into()),
                )
                .emit(),
            OpInfo::Rm(rm_size) => entry
                .begin_struct_field("rm")
                .field("kind", &op_kind_c_variant)
                .field_int("size_info_index", find_index(rm_size, &uniq_op_size_infos))
                .emit(),
            OpInfo::SpecificReg(specific_reg) => entry
                .begin_struct_field("specific_reg")
                .field("kind", &op_kind_c_variant)
                .field_int(
                    "size_info_index",
                    find_index(&specific_reg.size, &uniq_op_size_infos),
                )
                .field(
                    "reg",
                    &specific_reg_to_c_variant_name((&specific_reg.reg).into()),
                )
                .emit(),
            OpInfo::ZextSpecificReg(zext_specific_reg) => entry
                .begin_struct_field("zext_specific_reg")
                .field("kind", &op_kind_c_variant)
                .field_int(
                    "size_info_index",
                    find_index(&zext_specific_reg.size, &uniq_op_size_infos),
                )
                .field_int(
                    "extended_size_info_index",
                    find_index(&zext_specific_reg.extended_size, &uniq_op_size_infos),
                )
                .field(
                    "reg",
                    &specific_reg_to_c_variant_name((&zext_specific_reg.reg).into()),
                )
                .emit(),
            OpInfo::Rel(rel_size) => entry
                .begin_struct_field("rel")
                .field("kind", &op_kind_c_variant)
                .field_int("size_info_index", find_index(rel_size, &uniq_op_size_infos))
                .emit(),
            OpInfo::MemOffset(mem_offset) => entry
                .begin_struct_field("mem_offset")
                .field("kind", &op_kind_c_variant)
                .field_int(
                    "mem_operand_size_info_index",
                    find_index(&mem_offset.mem_operand_size, &uniq_op_size_infos),
                )
                .emit(),
            OpInfo::Implicit(implicit_size) => entry
                .begin_struct_field("implicit")
                .field("kind", &op_kind_c_variant)
                .field_int(
                    "size_info_index",
                    find_index(implicit_size, &uniq_op_size_infos),
                )
                .emit(),
            OpInfo::Cond => entry
                .begin_struct_field("cond")
                .field("kind", &op_kind_c_variant)
                .emit(),
        }
        entry.emit();
    }
    op_info_table.emit();

    let mut laid_out_ops_infos_table = tables_file.begin_table(
        &min_int_type_required_for_field(uniq_op_infos.len()),
        "laid_out_ops_infos_table",
    );
    for &ops_info in &uniq_ops_infos {
        for op_info in ops_info {
            laid_out_ops_infos_table.int_entry(find_index(op_info, &uniq_op_infos))
        }
    }
    laid_out_ops_infos_table.emit();

    let mut first_opcde_byte_table =
        tables_file.begin_table("insn_info_t", "first_opcode_byte_table");
    for insn_info in &first_opcode_byte_table {
        let mut entry = first_opcde_byte_table.begin_entry();
        match insn_info {
            InsnInfo::Regular(info) => entry
                .begin_struct_field("regular")
                .field("mnemonic", &mnemonic_to_c_variant_name(info.mnemonic))
                .field_int(
                    "first_op_index",
                    find_first_op_index(info.ops, &uniq_ops_infos),
                )
                .field_int("ops_amount", info.ops.len())
                .emit(),
            InsnInfo::ModrmRegOpcodeExt(modrm_reg_table) => entry
                .begin_struct_field("modrm_reg_opcode_ext")
                .field(
                    "mnemonic",
                    &mnemonic_to_c_variant_name(MNEMONIC_MODRM_REG_OPCODE_EXT),
                )
                .field_int(
                    "modrm_reg_table_index",
                    find_index(modrm_reg_table, &uniq_modrm_reg_opcode_ext_tables),
                )
                .emit(),
        }
        entry.emit();
    }
    first_opcde_byte_table.emit();
    GeneratedCode {
        types_file,
        tables_file,
    }
}

#[derive(Parser)]
struct Cli {
    output_dir: PathBuf,
}

fn main() {
    let cli = Cli::parse();
    let generated_code = generate_code();
    std::fs::write(
        cli.output_dir.join("types.h"),
        generated_code.types_file.code(),
    )
    .unwrap();
    std::fs::write(
        cli.output_dir.join("tables.h"),
        generated_code.tables_file.code(),
    )
    .unwrap();
}
