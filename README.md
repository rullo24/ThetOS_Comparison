# ThetOS_Comparison

Shows that eight common firmware mistakes compile in C (FreeRTOS, and ST's HAL for drivers) but not in ThetOS (Rust), or cannot be written in it. No hardware needed.

## Run it yourself

**1. Install the tools** (macOS shown; any OS with these works)

```
brew install rustup arm-none-eabi-gcc python3 git
rustup-init -y
```

`rustup` installs the pinned Rust toolchain and the `thumbv7m-none-eabi` target automatically from `thetos/rust-toolchain.toml`.

**2. Clone and run**

```
git clone git@github.com:rullo24/ThetOS_Comparison.git
cd ThetOS_Comparison
./run_all.sh
```

`run_all.sh` does everything and needs network access on the first run. Output: `results/side_by_side.md`.

**What gets pulled in** (pinned, no submodules needed)

| Source | Pinned to | How |
|---|---|---|
| ThetOS | tag `v1.1.0` | Cargo git dependency in `thetos/Cargo.toml`, fetched by `cargo` |
| FreeRTOS-Kernel | tag `V11.3.1` | `setup.sh` (shallow clone into `freertos/FreeRTOS-Kernel/`) |
| STM32L1 HAL, CMSIS device, CMSIS core | exact commits in `setup.sh` | `setup.sh` (into `freertos/third_party/`) |

To run the steps separately:

```
./setup.sh                  # download the pinned C sources
./tools/run_thetos.sh       # ThetOS side: cargo check --target thumbv7m-none-eabi --features fail_<case> / ok_<case>
./run_all.sh                # both sides plus the report
```

To check a single ThetOS case by hand:

```
cd thetos
cargo check --target thumbv7m-none-eabi --features fail_task_returns   # must fail with E0308
cargo check --target thumbv7m-none-eabi --features ok_task_returns     # must succeed
```

To compile a single C case by hand, see the `arm-none-eabi-gcc` line in `run_all.sh`.

## What was tested

Each mistake is written for both stacks and compiled for Cortex-M3 (STM32L152).

**Kernel level** (FreeRTOS kernel API vs ThetOS kernel API)

| Mistake | FreeRTOS V11.3.1 (C) | ThetOS v1.1.0 (Rust) |
|---|---|---|
| Task function returns | compiles | `E0308` (entry point must be `-> !`) |
| Priority and stack size swapped | compiles | `E0308` (distinct types) |
| Critical section not exited on every path | compiles | not expressible (closure-only API) |
| Two tasks share a plain variable | compiles | `E0277` (Rust `Sync` rule) |

**Driver level** (ST's STM32L1 HAL vs ThetOS drivers)

| Mistake | STM32 HAL (C) | ThetOS (Rust) |
|---|---|---|
| Write to a pin set as input | compiles | `E0599` (typestate) |
| Reuse a handle after it changed state | compiles | `E0382` (ownership) |
| Use a UART before init | compiles | `E0599` (typestate) |
| Use a PWM channel before the timer is active | compiles | `E0599` (typestate) |

Each ThetOS failure has a correct version that compiles.

## Differences and limits

- Kernel rows come from ThetOS's types (`-> !`, `TaskPriority`) and API shape; the shared-variable row is a Rust language rule, not a ThetOS feature.
- Driver rows come from typestate drivers. FreeRTOS has no peripheral layer, so the C side there is ST's HAL.
- Not different: stack overflow and priority-range/stack-size checks are runtime checks in both. Two handles to one physical pin still compile in ThetOS (ADR-0008).
- Compile-time behaviour only: nothing is run or timed.

## Files

- `thetos/src/lib.rs`: Rust cases, one `fail_*` and one `ok_*` per mistake (Cargo features).
- `freertos/cases/`: the C versions.
- `cases.txt`, `setup.sh`, `tools/`: case list, pinned downloads, runners.
