#include <stdlib.h>
#include <stdio.h>
#include "memstack.h"

Memstack* new_memstack() 
{
	Memstack* stack = malloc(sizeof(Memstack));
	stack->size = 0;
	return stack;
}

void* mem_push(Memstack* stack, void* ptr, void (*dealloc)(void*))
{
	if (stack->size == 0) {
		stack->first = stack->last = malloc(sizeof(Memnode));

	}
	else {
		stack->last->next = malloc(sizeof(Memnode));
		stack->last = stack->last->next;
	}

	stack->last->ptr = ptr;
	stack->last->dealloc = dealloc;
	stack->size++;

	return ptr;
}

void mem_pop(Memstack* stack) 
{
	Memnode* current = stack->first;
	Memnode* next;
	int size = stack->size;

	for (int i = 0; i < size; i++) {
		next = current->next;
		current->dealloc(current->ptr);
		
		free(current);
		
		current = next;
	}
}

void free2(void* ptr) 
{
	printf("freeing\n");
	free(ptr);
}