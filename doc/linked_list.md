# Linked List

## Problem

Represent a sequence of integer values using a singly linked structure,
where each element explicitly points to the next one.

## Approach

The list is built and manipulated using explicit node references.
Each node contains:
- an integer value (`data`)
- a reference to the next node (`next`)

Operations such as traversal, search, insertion, and elimination are
performed procedurally by moving pointers through the list.

No abstract data structure interface is defined.

## Invariant

At any point during execution:

- the head reference points to the first node of the list
- each node either points to another node or marks the end of the list
- the list is traversed by following successive `next` references

The correctness of the structure depends entirely on maintaining valid
pointer links.

## Implementation Notes

- Node references are manipulated directly.
- Temporary auxiliary pointers are used during traversal and search.
- The head pointer may be moved and later restored.
- No standard library collections or iterator abstractions are used.
- No encapsulation or helper methods are introduced.

The implementation follows a procedural style rather than an object-oriented one.

## Edge Cases Covered

- Traversal of the entire list
- Linear search for a target value
- Insertion after a successful search
- Correct termination at the end of the list
- Preservation of list structure after pointer manipulation

## Benchmark Observations

Traversal and search operations exhibit linear time complexity.

Performance depends directly on the number of nodes traversed and the
cost of pointer dereferencing.

