#include <stdint.h>

typedef enum {
    OP_KIND_IMM,
    OP_KIND_SPECIFIC_IMM,
    OP_KIND_REG,
    OP_KIND_RM,
    OP_KIND_SPECIFIC_REG,
    OP_KIND_ZEXT_SPECIFIC_REG,
    OP_KIND_REL,
    OP_KIND_MEM_OFFSET,
    OP_KIND_IMPLICIT,
    OP_KIND_COND,
} op_kind_t;

typedef union {
    op_kind_t kind:4;
    struct {
        uint8_t encoded_size_info_index: 
    } imm;
} op_info_t;
