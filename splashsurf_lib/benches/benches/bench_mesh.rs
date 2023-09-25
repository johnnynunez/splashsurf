use criterion::{criterion_group, Criterion};
use splashsurf_lib::io::particles_from_file;
use splashsurf_lib::nalgebra::Vector3;
use splashsurf_lib::{
    reconstruct_surface, OctreeDecompositionParameters, Parameters,
    ParticleDensityComputationStrategy, SpatialDecomposition, SubdivisionCriterion,
    SurfaceReconstruction,
};
use std::path::Path;
use std::time::Duration;

fn reconstruct_particles<P: AsRef<Path>>(particle_file: P) -> SurfaceReconstruction<i64, f32> {
    let particle_positions: &Vec<Vector3<f32>> = &particles_from_file(particle_file).unwrap();

    let particle_radius = 0.011;
    let compact_support_radius = 4.0 * particle_radius;
    let cube_size = 1.5 * particle_radius;

    let parameters = Parameters {
        particle_radius,
        rest_density: 1000.0,
        compact_support_radius,
        cube_size,
        iso_surface_threshold: 0.6,
        particle_aabb: None,
        enable_multi_threading: true,
        spatial_decomposition: Some(SpatialDecomposition::Octree(
            OctreeDecompositionParameters {
                subdivision_criterion: SubdivisionCriterion::MaxParticleCountAuto,
                ghost_particle_safety_factor: None,
                enable_stitching: true,
                particle_density_computation:
                    ParticleDensityComputationStrategy::SynchronizeSubdomains,
            },
        )),
        global_neighborhood_list: false,
    };

    reconstruct_surface::<i64, _>(particle_positions.as_slice(), &parameters).unwrap()
}

pub fn mesh_vertex_normals(c: &mut Criterion) {
    //let reconstruction = reconstruct_particles("../../canyon_13353401_particles.vtk");
    let reconstruction = reconstruct_particles("../data/hilbert_46843_particles.bgeo");
    let mesh = reconstruction.mesh();

    let mut group = c.benchmark_group("mesh");
    group.sample_size(50);
    group.warm_up_time(Duration::from_secs(3));
    group.measurement_time(Duration::from_secs(10));

    group.bench_function("mesh_vertex_normals", |b| {
        b.iter(|| {
            let normals = mesh.vertex_normals();
            criterion::black_box(normals)
        })
    });

    group.finish();
}

pub fn mesh_vertex_normals_parallel(c: &mut Criterion) {
    //let reconstruction = reconstruct_particles("../../canyon_13353401_particles.vtk");
    let reconstruction = reconstruct_particles("../data/hilbert_46843_particles.bgeo");
    let mesh = reconstruction.mesh();

    let mut group = c.benchmark_group("mesh");
    group.sample_size(50);
    group.warm_up_time(Duration::from_secs(3));
    group.measurement_time(Duration::from_secs(10));

    group.bench_function("mesh_vertex_normals_parallel", |b| {
        b.iter(|| {
            let normals = mesh.par_vertex_normals();
            criterion::black_box(normals)
        })
    });

    group.finish();
}

criterion_group!(
    bench_mesh,
    mesh_vertex_normals,
    mesh_vertex_normals_parallel
);
