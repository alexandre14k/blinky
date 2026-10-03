SECTIONS
{
  .text : {
    KEEP(*(.init))
    KEEP(*(.after_reset.*))
    *(.text*)
    *(.rodata*)
    KEEP(*(.eh_frame*))
  } > FLASH

  .ARM.exidx : {
    KEEP(*(.ARM.exidx*))
    KEEP(*(.gnu.linkonce.armexidx.*))
  } > FLASH

  .data : {
    _data = .;
    *(.data*)
    . = ALIGN(4);
    _edata = .;
  } > RAM AT > FLASH

  _sidata = LOADADDR(.data);

  .bss : {
    _bss = .;
    *(.bss*)
    *(COMMON)
    . = ALIGN(4);
    _ebss = .;
  } > RAM

  .heap (COPY) : {
    __heap_start = .;
    . = . + 8K;
    __heap_end = .;
  } > RAM
}