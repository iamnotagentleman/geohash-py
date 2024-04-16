# geohash-py

`geohash-py` is a Rust-powered library for geohashing, designed for high-performance spatial operations.

## Features

- Rust-powered performance
- Compatible with Python 3.8 and above
- Supports macOS and POSIX-compliant Linux systems
- Works with both CPython and PyPy implementations

## Installation

To install `geohash-py`, you will need a Python environment compliant with Python 3.8 or later. 


```bash
pip install geohash-py==0.1.2
```


## Quickstart

### Decoding With Offset
```
>>> lat, lon, lat_offset, lon_offset = geohash_py.decode_geohash_with_offset("ww8p1r4t8")
(112.55838632583618, 37.83238649368286, 2.1457672119140625e-05, 2.1457672119140625e-05)

```

### Decoding
```
>>> lat, lon = geohash_py.decode_geohash("ww8p1r4t8")
(112.55838632583618, 37.83238649368286)

```

### Encoding
```
>>> geohash_py.encode_geohash(112.55838632583618, 37.83238649368286, precision=9)
'ww8p1r4t8'
```

## Contributing

Contributions to geohash-py are welcome. Please ensure to follow the code style and include tests for new features. For major changes, please open an issue first to discuss what you would like to change.

