# OpenSCQ30 - P31i Support Fork

This is a fork of OpenSCQ30 with added support for the **Soundcore P31i**.

OpenSCQ30 is free software for controlling Soundcore headphones and earbuds. While it started with the Life Q30, it's grown to support a ton of devices. I've specifically added the P31i to this version.

### The P31i Situation
I've added support for the Soundcore P31i here. One weird thing I noticed is that while most Soundcore devices use an `AXXXX` model number (like A3959), the P31i shows up as `D1202`. I haven't quite figured out why it uses a different naming convention, so I might be off on the conventional naming, but the implementation is there and it's working.

This is still a work in progress. I'm currently figuring out exactly what's working and what isn't, but overall it's pretty well functioning and stable for daily use.

### Supported Platforms
- [x] Windows - Ready
- [x] Linux - Ready
- [x] Android - Ready

### Supported Devices
This fork includes everything from the main project plus the P31i:

| Model | Name                               |
| ----- | ---------------------------------- |
| **D1202** | **Soundcore P31i**             |
| A3004 | Soundcore Q20i                     |
| A3027 | Soundcore Life Q35                 |
| A3028 | Soundcore Life Q30                 |
| A3029 | Soundcore Life Tune                |
| A3030 | Soundcore Life Tune Pro            |
| A3031 | Soundcore Vortex                   |
| A3033 | Soundcore Life 2 Neo               |
| A3035 | Soundcore Space One                |
| A3040 | Soundcore Space Q45                |
| A3116 | Soundcore Motion+                  |
| A3926 | Soundcore Life Dot 2S              |
| A3930 | Soundcore Liberty 2 Pro            |
| A3931 | Soundcore Life Dot 2 NC            |
| A3933 | Soundcore Life Note 3              |
| A3935 | Soundcore Life A2 NC               |
| A3936 | Soundcore Space A40                |
| A3939 | Soundcore Life P3                  |
| A3945 | Soundcore Life Note 3S             |
| A3947 | Soundcore Liberty 4 NC             |
| A3948 | Soundcore A20i                     |
| A3949 | Soundcore P20i / P25i / R50i       |
| A3951 | Soundcore Liberty Air 2 Pro        |
| A3955 | Soundcore P40i                     |
| A3957 | Soundcore Liberty 5                |
| A3959 | Soundcore P30i / Soundcore R50i NC |

## Getting Started
If you're looking for the original project, check out [Oppzippy/OpenSCQ30](https://github.com/Oppzippy/OpenSCQ30). 

For this fork, you can build it yourself following the docs:
- Windows: [docs/build-windows.md](docs/build-windows.md)
- Linux: [docs/build-linux.md](docs/build-linux.md)
- Android: [docs/build-android.md](docs/build-android.md)

## Contributing
Everything here is open to interpretation.

## Original Project Links
- GitHub: https://github.com/Oppzippy/OpenSCQ30
- Codeberg: https://codeberg.org/Oppzippy/OpenSCQ30
