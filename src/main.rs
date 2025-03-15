use clap::{
    Parser,
    ValueEnum,
};
mod scenes;

#[allow(clippy::enum_variant_names)]
#[derive(Parser, ValueEnum, Clone, Default)]
enum Scene {
    #[default]
    ManySpheres,
    ManyBouncingSpheres,
    CheckerSpheres,
    Globe,
}

#[derive(Parser)]
struct Args {
    /// Enable draft mode for faster rendering
    #[arg(short, long)]
    draft: bool,
    /// The scene to render
    #[arg(short, long, default_value = "many-spheres")]
    scene: Scene,
}

fn main() {
    let args = Args::parse();
    let mut builder = match args.scene {
        Scene::ManySpheres => scenes::many_spheres(),
        Scene::ManyBouncingSpheres => scenes::many_bouncing_spheres(),
        Scene::CheckerSpheres => scenes::checkered_spheres(),
        Scene::Globe => scenes::world(),
    };
    if args.draft {
        builder = builder.draft();
    }
    let camera = builder.build();
    camera.render_to_file();
}
