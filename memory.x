/* Specify main memory areas */
MEMORY
{
  /* Use values from the ESP-IDF 'bootloader' component.
  /* TODO: Use human-readable lengths */
  /* vectors ( RX )       : ORIGIN = 0x40080000, len = 0x400 */
  iram_seg ( RX )       : ORIGIN = 0x40080400, len = 0x10000
  dram_seg ( RW )       : ORIGIN = 0x3FFB0000, len = 0x40000
}