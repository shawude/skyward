#!/usr/bin/env bash
# Collects what devices/dodge.toml asks about from a connected OnePlus 13.
# Run it on a computer with adb, while the phone runs its stock system.
#
#   bash scripts/probe-dodge.sh dodge-probe
#
# The folder it leaves behind is the source of truth for the TODO(dodge)
# lines: the recovery can only do what the phone's own firmware says it does.
set -eu

out="${1:-dodge-probe}"
mkdir -p "$out"

adb wait-for-device

dump() {
    local name="$1"
    shift
    printf '==> %s\n' "$name"
    adb shell "$@" > "$out/$name.txt" 2>&1 || true
}

dump props getprop
dump wm-size wm size
dump wm-density wm density
dump proc-modules cat /proc/modules
dump vendor-modules ls -l /vendor/lib/modules
dump vendor-hw ls -l /vendor/bin/hw
dump by-name ls -l /dev/block/by-name
dump sysfs ls -l /sys/class/touch /sys/class/qcom-haptics /sys/class/leds
dump input dumpsys input
dump data-vendor ls -l /data/vendor
dump oplusrom getprop ro.build.version.oplusrom

# The lines a port starts from, ready to paste into devices/dodge.toml.
{
    echo "# from $out"
    grep -E 'ro.build.version.oplusrom|ro.product.device|ro.sf.lcd_density' "$out/props.txt" || true
    cat "$out/wm-size.txt" "$out/wm-density.txt"
    echo
    echo "# the touch screen's module"
    grep -iE 'touch|synaptics|goodix|gt9' "$out/proc-modules.txt" || true
    echo
    echo "# the programs a recovery would have to run"
    grep -iE 'keymint|gatekeeper|weaver|secure_element|boot-service' "$out/vendor-hw.txt" || true
    echo
    echo "# the baseband and sensor partitions"
    grep -E 'modemst|fsg|fsc|persist' "$out/by-name.txt" || true
    echo
    echo "# the vibration motor and the touch driver's switches"
    cat "$out/sysfs.txt"
} > "$out/dodge-notes.txt"

printf '\nWritten to %s/ — start with dodge-notes.txt.\n' "$out"
