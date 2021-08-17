use crate::{prelude::*, Frame};

#[system]
#[read_component(Point)]
#[read_component(Render)]
#[read_component(FieldOfView)]
#[read_component(Player)]
pub fn entity_render(ecs: &SubWorld, #[resource] camera: &Camera, #[resource] frame: &Frame) {
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(1);

    let mut renderables = <(&Point, &Render)>::query();
    let mut fov = <&FieldOfView>::query().filter(component::<Player>());

    let player_fov = fov.iter(ecs).next().unwrap();
    let offset = Point::new(camera.left_x, camera.top_y);

    renderables
        .iter(ecs)
        .filter(|(pos, _)| player_fov.visible_tiles.contains(pos))
        .for_each(|(point, render)| {
            let frame_idx = frame.animation_count % render.glyphs.len();
            draw_batch.set(*point - offset, render.color, render.glyphs[frame_idx]);
        });
    draw_batch.submit(5000).expect("Batch Error")
}
