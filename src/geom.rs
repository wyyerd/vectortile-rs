// Copyright (c) Pirmin Kalberer. All rights reserved.
// Licensed under the MIT License. See LICENSE file in the project root for full license information.
//

use postgis::ewkb;

pub type Point = ewkb::Point;
pub type LineString = ewkb::LineString;
pub type Polygon = ewkb::Polygon;
pub type MultiPoint = ewkb::MultiPoint;
pub type MultiLineString = ewkb::MultiLineString;
pub type MultiPolygon = ewkb::MultiPolygon;
pub type GeometryCollection = ewkb::GeometryCollection;
pub type Geometry = ewkb::Geometry;
