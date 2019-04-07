vectortile-rs
=============

[![vectortile on crates.io](https://img.shields.io/crates/v/vectortile.svg)](https://crates.io/crates/vectortile)
[![vectortile on docs.rs](https://docs.rs/vectortile/badge.svg)](https://docs.rs/vectortile)

Rust library for building Mapbox Vector Tiles from PostGIS.
This library was adapted from the MVT implementation in Pirmin Kalberer's [t-rex server](https://github.com/pka/t-rex), with changes intended to improve usage ergonomics.

## Usage
Put this in your `Cargo.toml`:

```toml
[dependencies]
vectortile = "0.2.1"
```

And this in your crate root:

```rust
extern crate vectortile;
```

## Example
See [examples/streets.rs](examples/streets.rs) for a full example w/ PostGIS.

This example shows creating Features and Tiles programmatically:
```rust
use vectortile::{Tile, Layer, Feature, Value};
use vectortile::geom::{Geometry, Point};
use vectortile::grid::{Grid, Extent};
use postgis::ewkb;

// Build a new tile, the hard way
let bbox = Extent{minx: 958826.08, miny: 5987771.04, maxx: 978393.96, maxy: 6007338.92};
let mut tile = Tile::new(&bbox);
let mut layer = Layer::new("place");
tile.add_layer(layer);

// Add a new point feature "Ed's Mospresso Shack"
let geom: Geometry = ewkb::GeometryT::Point(Point::new(960000.0, 6002729.0, Some(3857)));
let mut feature = Feature::new(geom);
feature.add_property("place", Value::String(String::from("business")));
feature.add_property("name", Value::String(String::from("Ed's Mospresso Shack")));
layer.add_feature(feature);

// Encode the tile as protobuf and inspect it
let grid = Grid::wgs84();
let data = tile.encode(&grid);
println!("{:#?}", data);
```
