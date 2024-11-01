# spaday

GFWL/Xbox 360 SPAFILE parser

[![GitHub Workflow Status](https://img.shields.io/github/actions/workflow/status/widberg/spaday/build.yml)](https://github.com/widberg/spaday/actions)
![GitHub Release](https://img.shields.io/github/v/release/widberg/spaday)
[![Release Nightly](https://img.shields.io/badge/release-nightly-5e025f?labelColor=301934)](https://nightly.link/widberg/spaday/workflows/build/master)

With a complementary [ImHex](https://imhex.werwolv.net/) pattern, `spafile.hexpat`.

### Usage

The SPAFILE resource can be extracted from the executable using [Resource Hacker](https://www.angusj.com/resourcehacker/). It should be under the `RT_RCDATA` folder in the tree view on the left. Once located, right-click on the resource and select "Save *.bin resource...". Then spaday can be used to extract the contents of the SPAFILE.

```console
$ spaday path/to/spafile -o path/to/output_dir
```

### Issues

If you find a SPAFILE that fails to parse or yields unexpected results, please open an issue with the offending SPAFILE attached so I can add it to my test corpus. I don't have a large sample size of SPAFILES to test with, so I'm sure there are edge cases that I haven't encountered yet.

### Acknowledgements

* https://free60.org/System-Software/Formats/XDBF/
* https://free60.org/System-Software/Formats/SPA/
* https://free60.org/System-Software/Profile_Account/

### Prior Work

* Xenia
    - [src/xenia/kernel/util/xdbf_utils.cc](https://github.com/xenia-canary/xenia-canary/blob/canary_experimental/src/xenia/kernel/util/xdbf_utils.cc)
    - [src/xenia/kernel/util/xdbf_utils.h](https://github.com/xenia-canary/xenia-canary/blob/canary_experimental/src/xenia/kernel/util/xdbf_utils.h)
    - [src/xenia/kernel/util/xlast.cc](https://github.com/xenia-canary/xenia-canary/blob/canary_experimental/src/xenia/kernel/util/xlast.cc)
    - [src/xenia/kernel/util/xlast.h](https://github.com/xenia-canary/xenia-canary/blob/canary_experimental/src/xenia/kernel/util/xlast.h)

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
* The XPBM and XVC2 structures were implemented based on the Xenia code, as the documentation treated them as unstructured byte arrays.
* The documentation has XMAT as an unstructured byte array, but it was an XPBM in my limited testing.
* The documentation says that there will occasionally be unaccounted for bytes following a string in the string tables, but in my testing I never encountered this.
* The documentation says that endianness is based on the magic, but in my limited testing and the Xenia code it was always big-endian.
