See <https://todo.sr.ht/~az1/ra4-rs/4/>


|             | RA0E1           | RA0L1           | RA2A1           | RA2L1           | RA2L2           | RA4M1           | RA4M2           | RA4L1           | RA6M5           | RA8M1           |
| --          | --              | --              | --              | --              | --              | --              | --              | --              | --              | --              |
| <td colspan=10 class="subhead">**Core**</td>                                                                                                                                                    |
|             | CM23            | CM23            | CM23            | CM23            | CM23            | CM4             | CM33            | CM33            | CM33            | CM85            |
|             | 32 MHz          | 32 MHz          | 48 MHz          | 48 MHz          | 48 MHz          | 48 MHz          | 100 MHz         | 80 MHz          | 200 MHz         | 480 MHz         |
| <td colspan=10 class="subhead">**Option Setting Memory**</td>                                                                                                                                   | 
| OFS0        | `0x0000_0400`   | `0x0000_0400`   | `0x0000_0400`   | `0x0000_0400`   | `0x0000_0400`   | `0x0000_0400`   | `0x0100_A100`   | `0x0100_A100`   | `0x0100_A100`   | `0x0300_A100`   |
| OFS1        | `0x0000_0404`   | `0x0000_0404`   | `0x0000_0404`   | `0x0000_0404`   | `0x0000_0404`   | `0x0000_0404`   | `0x0100_A180`   | `0x0100_A180`   | `0x0100_A180`   | `0x1300_A180`   |
| OFS2        |                 |                 |                 |                 |                 |                 |                 |                 |                 | `0x0300_A104`   |
| SEC_MPU     |                 |                 | `0x0000_0408`   | `0x0000_0408`   | `0x0000_0408`   | `0x0000_0408`   |                 |                 |                 |                 |
| DUALSEL     |                 |                 |                 |                 |                 |                 |                 | `0x0100_A110`   | `0x0100_A110`   | `0x0300_A110`   |
| SAS         |                 |                 |                 |                 |                 |                 | `0x0100_A134`   | `0x0100_A134`   | `0x0100_A134`   | `0x0300_A134`   |
| BANKSEL     |                 |                 |                 |                 |                 |                 |                 | `0x0100_A190`   | `0x0100_A190`   | `0x1300_A190`   |
| BPS         |                 |                 |                 |                 |                 |                 | `0x0100_A1C0`   | `0x0100_A1C0`   | `0x0100_A1C0`   | `0x1300_A1C0`   |
|<td colspan=10 class="subhead">**Interrupt Controller Unit**</td>                                                                                                                                |
| ## IRQ      | 39?             | 48              | 32              | 32              | 32              | 32              | 96              | 64              | 96              | 96              |
| DTC         | Y               | Y               | Y               | Y               | Y               | Y               | Y               | Y               | Y               | Y               |
| DMAC        |                 |                 |                 |                 |                 | 4               | 8               | 8               | 8               | 8               |
| Ext IRQ     | 6               | 8               | 8               | 8               | 8               | 16              | 16              | 16              | 16              | 16              |
| <td colspan=10 class="subhead">**Timers**</td>                                                                                                                                                  |
| AGT         |                 |                 | 2               | 2               |                 | 2               | 6               |                 | 6               | 2               |
| AGTW        |                 |                 |                 |                 | 2               |                 |                 | 2               |                 |                 |
| GPT 32 / 16 |                 |                 | 1 / 6           | 4 / 6           | 1 / 6           | 2 / 6           | 4 / 4           | 2 / 4           | 4 / 6           | 8 / 6           |
| ULPT        |                 |                 |                 |                 |                 |                 |                 |                 |                 | 2               |
| TAU         | 8               | 8               |                 |                 |                 |                 |                 |                 |                 |                 |
| TML32       | 1               | 1               |                 |                 |                 |                 |                 |                 |                 |                 |
| <td colspan=10 class="subhead">**Numeric**</td>                                                                                                                                                 |
| CRC         | Y               | Y               | Y, snoop        | Y, snoop        | Y, snoop        | Y, snoop        | Y               | Y               | Y               | Y, snoop+32     |
| <td colspan=10 class="subhead">**Serial**</td>                                                                                                                                                  |
| SAU         | 6               | 6               |                 |                 |                 |                 |                 |                 |                 |                 |
| SCI         |                 |                 | 3               | 5               | 4               | 4               | 6               | 6               | 10              | 6               |
| UARTA       | 1               | 1               |                 |                 |                 |                 |                 |                 |                 |                 |
| SPI         |                 |                 | 2               | 2               | 1               | 2               | 1               | 1               | 2               |                 |
| SPI_B       |                 |                 |                 |                 |                 |                 |                 |                 |                 | 2               |
| QSPI        |                 |                 |                 |                 |                 |                 | 1               | 1               | 1               |                 |
| OSPI        |                 |                 |                 |                 |                 |                 |                 |                 | 1               |                 |
| OSPI_B      |                 |                 |                 |                 |                 |                 |                 |                 |                 | 1               |
| IIC         |                 |                 | 2, FM           | 2, FM           |                 | 2, FM           | 2, FM+          | 1, FM+          | 3, FM+          | 2, FM+          | 
| IIC_A       | 1, FM+          | 2, FM+          |                 |                 |                 |                 |                 |                 |                 |                 | 
| I3C         |                 |                 |                 |                 | 1, HSM          |                 |                 | 1, HSM          |                 | 1, HSM          |
| <td colspan=10 class="subhead">**CAN**</td>                                                                                                                                                     |
| CAN         |                 |                 | 1x 1 Mbps       | 1x 1 Mbps       | 1x 1 Mbps       | 1x 1 Mbps       | 1x 1 Mbps       |                 |                 |                 |
| CANFD       |                 |                 |                 |                 |                 |                 |                 | 1x 8 Mbps       | 2x 8 Mbps       | 2x 8 Mbps       |
| <td colspan=10 class="subhead">**USB**</td>                                                                                                                                                     |
| USBFS       |                 |                 | Y               |                 | Y               | Y               | Y               | Y               | Y               | Y               |
| USBHS       |                 |                 |                 |                 |                 |                 |                 |                 | Y               | Y               |
| <td colspan=10 class="subhead">**I/O**</td>                                                                                                                                                     |
| ADC12       |                 |                 |                 | 19x 12-bit      | 17x 12-bit      |                 | 13x 8/10/12-bit | 16x 12-bit      | 26x 8/10/12-bit | 25x 8/10/12-bit |
| ADC12_D     | 10x 8/10/12-bit | 13x 8/10/12-bit |                 |                 |                 |                 |                 |                 |                 |                 |
| ADC14       |                 |                 |                 |                 |                 | 25x 12/14-bit   |                 |                 |                 |                 |
| ADC16       |                 |                 | 17x 16bit       |                 |                 |                 |                 |                 |                 |                 |
| SDADC24     |                 |                 | 10x 24bit       |                 |                 |                 |                 |                 |                 |                 |
| DAC12       |                 |                 | 1               | 1               |                 | 1               | 2               | 1               | 2               | 2               |
