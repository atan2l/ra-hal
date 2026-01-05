/* Linker script for the RA4M1 */
/* Note: this *will* clobber the Arduino bootloader */
MEMORY
{
  /* NOTE 1 K = 1 KiBi = 1024 bytes */

  /* 0x0000_0000-0x0000_03FF (free § 4.1) */
  UNUSED : ORIGIN = 0x00000000, LENGTH = 1K

  /* 0x0000_0400-0x0000_0403 (OFS0 § 6.2.1) */
  OFS0 : ORIGIN = 0x00000400, LENGTH = 4

  /* 0x0000_0404-0x0000_0407 (OFS1 § 6.2.2) */
  OFS1 : ORIGIN = 0x00000404, LENGTH = 4

  /* 0x0000_0408-0x0000_043B (Security MPU SECMPU § 6.2.3) */

  /* 0x0000_043C-0x0004_0000 (free § 4.1) */
  FLASH : ORIGIN = 0x00000800, LENGTH = 254K

  /* 0x2000_0000-0x2000_8000 (RAM § 4.1) */
  RAM : ORIGIN = 0x20000000, LENGTH = 32K
}

SECTIONS {
    .ofs0 : 
    {
      . = ALIGN(4);
      KEEP(*(.ofs0));
    } > OFS0

    .ofs1 : 
    {
      . = ALIGN(4);
      KEEP(*(.ofs1));
    } > OFS1
} 

ASSERT(SIZEOF(.ofs0) == 4, "OFS0 must be 32-bits.");
ASSERT(SIZEOF(.ofs1) == 4, "OFS1 must be 32-bits.");
