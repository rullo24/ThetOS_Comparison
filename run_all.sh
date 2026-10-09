#!/usr/bin/env bash
# Runs both sides and writes results/side_by_side.md.
#   ThetOS side  : fixtures against the ThetOS board crate, checked for thumbv7m (no hardware)
#   FreeRTOS side: C cases compiled against the real FreeRTOS-Kernel + STM32L1 HAL headers (no hardware)
set -u
cd "$(dirname "$0")"

# locate tools if they are not already on PATH
if ! command -v cargo >/dev/null 2>&1; then
    TC="$HOME/.rustup/toolchains/1.98.0-aarch64-apple-darwin/bin"
    [ -d "$TC" ] && export PATH="$TC:$PATH"
fi
if ! command -v arm-none-eabi-gcc >/dev/null 2>&1 && [ -d /opt/arm-toolchain/bin ]; then
    export PATH="/opt/arm-toolchain/bin:$PATH"
fi

mkdir -p results/freertos results/thetos
./setup.sh || { echo "setup failed"; exit 1; }

echo "== ThetOS side (cargo check, thumbv7m-none-eabi) =="
./tools/run_thetos.sh

echo "== FreeRTOS side (arm-none-eabi-gcc) =="
K="freertos/FreeRTOS-Kernel"
T="freertos/third_party"
while read -r c; do
    [ -z "$c" ] && continue
    arm-none-eabi-gcc -c -Wall -Wextra -std=c11 -mcpu=cortex-m3 -mthumb \
        -DSTM32L152xE -DUSE_HAL_DRIVER \
        -Ifreertos -Ifreertos/hal_conf \
        -I"$K/include" -I"$K/portable/GCC/ARM_CM3" \
        -I"$T/stm32l1xx_hal_driver/Inc" -I"$T/cmsis_device_l1/Include" -I"$T/cmsis_core/CMSIS/Core/Include" \
        "freertos/cases/$c.c" -o /dev/null 2> "results/freertos/$c.log"
    echo "$?" > "results/freertos/$c.exit"
    echo "  $c: exit $(cat results/freertos/$c.exit)"
done < cases.txt

python3 tools/make_report.py
echo "Wrote results/side_by_side.md"
