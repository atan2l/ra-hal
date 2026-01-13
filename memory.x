MEMORY
{
  FLASH : ORIGIN = 0x0000000, LENGTH = 256K
  RAM : ORIGIN = 0x20000000, LENGTH = 32K
}

/*
Could look a little like this.

MEMORY
{

  // 0x0000_0400-0x0000_0403 (OFS0 § 6.2.1)
  OFS0 : ORIGIN = 0x00000400, LENGTH = 4 

  // 0x0000_0404-0x0000_0407 (OFS1 § 6.2.2)
  OFS1 : ORIGIN = 0x00000404, LENGTH = 4 

  SECMPU_PC_S0 : ORIGIN = 0x408, LENGTH = 4
  SECMPU_PC_E0 : ORIGIN = 0x40C, LENGTH = 4
  SECMPU_PC_S1 : ORIGIN = 0x410, LENGTH = 4
  SECMPU_PC_E1 : ORIGIN = 0x414, LENGTH = 4
  SECMPU_S0    : ORIGIN = 0x418, LENGTH = 4
  SECMPU_E0    : ORIGIN = 0x41C, LENGTH = 4
  SECMPU_S1    : ORIGIN = 0x420, LENGTH = 4
  SECMPU_E1    : ORIGIN = 0x424, LENGTH = 4
  SECMPU_S2    : ORIGIN = 0x428, LENGTH = 4
  SECMPU_E2    : ORIGIN = 0x42C, LENGTH = 4
  SECMPU_S3    : ORIGIN = 0x430, LENGTH = 4
  SECMPU_E3    : ORIGIN = 0x434, LENGTH = 4
  SECMPU_AC    : ORIGIN = 0x438, LENGTH = 4

  SEC_MPU    : ORIGIN = 0x408, LENGTH = 4 * 13

  FLASH : ORIGIN = 0x00000500, LENGTH = 256K - 0x500

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
  
  .sec_mpu : 
  {
    . = ALIGN(4);
    KEEP(*(.sec_mpu));
  } > SEC_MPU
}

ASSERT(SIZEOF(.ofs0) == 4, "OFS0 must be 32-bits.");
ASSERT(SIZEOF(.ofs1) == 4, "OFS1 must be 32-bits.");
ASSERT(SIZEOF(.sec_mpu) == 4*13, "SEC_MPU must have 13 entries.");
*/
