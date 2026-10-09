#!/usr/bin/env bash
# Fetches the pinned third-party sources used by the FreeRTOS side of the comparison:
#   FreeRTOS-Kernel, and the STM32L1 HAL + CMSIS headers (the vendor stack used with FreeRTOS on this MCU).
set -eu
cd "$(dirname "$0")"

FREERTOS_TAG="V11.3.1"
HAL_URL="https://github.com/STMicroelectronics/stm32l1xx_hal_driver.git"
HAL_REF="940073207fa21eff5b808e4f98b638a1486c09d7"
DEV_URL="https://github.com/STMicroelectronics/cmsis_device_l1.git"
DEV_REF="a23ade4ccf14012085fedf862e33a536ab7ed8be"
CORE_URL="https://github.com/STMicroelectronics/cmsis_core.git"
CORE_REF="01d18147527e30ed69ace1cb03b818de78bc7062"

fetch_pinned() {
    local dir="$1" url="$2" ref="$3"
    if [ -d "$dir/.git" ]; then
        echo "present: $dir ($(git -C "$dir" rev-parse --short=10 HEAD))"
        return
    fi
    mkdir -p "$dir"
    git -C "$dir" init -q
    git -C "$dir" remote add origin "$url"
    git -C "$dir" fetch -q --depth 1 origin "$ref"
    git -C "$dir" checkout -q FETCH_HEAD
    echo "fetched: $dir ($(git -C "$dir" rev-parse --short=10 HEAD))"
}

if [ -d freertos/FreeRTOS-Kernel/.git ]; then
    echo "present: freertos/FreeRTOS-Kernel ($(git -C freertos/FreeRTOS-Kernel describe --tags --always))"
else
    git clone -q --depth 1 --branch "$FREERTOS_TAG" https://github.com/FreeRTOS/FreeRTOS-Kernel.git freertos/FreeRTOS-Kernel
    echo "fetched: freertos/FreeRTOS-Kernel $FREERTOS_TAG"
fi

fetch_pinned freertos/third_party/stm32l1xx_hal_driver "$HAL_URL" "$HAL_REF"
fetch_pinned freertos/third_party/cmsis_device_l1 "$DEV_URL" "$DEV_REF"
fetch_pinned freertos/third_party/cmsis_core "$CORE_URL" "$CORE_REF"
