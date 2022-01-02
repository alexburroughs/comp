#ifndef MEMSTACK_H
#define MEMSTACK_H

typedef struct memnode
{
	void* ptr;
	void (*dealloc)(void*);
	struct memnode* next;
} Memnode;

typedef struct memstack 
{
	Memnode* first;
	Memnode* last;
	int size;
} Memstack;

void free2(void* ptr);
void mem_pop(Memstack* stack);
void* mem_push(Memstack* stack, void* ptr, void (*dealloc)(void*));
Memstack* new_memstack();

#endif