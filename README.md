# Arbitrage exploration

## Benchmark

Baseline: 167066 cu
Diff: 43481 cu

CU consumption
- Pinocchio with couple optimisations
  - before first swap: 1095
  - total: 125633
- Without convenience logs
  - before first swap: 887
  - total: 125007
- Const initialization ix data (hardcode disciminant, etc)
  - before first swap: 887
  - total: 125007
  - Note: No improvement - The compiler should already do that
- Pass ta out index as argument
  - before first swap: 890
  - total: 124995
  - Note: Larger mem alloc but less computation for balance snapshot
- Skip initialization of account_meta and account_info_ptrs with MaybeUninit, and iterate only once over account_infos
  - before first swap: 832
  - total: 124894
- Use pointer arithmetic in instead of reference
  - before first swap: 765
  - total: 124766
- Use unchecked cpi
  - before first swap: 1119
  - total: 123664
  - Note: `Account::from(account_info)` is more cu intensive that building `[&AccountInfo]` with pointer arithmetic
- Manually unroll loop
  - before first swap: 1097
  - total: 123629
- Pointer arithmetic for snapshot
  - before first swap: 1094
  - total: 123620
- Pointer arithmetic for swap loop
  - before first swap: 1092
  - total: 123604
- Break early if no remaining swap
  - before first swap: 1092
  - total: 123585
