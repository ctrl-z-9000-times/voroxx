use cpp::cpp;

cpp! {{
    #include "voro++.hh"
    using namespace voro;
}}

/// Make a rust vector with the given size and return a pointer to its internal
/// data buffer.
macro_rules! vec_ptr_pair {
    ($size:expr) => {{
        let size: usize = ($size);
        let mut rust_vec = Vec::with_capacity(size);
        unsafe {
            rust_vec.set_len(size);
        }
        let data_ptr = rust_vec.as_mut_ptr();
        (rust_vec, data_ptr)
    }};
}

/// Private trait. Allows the public trait "VoronoiCellBase" to access the
/// pointer to the underlying C++ structure.
pub trait VoronoiCellBaseFFI {
    fn ptr(&self) -> *mut std::ffi::c_void;
}

/// Methods which are common to both variants of Voronoi cells.
pub trait VoronoiCellBase: VoronoiCellBaseFFI {
    /// Counts the number of faces of the Voronoi cell.
    fn number_of_faces(&self) -> i32 {
        let ptr = self.ptr();
        cpp!(unsafe [ptr as "voronoicell_base*"] -> i32 as "int" {
            return ptr->number_of_faces();
        })
    }

    /// Counts the number of edges of the Voronoi cell.
    fn number_of_edges(&self) -> i32 {
        let ptr = self.ptr();
        cpp!(unsafe [ptr as "voronoicell_base*"] -> i32 as "int" {
            return ptr->number_of_edges();
        })
    }

    /// Counts the total number of vertices of the Voronoi cell.
    fn number_of_vertices(&self) -> i32 {
        let ptr = self.ptr();
        cpp!(unsafe [ptr as "voronoicell_base*"] -> i32 as "int" {
            return ptr->p;
        })
    }

    /// Translates the vertices of the Voronoi cell by a given vector.
    fn translate(&mut self, xyz: &[f64; 3]) {
        let ptr = self.ptr();
        cpp!(unsafe [ptr as "voronoicell_base*",
            xyz as "double*"] -> f64 as "double" {
            ptr->translate(xyz[0], xyz[1], xyz[2]);
        });
    }

    /// Calculates the total surface area of the Voronoi cell.
    fn surface_area(&self) -> f64 {
        let ptr = self.ptr();
        cpp!(unsafe [ptr as "voronoicell_base*"] -> f64 as "double" {
            return ptr->surface_area();
        })
    }

    /// Calculates the volume of the Voronoi cell, by decomposing the cell into
    /// tetrahedra extending outward from the zeroth vertex, whose volumes are
    /// evaluated using a scalar triple product.
    fn volume(&self) -> f64 {
        let ptr = self.ptr();
        cpp!(unsafe [ptr as "voronoicell_base*"] -> f64 as "double" {
            return ptr->volume();
        })
    }

    /// Calculates the total edge distance of the Voronoi cell.
    fn total_edge_distance(&self) -> f64 {
        let ptr = self.ptr();
        cpp!(unsafe [ptr as "voronoicell_base*"] -> f64 as "double" {
            return ptr->total_edge_distance();
        })
    }

    /// Calculates the centroid of the Voronoi cell, by decomposing the cell
    /// into tetrahedra extending outward from the zeroth vertex.
    ///
    /// Returns (x, y, z) coordinates of the centroid.
    fn centroid(&self) -> [f64; 3] {
        let ptr = self.ptr();
        let mut c = [f64::NAN; 3];
        let x = &mut c;
        cpp!(unsafe [ptr as "voronoicell_base*", x as "double*"] {
            ptr->centroid(x[0], x[1], x[2]);
        });
        return c;
    }

    /// Returns a vector of the vertex coordinates using the local coordinate system.
    fn vertices(&self) -> Vec<[f64; 3]> {
        let ptr = self.ptr();
        let (coords, data_ptr) = vec_ptr_pair!(self.number_of_vertices() as usize);
        cpp!(unsafe [ptr as "voronoicell_base*", data_ptr as "double*"] {
            std::vector<double> temp;
            ptr->vertices(temp);
            std::copy(temp.begin(), temp.end(), data_ptr);
        });
        return coords;
    }

    /// Calculates the areas of each face of the Voronoi cell.
    fn face_areas(&self) -> Vec<f64> {
        let ptr = self.ptr();
        let (areas, data_ptr) = vec_ptr_pair!(self.number_of_faces() as usize);
        cpp!(unsafe [ptr as "voronoicell_base*", data_ptr as "double*"] {
            std::vector<double> temp;
            ptr->face_areas(temp);
            std::copy(temp.begin(), temp.end(), data_ptr);
        });
        return areas;
    }

    /// Calculates the perimeters of each face.
    fn face_perimeters(&self) -> Vec<f64> {
        let ptr = self.ptr();
        let (perimeters, data_ptr) = vec_ptr_pair!(self.number_of_faces() as usize);
        cpp!(unsafe [ptr as "voronoicell_base*", data_ptr as "double*"] {
            std::vector<double> temp;
            ptr->face_perimeters(temp);
            std::copy(temp.begin(), temp.end(), data_ptr);
        });
        return perimeters;
    }

    /// Returns the vertices that make up each face of the Voronoi cell, as
    /// indices into the vertices list.
    fn face_vertices(&self) -> Vec<Vec<usize>> {
        let ptr = self.ptr();
        let retval = vec![];
        let ptr_retval = &retval;
        cpp!(unsafe [ptr as "voronoicell_base*", ptr_retval as "void*"] {
            std::vector<int> temp;
            ptr->face_vertices(temp);
            int *data = temp.data();
            int len = temp.size();
            return rust!(_unused_name [data: *const i32 as "int*", len: i32 as "int",
                                        ptr_retval: &mut Vec<Vec<usize>> as "void*"] {
                use std::convert::TryInto;
                let mut countdown = 0;
                let mut face_idx = -1;
                for i in 0..len as isize {
                    let v = *data.offset(i);
                    if countdown == 0 {
                        countdown = v;
                        ptr_retval.push(Default::default());
                        face_idx += 1;
                    }
                    else {
                        countdown -= 1;
                        ptr_retval[face_idx as usize].push(v.try_into().unwrap());
                    }
                }
            });
        });
        return retval;
    }

    /// Calculates the normal vector of each face of the Voronoi cell, and
    /// scales it to the distance from the cell center to that plane.
    fn normals(&self) -> Vec<[f64; 3]> {
        let ptr = self.ptr();
        let (normals, data_ptr) = vec_ptr_pair!(self.number_of_faces() as usize);
        cpp!(unsafe [ptr as "voronoicell_base*", data_ptr as "double*"] {
            std::vector<double> temp;
            ptr->normals(temp);
            std::copy(temp.begin(), temp.end(), data_ptr);
        });
        return normals;
    }

    /// Calculates the maximum radius squared of any vertex from the center of
    /// the cell. This can be used to determine when enough particles have been
    /// testing an all planes that could cut the cell have been considered.
    fn max_radius_squared(&self) -> f64 {
        let ptr = self.ptr();
        cpp!(unsafe [ptr as "voronoicell_base*"] -> f64 as "double" {
            return ptr->max_radius_squared() * 0.25;
        })
    }

    // TODO: Drawing routines (all 6 of them?)
}

// I think these 2 methods are internal? There is no real reason that they can
// not be public, but also they don't look very useful...
// bool plane_intersects (double x, double y, double z, double rsq)
// bool plane_intersects_guess (double x, double y, double z, double rsq)

// What even is this method?
// void voro::voronoicell_base::construct_relations()
