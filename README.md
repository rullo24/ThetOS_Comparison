# ThetOS_Comparison

Shows that five common firmware mistakes compile in C (FreeRTOS + STM32 HAL) but not in ThetOS (Rust). No hardware needed.

## Run

```
./run_all.sh
```

Needs `rustup`, `arm-none-eabi-gcc`, `git`, `python3` and network access on the first run. Output: `results/side_by_side.md`.

## What was tested

Each mistake is written for both stacks and compiled for Cortex-M3 (STM32L152).

| Mistake | FreeRTOS V11.3.1 + STM32L1 HAL (C) | ThetOS v1.1.0 (Rust) |
|---|---|---|
| Write to a pin set as input | compiles | `E0599` (typestate) |
| Reuse a handle after it changed state | compiles | `E0382` (ownership) |
| Use a UART before init | compiles | `E0599` (typestate) |
| Use a PWM channel before the timer is active | compiles | `E0599` (typestate) |
| Two tasks share a plain variable | compiles | `E0277` (Rust `Sync` rule) |

Each ThetOS failure has a correct version that compiles.

## Differences and limits

- ThetOS rejects these mistakes because its drivers encode state in types. The last row is a Rust language rule, not a ThetOS feature.
- FreeRTOS has no peripheral layer, so the C side uses ST's HAL. This compares firmware stacks, not kernels alone.
- Not different: stack overflow and priority/stack-size checks are runtime checks in both. Two handles to one physical pin still compile in ThetOS (ADR-0008).
- Compile-time behaviour only: nothing is run or timed.

## Files

- `thetos/src/lib.rs`: Rust cases, one `fail_*` and one `ok_*` per mistake (Cargo features).
- `freertos/cases/`: the C versions.
- `cases.txt`, `setup.sh`, `tools/`: case list, pinned downloads, runners.
