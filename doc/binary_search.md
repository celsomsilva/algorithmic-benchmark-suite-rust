# Binary Search

## Problem

Given a sorted array, determine whether a target value exists and return
its position (index). If the value is not present, a sentinel value is returned.

## Invariant

At each iteration, if the target exists, it must be inside the inclusive
interval:

[start, end]

This interval shrinks monotonically until the value is found or the
interval becomes empty.

## Implementation Notes

- Indices are controlled explicitly.
- The midpoint is computed using a bit shift instead of division by 2.
- No helper methods or abstractions are used.
- The implementation mirrors classical Pascal and C versions.

## Edge Cases Covered

- Empty array
- Single-element array
- Target not present
- Target at boundaries

## Benchmark Observations

Binary search exhibits logarithmic time complexity.

The benchmark is intended to show general performance trends rather than
micro-optimizations.

