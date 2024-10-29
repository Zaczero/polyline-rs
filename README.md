# polyline-rs

[![PyPI - Python Version](https://shields.monicz.dev/pypi/pyversions/polyline-rs)](https://pypi.org/project/polyline-rs)
[![Liberapay Patrons](https://shields.monicz.dev/liberapay/patrons/Zaczero?logo=liberapay&label=Patrons)](https://liberapay.com/Zaczero/)
[![GitHub Sponsors](https://shields.monicz.dev/github/sponsors/Zaczero?logo=github&label=Sponsors&color=%23db61a2)](https://github.com/sponsors/Zaczero)

Fast Google Encoded Polyline encoding & decoding in Rust. A Python PyO3 wrapper for [polyline](https://crates.io/crates/polyline) with out-of-the-box support for both (lat, lon) and (lon, lat) coordinates.

## Installation

The recommended installation method is through the PyPI package manager. The project is implemented in Rust and several pre-built binary wheels are available for Linux, macOS, and Windows, with support for both x64 and ARM architectures.

```sh
pip install polyline-rs
```

## Basic usage

```py
from polyline_rs import encode_latlon, encode_lonlat, decode_latlon, decode_lonlat

line = encode_latlon([(38.5, -120.2), (40.7, -120.95), (43.252, -126.453)], 5)
assert line == "_p~iF~ps|U_ulLnnqC_mqNvxq`@"

coords = decode_latlon(line, 5)
assert coords == [(38.5, -120.2), (40.7, -120.95), (43.252, -126.453)]

coords2 = decode_lonlat(line, 5)
assert coords2 == [(-120.2, 38.5), (-120.95, 40.7), (-126.453, 43.252)]
```
