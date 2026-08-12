use ray_tracing::scenes::bouncing_spheres::bouncing_spheres;
use ray_tracing::scenes::checkered_spheres::checkered_spheres;
use ray_tracing::scenes::cornell_box::cornell_box;
use ray_tracing::scenes::cornell_smoke::cornell_smoke;
use ray_tracing::scenes::earth::earth;
use ray_tracing::scenes::final_scene::final_scene;
use ray_tracing::scenes::perlin_spheres::perlin_spheres;
use ray_tracing::scenes::quads::quads;
use ray_tracing::scenes::simple_light::simple_light;

#[allow(dead_code)]
enum Scene {
    Bouncing,
    Checkered,
    CornellBox,
    CornellSmoke,
    Earth,
    Final,
    FinalBenchmark,
    FinalHD,
    Perlin,
    Quads,
    SimpleLight,
}

fn main() {
    let scene = Scene::FinalBenchmark;
    match scene {
        Scene::Bouncing => bouncing_spheres(),
        Scene::Checkered => checkered_spheres(),
        Scene::CornellBox => cornell_box(),
        Scene::CornellSmoke => cornell_smoke(),
        Scene::Earth => earth(),
        Scene::Final => final_scene(400.0, 250, 4),
        Scene::FinalBenchmark => final_scene(800.0, 250, 10),
        Scene::FinalHD => final_scene(800.0, 10000, 40),
        Scene::Perlin => perlin_spheres(),
        Scene::Quads => quads(),
        Scene::SimpleLight => simple_light(),
    }
}
