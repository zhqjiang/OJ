# [Vase Collection](https://open.kattis.com/problems/vase)

Northwestern Europe Programming Contest (NWERC) 2003

1. For a given k, use `clean` of `struct M` to cut useless points, then use brute force method to check whether the solution of (k + 1) exists.

2. If one solution is found, back to 1, check whether (k + 2) is possbile.

3. If not, k is the final result.

Runtime: 0.07 s

