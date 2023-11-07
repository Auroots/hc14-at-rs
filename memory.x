MEMORY
{
  /* NOTE K = KiBi = 1024 bytes 
    STM32F103C6T6：FLASH(ROM) = 32K, RAM = 10K, (Freq：72Mhz)
    STM32F103C8T6：FLASH(ROM) = 64K, RAM = 20K, (Freq：72Mhz)
    STM32F103CBT6：FLASH(ROM) = 128K, RAM = 20K, (Freq：72Mhz)
    STM32F411CEx：FLASH(ROM) = 512K, RAM = 128K, (Freq：100Mhz)
  */
  FLASH (rx) : ORIGIN = 0x08000000, LENGTH = 128K
  RAM (rwx) : ORIGIN = 0x20000000, LENGTH = 20K
}
 
/* This is where the call stack will be allocated. */
/* The stack is of the full descending type. */
/* NOTE Do NOT modify `_stack_start` unless you know what you are doing */
_stack_start = ORIGIN(RAM) + LENGTH(RAM);