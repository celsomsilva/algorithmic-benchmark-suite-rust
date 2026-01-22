# Bubble Sort

## Problem

Sort an array of integers in ascending order.

## Idea

Repeatedly traverse the array, swapping adjacent elements when they
are out of order. After each pass, the largest element is guaranteed
to be placed at the end of the array.

## Invariant

After the i-th outer iteration, the last i elements are already sorted
and will not be moved again.

## Implementation Notes

- Two explicit nested loops are used.
- A temporary variable is used for swapping.
- No early-exit optimization is applied.
- No helper methods or abstractions are introduced.

The implementation intentionally follows a classical, educational
version of the algorithm.

## Edge Cases Covered

- Empty array
- Single-element array
- Already sorted array
- Reverse-sorted array
- Arrays with duplicate values

## Benchmark Observations

Bubble sort exhibits quadratic time complexity.

Even for moderate input sizes, the cost becomes apparent, making it
useful as a contrast against more efficient algorithms.

