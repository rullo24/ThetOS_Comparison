#!/usr/bin/env bash
# For every case: the fail_<case> fixture must NOT compile (and show the expected error code);
# the ok_<case> control must compile. Checked for the real target, thumbv7m-none-eabi. No board needed.
set -u
cd "$(dirname "$0")/.."
mkdir -p results/thetos

expected_code() {
    case "$1" in
        wrong_mode)        echo E0599 ;;
        stale_handle)      echo E0382 ;;
        uart_before_init)  echo E0599 ;;
        pwm_before_active) echo E0599 ;;
        shared_state)      echo E0277 ;;
        *)                 echo UNKNOWN ;;
    esac
}

status=0
while read -r c; do
    [ -z "$c" ] && continue
    ( cd thetos && cargo check --target thumbv7m-none-eabi --features "fail_$c" ) > "results/thetos/$c.fail.log" 2>&1
    fail_exit=$?
    ( cd thetos && cargo check --target thumbv7m-none-eabi --features "ok_$c" ) > "results/thetos/$c.ok.log" 2>&1
    ok_exit=$?
    code="$(expected_code "$c")"
    if [ "$fail_exit" -ne 0 ] && grep -q "error\[$code\]" "results/thetos/$c.fail.log" && [ "$ok_exit" -eq 0 ]; then
        verdict="PASS"
    else
        verdict="UNEXPECTED"; status=1
    fi
    echo "$fail_exit" > "results/thetos/$c.fail.exit"
    echo "$ok_exit" > "results/thetos/$c.ok.exit"
    echo "  $c: fail_exit=$fail_exit ok_exit=$ok_exit expected=$code -> $verdict"
done < cases.txt
echo "$status" > results/thetos.status
exit 0
