use ray_tracing::scenes::bouncing_spheres::bouncing_spheres;
use ray_tracing::scenes::checkered_spheres::checkered_spheres;
use ray_tracing::scenes::earth::earth;
use ray_tracing::scenes::perlin_spheres::perlin_spheres;
use ray_tracing::scenes::quads::quads;
use ray_tracing::scenes::simple_light::simple_light;

#[allow(dead_code)]
enum Scene {
    Bouncing,
    Checkered,
    Earth,
    Perlin,
    Quads,
    SimpleLight,
}

fn main() {
    let scene = Scene::SimpleLight;
    match scene {
        Scene::Bouncing => bouncing_spheres(),
        Scene::Checkered => checkered_spheres(),
        Scene::Earth => earth(),
        Scene::Perlin => perlin_spheres(),
        Scene::Quads => quads(),
        Scene::SimpleLight => simple_light(),
    }
}
