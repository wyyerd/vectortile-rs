extern crate postgres;
extern crate vectortile;

use postgres::{Connection, TlsMode};
use vectortile::{Encode, Tile, Layer, Feature, Value};
use vectortile::grid::Grid;
use std::fs::File;

fn main() {
    // WebMercator coordinates for downtown Denver, CO
    let zoom = 12;
    let x = 853;
    let y = 1554;

    // Open a database connection
    let conn = Connection::connect("postgres://postgres@localhost", TlsMode::None).unwrap();

    // Define a projection and tile
    let grid = Grid::web_mercator();
    let bbox = grid.tile_extent(zoom, x, y);
    let mut tile = Tile::new(&bbox);

    // Add features to the tile
    let mut layer = Layer::new("streets");
    let rows =
        conn.query("SELECT ST_Transform(geom::geometry,3857) AS geom, id, name FROM streets \
                    WHERE geom && ST_Transform(ST_MakeEnvelope($1, $2, $3, $4, $5), 4326)",
                   &[&bbox.minx, &bbox.miny, &bbox.maxx, &bbox.maxy, &grid.srid])
            .unwrap();
    for row in rows.iter() {
        let mut feature = Feature::new(row.get(0));
        feature.set_id(row.get::<usize, i32>(1) as u64);
        layer.add_feature(feature);
    }
    tile.add_layer(layer);

    // Write the
    let encoded = tile.encode(&grid);
    let mut file = File::create("streets.mvt").unwrap();
    encoded.to_writer(&mut file);

    // Inspect the tile
    println!("{:?}", encoded);
}
