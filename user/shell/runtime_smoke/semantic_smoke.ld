ENTRY(_start)

PHDRS
{
  text PT_LOAD FLAGS(5);
  rodata PT_LOAD FLAGS(4);
  data PT_LOAD FLAGS(6);
}

SECTIONS
{
  . = 0x10000;
  .text : ALIGN(0x1000)
  {
    KEEP(*(.text._start))
    *(.text .text.*)
  } :text

  . = ALIGN(0x1000);
  .rodata : ALIGN(0x1000)
  {
    *(.rodata .rodata.*)
  } :rodata

  . = ALIGN(0x1000);
  .data : ALIGN(0x1000)
  {
    *(.data .data.*)
  } :data

  .bss : ALIGN(0x1000)
  {
    *(.bss .bss.*)
    *(COMMON)
  } :data

  /DISCARD/ :
  {
    *(.comment .comment.*)
    *(.eh_frame .eh_frame.*)
    *(.note .note.*)
  }
}
