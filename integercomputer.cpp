#include <cstdio>
#include <cstring>
#include <assert.h>
#include <cstdlib>
#define MAX_MEM (65536*4)
#define OP_ADD 1
#define OP_MULT 2
#define OP_INPUT 3
#define OP_OUTPUT 4
#define OP_JUMP_IF_TRUE 5
#define OP_JUMP_IF_FALSE 6
#define OP_CMP_LT 7
#define OP_CMP_EQ 8
#define OP_ADJUST_BASE 9
#define OP_HALT 99

#define MODE_POSITION  0
#define MODE_INMEDIATE 1
#define MODE_RELATIVE 2

#define OK 0
#define ERR_NOT_FOUND 1
#define ERR_INVALID_INST 2
#define OP(arg) (arg % 100)
#define MODE1(arg) ((arg / 100) % 10)
#define MODE2(arg) ((arg / 1000) % 10)
#define MODE3(arg) ((arg / 10000) % 10)
#define PANIC(msg) { printf(msg "\n"); exit(-1); }
#define DEBUG 1
#ifdef DEBUG
#define pdebug(args...) do { printf(args); } while(0)
#else
#define pdebug(args...)
#endif
typedef long long word_t;


size_t load(char const *program_path, word_t *mem) {
  FILE *program_file = fopen(program_path, "r");
  size_t i = 0;
  while (1) {
      fscanf(program_file, "%lld,", &mem[i++]);
      if (feof(program_file)) {
        fclose(program_file);
        return i;
      }
  }
}

word_t get_val(word_t *mem, word_t op, unsigned char mode, word_t base) {
  switch (mode) {
    case MODE_POSITION:
      return mem[op];
    case MODE_INMEDIATE:
      return op;
    case MODE_RELATIVE:
      return mem[base + op];
    default:
      PANIC("UNKNOWN MODE");
  }
}

word_t &get_cell(word_t *mem, word_t op, unsigned char mode, word_t base) {
  switch (mode) {
    case MODE_POSITION:
      return mem[op];
    case MODE_INMEDIATE:
      PANIC("INMEDIATE MODE NOT SUPPORTED FOR OUTPUT");
    case MODE_RELATIVE:
      return mem[base + op];
    default:
      PANIC("UNKNOWN MODE");
  }

}

int execute(word_t *mem) {
  // fast way but with high chances of getting segfaults
  size_t pc = 0;
  word_t base = 0;
  word_t op1, op2;
  while (1) {
    word_t opcode = mem[pc];
    switch (OP(opcode)) {
      case OP_ADD:
        op1 = get_val(mem, mem[pc + 1], MODE1(opcode), base);
        op2 =  get_val(mem, mem[pc + 2], MODE2(opcode), base);
        get_cell(mem, mem[pc + 3], MODE3(opcode), base) = op1 + op2;
        pc += 4;
        break;
      case OP_MULT:
        op1 = get_val(mem, mem[pc + 1], MODE1(opcode), base);
        op2 =  get_val(mem, mem[pc + 2], MODE2(opcode), base);
        get_cell(mem, mem[pc + 3], MODE3(opcode), base) = op1 * op2;
        pc += 4;
        break;
      case OP_INPUT:
        word_t input;
        assert(scanf("%lld", &input) == 1);
        get_cell(mem, mem[pc + 1], MODE1(opcode), base) = input;
        pc += 2;
        break;
      case OP_OUTPUT:
        printf("%lld\n", get_val(mem, mem[pc + 1], MODE1(opcode), base));
        pc += 2;
        break;
      case OP_JUMP_IF_TRUE:
        if (get_val(mem, mem[pc + 1], MODE1(opcode), base))
          pc = get_val(mem, mem[pc + 2], MODE2(opcode), base);
        else
          pc += 3;
        break;
      case OP_JUMP_IF_FALSE:
        if (!get_val(mem, mem[pc + 1], MODE1(opcode), base))
          pc = get_val(mem, mem[pc + 2], MODE2(opcode), base);
        else
          pc += 3;
        break;
      case OP_CMP_LT:
        if (get_val(mem, mem[pc + 1], MODE1(opcode), base) <
            get_val(mem, mem[pc + 2], MODE2(opcode), base))
          get_cell(mem, mem[pc + 3], MODE3(opcode), base) = 1;
        else
          get_cell(mem, mem[pc + 3], MODE3(opcode), base) = 0;
        pc += 4;
        break;
      case OP_CMP_EQ:
        if (get_val(mem, mem[pc + 1], MODE1(opcode), base) ==
            get_val(mem, mem[pc + 2], MODE2(opcode), base))
          get_cell(mem, mem[pc + 3], MODE3(opcode), base) = 1;
        else
          get_cell(mem, mem[pc + 3], MODE3(opcode), base) = 0;
        pc += 4;
        break;
      case OP_ADJUST_BASE:
        base += get_val(mem, mem[pc + 1], MODE1(opcode), base);
        pc += 2;
        break;
      case OP_HALT:
        return OK;
      default:
        printf("Invalid instruction %lld\tPC: %lu\n",opcode, pc);
        return ERR_INVALID_INST;
    }
  }
  return OK;
}

int day2() {
    word_t mem[MAX_MEM];
    word_t mem_bck[MAX_MEM];
    size_t program_size = load("program_day2", mem);
    printf("Loaded program of size: %lu\n", program_size);
    memcpy(mem_bck, mem, program_size*sizeof(mem[0]));
    for(int i=0; i < 100; i++) {
      for(int j=0; j < 100; j++) {
        mem[1] = i;
        mem[2] = j;
        int rc;
        if ((rc=execute(mem)) != 0)
          return rc;
        if (mem[0] == 19690720) {
          printf("found: i=%d j=%d result=%d\n", i, j, i * 100 + j);
          return OK;
        }
        memcpy(mem, mem_bck, program_size*sizeof(mem[0]));
      }
    }
    return ERR_NOT_FOUND;
}

int run(char const *program_path) {
    word_t mem[MAX_MEM];
    size_t program_size;
    program_size = load(program_path, mem);
    printf("Loaded program of size: %lu\n", program_size);
    int rc = execute(mem);
    printf("rc: %d\n", rc);
    return rc;
}

int main(int argc, char **argv) {
  // return day2();
  char program_path[256];
  scanf("%s", program_path);
  return run(program_path);
}

