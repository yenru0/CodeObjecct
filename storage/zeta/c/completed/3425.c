#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

typedef enum {
    POP,
    NUM,

    INV,
    DUP,
    SWP,

    ADD,
    SUB,
    MUL,
    DIV,
    MOD,

    END,
    ERR,

    QUIT,
} InstructionType;

typedef struct {
    InstructionType type;
    int32_t num;
} Instruction;

/**
 * Stack Implementation
 */

typedef struct {
    int32_t size;
    int32_t data[1003];
} Stack;

int32_t stack_is_empty(Stack *stack) {
    return stack->size == 0;
}

int32_t stack_push(Stack *stack, int32_t num) {
    stack->data[stack->size] = num;
    stack->size++;
    return 1;
}

int32_t stack_pop(Stack *stack, int32_t *out) {
    if (stack_is_empty(stack)) {
        return 0;
    } else {
        *out = stack->data[stack->size - 1];
        stack->size--;
        return 1;
    }
}

int32_t stack_clear(Stack *stack) {
    stack->size = 0;
    return 1;
}

int32_t stack_free(Stack *stack) {
    free(stack);
    return 1;
}

/**
 * Program Executor
 */

typedef struct {
    size_t pc;
    size_t inst_size;
    Stack *stack;
    char *string_buff;
    Instruction *instructions;
} ProgramExecutor;

void init_executor(ProgramExecutor *executor) {
    if (executor->stack == NULL) {
        executor->stack = malloc(sizeof(Stack));
    }

    if (executor->string_buff == NULL) {
        executor->string_buff = calloc(100, sizeof(char));
    }

    if (executor->instructions == NULL) {
        executor->instructions = calloc(100002, sizeof(Instruction));
    }
    executor->pc = 0;
    executor->inst_size = 0;
    stack_clear(executor->stack);
}

void reset_executor(ProgramExecutor *executor) {
    stack_clear(executor->stack);
    executor->pc = 0;
}

void delete_executor(ProgramExecutor *executor) {
    free(executor->string_buff);
    stack_free(executor->stack);
    free(executor->instructions);
    free(executor);
}

int32_t fetch_instruction(ProgramExecutor *executor) {
    Instruction inst;
    int num;
    scanf("%s", executor->string_buff);

    if (!strcmp(executor->string_buff, "POP")) {
        inst.type = POP;
    } else if (!strcmp(executor->string_buff, "NUM")) {
        inst.type = NUM;
        scanf("%d", &num);
        inst.num = num;
    } else if (!strcmp(executor->string_buff, "INV")) {
        inst.type = INV;
    } else if (!strcmp(executor->string_buff, "DUP")) {
        inst.type = DUP;
    } else if (!strcmp(executor->string_buff, "SWP")) {
        inst.type = SWP;
    } else if (!strcmp(executor->string_buff, "ADD")) {
        inst.type = ADD;
    } else if (!strcmp(executor->string_buff, "SUB")) {
        inst.type = SUB;
    } else if (!strcmp(executor->string_buff, "MUL")) {
        inst.type = MUL;
    } else if (!strcmp(executor->string_buff, "DIV")) {
        inst.type = DIV;
    } else if (!strcmp(executor->string_buff, "MOD")) {
        inst.type = MOD;
    } else if (!strcmp(executor->string_buff, "END")) {
        inst.type = END;
    } else if (!strcmp(executor->string_buff, "QUIT")) {
        inst.type = QUIT;
    } else {
        inst.type = ERR;
    }

    executor->instructions[executor->inst_size] = inst;
    executor->inst_size++;
    if (inst.type == QUIT || inst.type == ERR) {
        return -1;
    } else if (inst.type == END) {
        return 0;
    } else {
        return 1;
    }
}

int32_t execute_once(ProgramExecutor *executor) {
    Instruction instruction = executor->instructions[executor->pc];
    executor->pc++;
    int error_flag = 0;
    int temp = 0, temp2;
    switch (instruction.type) {
        case POP:
            if (!stack_pop(executor->stack, &temp)) {
                error_flag = 1;
            }
            break;
        case NUM:
            if (!stack_push(executor->stack, instruction.num)) {
                error_flag = 1;
            }
            break;
        case INV:
            if (!stack_pop(executor->stack, &temp)) error_flag = 1;
            if (!stack_push(executor->stack, -temp)) error_flag = 1;
            break;
        case DUP:
            if (!stack_pop(executor->stack, &temp)) error_flag = 1;
            if (!stack_push(executor->stack, temp)) error_flag = 1;
            if (!stack_push(executor->stack, temp)) error_flag = 1;
            break;
        case SWP:
            if (!stack_pop(executor->stack, &temp)) error_flag = 1;
            if (!stack_pop(executor->stack, &temp2)) error_flag = 1;
            if (!stack_push(executor->stack, temp)) error_flag = 1;
            if (!stack_push(executor->stack, temp2)) error_flag = 1;
            break;
        case ADD:
            if (!stack_pop(executor->stack, &temp)) error_flag = 1;
            if (!stack_pop(executor->stack, &temp2)) error_flag = 1;
            temp = temp2 + temp;
            if (abs(temp) > 1000000000) {
                error_flag = 1;
                break;
            }
            if (!stack_push(executor->stack, temp)) error_flag = 1;
            break;
        case SUB:
            if (!stack_pop(executor->stack, &temp)) error_flag = 1;
            if (!stack_pop(executor->stack, &temp2)) error_flag = 1;
            temp = temp2 - temp;
            if (abs(temp) > 1000000000) {
                error_flag = 1;
                break;
            }
            if (!stack_push(executor->stack, temp)) error_flag = 1;
            break;
        case MUL:
            if (!stack_pop(executor->stack, &temp)) error_flag = 1;
            if (!stack_pop(executor->stack, &temp2)) error_flag = 1;

            /* considering mult overflow */
            if (temp == 0 || temp2 == 0) {
                temp = temp * temp2;
            } else if (abs(temp) > abs(1000000000 / temp2)) {
                error_flag = 1;
                break;
            } else {
                temp = temp * temp2;
            }

            if (!stack_push(executor->stack, temp)) error_flag = 1;
            break;
        case DIV:
            if (!stack_pop(executor->stack, &temp)) error_flag = 1;
            if (!stack_pop(executor->stack, &temp2)) error_flag = 1;
            if (temp == 0) {
                error_flag = 1;
                break;
            }
            temp = temp2 / temp;
            if (abs(temp) > 1000000000) {
                error_flag = 1;
                break;
            }
            if (!stack_push(executor->stack, temp)) error_flag = 1;
            break;
        case MOD:
            if (!stack_pop(executor->stack, &temp)) error_flag = 1;
            if (!stack_pop(executor->stack, &temp2)) error_flag = 1;
            if (temp == 0) {
                error_flag = 1;
                break;
            }
            temp = temp2 % temp;
            if (abs(temp) > 1000000000) {
                error_flag = 1;
                break;
            }
            if (!stack_push(executor->stack, temp)) error_flag = 1;
            break;
        case END:
            return 0;
        case QUIT:
        case ERR:
            return -1;
    }
    if (error_flag) {
        return -1;
    }
    return 1;
}

int main(void) {
    ProgramExecutor *executor = malloc(sizeof(ProgramExecutor));
    executor->stack = NULL;
    executor->string_buff = NULL;
    executor->instructions = NULL;
    while (1) {
        init_executor(executor);
        int32_t signal;

        signal = fetch_instruction(executor);
        if (signal == -1) {
            break;
        } else if (signal == 0) {

        } else {
            while (fetch_instruction(executor) == 1) {
            }
        }

        size_t tc;
        int32_t input;
        int32_t temp;

        scanf("%zu", &tc);
        for (size_t i = 0; i < tc; i++) {
            reset_executor(executor);
            scanf("%d", &input);
            stack_push(executor->stack, input);

            do {
                signal = execute_once(executor);
            } while (signal == 1);

            if (signal == -1) {
                printf("ERROR\n");
            } else {
                if (executor->stack->size == 1) {
                    stack_pop(executor->stack, &temp);
                    printf("%d\n", temp);
                } else {
                    printf("ERROR\n");
                }
            }
        }
        printf("\n");
    }

    delete_executor(executor);
    executor = NULL;

    return 0;
}