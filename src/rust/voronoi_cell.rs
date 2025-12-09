use crate::rust::voronoi_cell_base::{VoronoiCellBase, VoronoiCellBaseFFI};
use cpp::cpp;

cpp! {{
    #include "voro++.hh"
    using namespace voro;
}}

/// A single Voronoi cell, without neighbor information.
///
/// This class represents a single Voronoi cell, as a collection of vertices
/// that are connected by edges. The class contains routines for initializing
/// the Voronoi cell to be simple shapes such as a box, tetrahedron, or
/// octahedron. It the contains routines for recomputing the cell based on
/// cutting it by a plane, which forms the key routine for the Voronoi cell
/// computation. It contains numerous routine for computing statistics about the
/// Voronoi cell, and it can output the cell in several formats.
///
/// Use this class in cases when is __not__ necessary to track the IDs of
/// neighboring particles associated with each face of the Voronoi cell.
#[repr(C)]
pub struct VoronoiCell(*mut std::ffi::c_void);

impl VoronoiCellBaseFFI for VoronoiCell {
    fn ptr(&self) -> *mut std::ffi::c_void {
        self.0
    }
}

impl VoronoiCellBase for VoronoiCell {}

impl VoronoiCell {
    /// Initializes the Voronoi cell to be an axis aligned rectangular box with
    /// the given dimensions.
    ///
    /// __Parameters:__
    /// * `xyz_min` The minimum coordinates.
    /// * `xyz_max` The maximum coordinates.
    pub fn init(xyz_min: &[f64; 3], xyz_max: &[f64; 3]) -> Self {
        debug_assert!(xyz_min[0] <= xyz_max[0]);
        debug_assert!(xyz_min[1] <= xyz_max[1]);
        debug_assert!(xyz_min[2] <= xyz_max[2]);
        Self(cpp!(unsafe
                [xyz_min as "double*", xyz_max as "double*"]
                -> *mut std::ffi::c_void as "voronoicell*" {
            voronoicell* x = new voronoicell;
            x->init(xyz_min[0], xyz_max[0], xyz_min[1], xyz_max[1], xyz_min[2], xyz_max[2]);
            return x;
        }))
    }

    /// Initializes the cell to be an octahedron with vertices at (l,0,0),
    /// (-l,0,0), (0,l,0), (0,-l,0), (0,0,l), and (0,0,-l).
    ///
    /// __Parameters:__
    /// * `l` a parameter setting the size of the octahedron.
    pub fn init_octahedron(l: f64) -> Self {
        Self(cpp!(unsafe
                [l as "double"]
                -> *mut std::ffi::c_void as "voronoicell*" {
            voronoicell* x = new voronoicell;
            x->init_octahedron(l);
            return x;
        }))
    }

    /// Initializes the cell to be a tetrahedron.
    ///
    /// __Parameters:__
    /// * `v1` The coordinates (x,y,z) of the first vertex.
    /// * `v2` The coordinates (x,y,z) of the second vertex.
    /// * `v3` The coordinates (x,y,z) of the third vertex.
    /// * `v4` The coordinates (x,y,z) of the fourth vertex.
    pub fn init_tetrahedron(v1: &[f64; 3], v2: &[f64; 3], v3: &[f64; 3], v4: &[f64; 3]) -> Self {
        Self(cpp!(unsafe
                [v1 as "double*", v2 as "double*", v3 as "double*", v4 as "double*"]
                -> *mut std::ffi::c_void as "voronoicell*" {
            voronoicell* x = new voronoicell;
            x->init_tetrahedron(
                v1[0], v1[1], v1[2],
                v2[0], v2[1], v2[2],
                v3[0], v3[1], v3[2],
                v4[0], v4[1], v4[2]);
            return x;
        }))
    }

    /// Cuts a Voronoi cell using by the plane corresponding to the
    /// perpendicular bisector between the particle and the origin.
    ///
    /// __Parameters:__
    /// (`x`, `y`, `z`) The position of the particle.
    ///
    /// __Returns:__
    ///     False if the plane cut deleted the cell entirely, true otherwise.
    pub fn plane(&mut self, xyz: &[f64; 3]) -> bool {
        let ptr = self.0;
        return cpp!(unsafe [ptr as "voronoicell*",
                    xyz as "double*"] -> bool as "bool" {
            return ptr->plane(xyz[0], xyz[1], xyz[2]);
        });
    }
}

impl Clone for VoronoiCell {
    fn clone(&self) -> Self {
        let ptr = self.0;
        Self(
            cpp!(unsafe [ptr as "voronoicell*"] -> *mut std::ffi::c_void as "voronoicell*" {
                voronoicell* x = new voronoicell;
                *x = *ptr;
                return x;
            }),
        )
    }
}

impl Drop for VoronoiCell {
    fn drop(&mut self) {
        let ptr = self.0;
        cpp!(unsafe [ptr as "voronoicell*"] {
            delete ptr;
        })
    }
}

/// Call every public API entry point. Check for sane results & no program crashes.
#[test]
fn ffi_sanity() {
    let x = VoronoiCell::init(&[0.0; 3], &[1.0; 3]);
    let x = x.clone();
    assert!(x.number_of_faces() == 6);
    assert!(x.number_of_edges() == 12);
    assert!(x.number_of_vertices() == 8);
    assert!(x.surface_area() == 6.0);
    assert!(x.volume() == 1.0);
    assert!(x.total_edge_distance() == 12.0);
    assert!(x.vertices().len() == 8);
    assert!(x.face_areas() == vec![1.0; 6]);
    assert!(x.face_perimeters() == vec![4.0; 6]);
    assert!(x.normals().len() == 6);
    for n in x.normals() {
        assert!(n.iter().all(|&z| z == 0.0 || z.abs() == 1.0));
        assert!(n.iter().sum::<f64>().abs() == 1.0);
    }
    assert!(x.face_vertices().len() == 6);
    for f in x.face_vertices() {
        assert!(f.iter().all(|&z| z < x.number_of_vertices() as usize));
        let mut q = f.clone();
        q.sort();
        q.dedup();
        assert!(q.len() == f.len());
    }
    assert!(x.max_radius_squared() == 3.0);
    let mut x = x;
    x.translate(&[2.0, -2.0, 0.5]);
    x.translate(&[-2.0, 2.0, -0.5]);
    assert!(x.centroid() == [0.5, 0.5, 0.5]);

    assert!(x.plane(&[10.0, 10.0, 10.0]) == true);
    assert!(x.plane(&[1.0, 1.0, 1.0]) == true);
    x.translate(&[3.3, 3.3, 3.3]);
    assert!(x.plane(&[1.0, 1.0, 1.0]) == false);

    let octahedron = VoronoiCell::init_octahedron(1.0);
    assert!(octahedron.number_of_faces() == 8);

    let tetrhedron = VoronoiCell::init_tetrahedron(
        &[0.0, 0.0, 0.0],
        &[1.0, 1.0, 1.0],
        &[0.0, 1.0, 0.0],
        &[1.0, 1.0, 0.0],
    );
    assert!(tetrhedron.number_of_faces() == 4);
}
