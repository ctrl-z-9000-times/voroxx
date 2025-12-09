/*! Voro++, a 3D cell-based Voronoi library

Voro++ is a software library for carrying out three-dimensional computations of
the Voronoi tessellation. A distinguishing feature of the Voro++ library is that
it carries out cell-based calculations, computing the Voronoi cell for each
particle individually, rather than computing the Voronoi tessellation as a
global network of vertices and edges. It is particularly well-suited for
applications that rely on cell-based statistics, where features of Voronoi cells
(eg. volume, centroid, number of faces) can be used to analyze a system of
particles.

For a general overview of the program, see the Voro++ website at
<http://math.lbl.gov/voro++/> and in particular the example programs at
<http://math.lbl.gov/voro++/examples/> that demonstrate many of the library's
features.

Voro++ is written in C++ and this rust crate provides API bindings to a limited
subset of the voro++ library. */

// Rust API bindings written by David McDougall, 2020. Email Address: dam1784@rit.edu

mod rust;
pub use rust::voronoi_cell::VoronoiCell;
pub use rust::voronoi_cell_base::VoronoiCellBase;
pub use rust::voronoi_cell_neighbor::VoronoiCellNeighbor;
