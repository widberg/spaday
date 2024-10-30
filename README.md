# spaday

GFWL/Xbox 360 SPAFILE parser

### Acknowledgements

* https://free60.org/System-Software/Formats/XDBF/
* https://free60.org/System-Software/Formats/SPA/
* https://free60.org/System-Software/Profile_Account/

### Notes

#### Binwalk Analysis

Sample output from [binwalk](https://github.com/ReFirmLabs/binwalk) for a SPAFILE:

```console
HEXADECIMAL                        DESCRIPTION
--------------------------------------------------------------------------------------------------------
0xBEC5                             PNG image, total size: 10885 bytes
0xE94A                             PNG image, total size: 8039 bytes
0x108B1                            PNG image, total size: 9620 bytes
0x12E45                            PNG image, total size: 9067 bytes
0x151B0                            PNG image, total size: 8358 bytes
0x17256                            PNG image, total size: 9409 bytes
0x19717                            PNG image, total size: 10096 bytes
0x1BE87                            PNG image, total size: 11186 bytes
0x1EA39                            PNG image, total size: 8047 bytes
0x209A8                            PNG image, total size: 9322 bytes
0x22E12                            PNG image, total size: 9403 bytes
0x252CD                            PNG image, total size: 9976 bytes
0x279C5                            PNG image, total size: 9160 bytes
0x29D8D                            PNG image, total size: 10681 bytes
0x2C746                            PNG image, total size: 8862 bytes
0x2E9E4                            PNG image, total size: 10248 bytes
0x311EC                            PNG image, total size: 8830 bytes
0x3346A                            PNG image, total size: 8862 bytes
0x35708                            PNG image, total size: 9574 bytes
0x37E35                            gzip compressed data, operating system: FAT filesystem (MS-DOS, OS/2,
                                    NT/Win32, timestamp: 1970-01-01 00:00:00, total size: 32444 bytes
```

The PNG images are the game and achievement icons in namespace `2`, and the gzip compressed data is the ".xlast" XML file in namespace `1`, id `XSRC`. When extracting a SPAFILE with 7-Zip it will only find and extract the XML file. In the few cases I checked these XML files were UTF-16 LE encoded.

#### Deviations from Free60 Documentation

* The "Record Count" field in the XPRP structure needed to be changed from a `u32` to a `u16` to avoid overrunning the end of the file.
* The XRPT and XVC2 structures as laid out in the documentation would not consistently parse, so they were changed to consume the entire remaining data in the entry as a byte array.
* The documentation says that there will occasionally be unaccounted for bytes following a string in the string tables, but in my testing I never encountered this.
