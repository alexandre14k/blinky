#!/usr/bin/env bash
set -u

# build.sh
# alexandre raduly <alexander14k28@gmail.com>
# update 03 oct 2026
# BSD-3-Clause

FLASH_BASE=0x08000000

device_load() {
    DEVICE=$(sed -n 's/^default *= *\[ *"\([a-z0-9]*\)".*/\1/p' \
        Cargo.toml)
    NAME=$(sed -n 's/^name *= *"\([^"]*\)".*/\1/p' Cargo.toml)
    VERSION=$(sed -n 's/^version *= *"\([^"]*\)".*/\1/p' \
        Cargo.toml)
    TARGET=$(target_get "$DEVICE")
    OCD_CFG=$(cfg_get "$DEVICE")
    FIRMWARE="$DEVICE-$NAME-$VERSION.bin"
}

target_get() {
    echo thumbv7em-none-eabihf
}

cfg_get() {
    echo target/stm32f4x.cfg
}

mass_erase_get() {
    echo "stm32f4x mass_erase 0"
}

menu_show() {
    echo "b -- build release"
    echo "c -- clean release"
    echo "r -- reset target"
    echo "e -- erase target"
    echo "f -- flash target"
    echo "k -- clear screen"
    echo "m -- show menu"
    echo "x -- exit"
}

build() {
    cargo -q build --release --target "$TARGET" \
        --no-default-features --features "$DEVICE" || return 1
    cargo size --release --target "$TARGET" -- -A
}

clean() {
    if [[ ! -d target ]]; then
        return 0
    fi

    cargo clean --release

    if find target -mindepth 1 -maxdepth 1 \
        ! -name '.rustc_info.json' -print -quit |
        grep -q .; then

        tree -a target -L 2
        read -rp 'purge "target/*" ? [y/N] ' answer
        [[ "$answer" == [yY] ]] &&
            find target -mindepth 1 ! -name '.rustc_info.json' -delete
    fi
}

reset_target() {
    openocd -f interface/stlink.cfg -f "$OCD_CFG" \
        -c "init; reset; exit"
}

erase_target() {
    local me
    me=$(mass_erase_get "$DEVICE")
    openocd -f interface/stlink.cfg -f "$OCD_CFG" \
        -c "init; reset halt; $me; exit"
}

flash_target() {
    cargo -q objcopy --release --target "$TARGET" -- \
        -O binary "target/$FIRMWARE" || return 1
    openocd -f interface/stlink.cfg -f "$OCD_CFG" \
        -c "program target/$FIRMWARE verify reset exit $FLASH_BASE"
}

main() {
    device_load
    menu_show
    local cmd
    while true; do
        printf "> "
        read -r cmd || break
        case "$cmd" in
            b) build ;;
            c) clean ;;
            r) reset_target ;;
            e) erase_target ;;
            f) flash_target ;;
            k) clear ;;
            m) menu_show ;;
            x) break ;;
            *) : ;;
        esac
    done
}

if [ -t 0 ]; then
    main "$@"
else
    title="$(basename $(pwd))"
    xfce4-terminal\
        --title="$title"\
        -e "bash -c '$0 $@; exec bash'"
fi