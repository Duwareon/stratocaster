use hecs::*;
use macroquad::prelude::*;

// ECS properties
#[derive(Debug)]
struct Position(Vec2);

#[derive(Debug)]
#[allow(dead_code)]
struct Velocity(Vec2);

#[derive(Debug)]
struct Rotation(Vec2);

#[derive(Debug)]
struct Player;

struct Active(bool);

struct Tilemap([[u8; 24]; 24]);

fn color_from_val(val: u8) -> Color {
    match val {
        1 => YELLOW,
        2 => GREEN,
        3 => BLUE,
        4 => PINK,
        5 => ORANGE,
        _ => BLACK,
    }
}

fn draw_tilemap(mapscale: f32, world: &World) {
    for (_id, (map, active)) in world.query::<(&Tilemap, &Active)>().iter() {
        if active.0 {
            let mut j = 0;
            for row in map.0 {
                let mut i = 0;
                for tile in row {
                    draw_rectangle(
                        i as f32 * mapscale,
                        j as f32 * mapscale,
                        mapscale,
                        mapscale,
                        color_from_val(tile),
                    );
                    i += 1;
                }
                j += 1;
            }
        }
    }

    for (_id, (pos, rot, actv, _)) in world
        .query::<(&Position, &Rotation, &Active, &Player)>()
        .iter()
    {
        if actv.0 {
            let renderpos = Vec2::new(
                pos.0.x * mapscale,
                pos.0.y * mapscale,
            );
            draw_line(
                renderpos.x,
                renderpos.y,
                renderpos.x + rot.0.x * mapscale * 20.0,
                renderpos.y + rot.0.y * mapscale * 20.0,
                mapscale / 6.0,
                WHITE,
            );
            draw_circle(renderpos.x, renderpos.y, mapscale / 2.0, RED);
        }
    }
}

                    

fn single_cast(pos: Vec2, angle: Vec2, map: &Tilemap) -> (u8, f32) {
    let mut mappos = Vec2::new(pos.x.floor(), pos.y.floor());
    
    //draw_line(pos.x*5.0, pos.y*5.0, (pos.x+angle.x)*5.0, (pos.y+angle.y)*5.0, 1.0, WHITE);

    let d_dist = Vec2::new((1.0 / angle.x).abs(), (1.0 / angle.y).abs());

    let step = Vec2::new(angle.x.signum(), angle.y.signum());

    let mut side_dist = Vec2::new(0.0, 0.0);

    side_dist.x = (pos.x*(-step.x) - mappos.x*(-step.x) + step.x.max(0.0)) * d_dist.x;
    side_dist.y = (pos.y*(-step.y) - mappos.y*(-step.y) + step.y.max(0.0)) * d_dist.y;

    // DDA
    let mut hit = false;
    let mut side = false;
    while !hit {
        if side_dist.x < side_dist.y {
            side_dist.x += d_dist.x;
            mappos.x += step.x;
            side = false;
        }
        else {
            side_dist.y += d_dist.y;
            mappos.y += step.y;
            side = true;
        }

        if map.0[mappos.y as usize][mappos.x as usize] > 0 { hit = true }
    }

    // Sometimes mappos is shifted by 1
    let val = map.0[mappos.y as usize][mappos.x as usize];
    let depth;
    if side { depth = side_dist.y - d_dist.y }
    else { depth = side_dist.x - d_dist.x }

    //let depth = (pos - mappos).length(); // Fisheye effect
    
    (val, depth)
}

fn wall_height(depth: f32, height: f32) -> (f32, f32) {
    let rectheight = height/depth;
    let top = -rectheight / 2.0 + height / 2.0;
    (top, rectheight)
}

fn draw_raycaster(resolution: usize, world: &World) {
    for (_id, (map, mapactv)) in world.query::<(&Tilemap, &Active)>().iter() {
        if mapactv.0 {
            for (_id, (pos, rot, plyractv, _)) in world.query::<(&Position, &Rotation, &Active, &Player)>().iter() {
                if plyractv.0 {
                    let step = 1.0/resolution as f32;


                    let plane = rot.0.perp();

                    for i in 0..resolution {
                        let camera = 2.0 * i as f32 / resolution as f32 - 1.0; // Position on camera plane
                        let rayangle = rot.0 + plane * camera; // Angle through camera plane
                        let (val, depth) = single_cast(pos.0, rayangle, &map);

                        let mut color = Color::from_vec(color_from_val(val).to_vec()
						    /(depth/10.0).exp());
                        color.a = 1.0;

                        let (walltop, wallheight) = wall_height(depth, screen_height());
                        draw_rectangle(screen_width()*step*i as f32, walltop, screen_width()*step, wallheight, color);
                    }
                }
            }
        }
    }
}

#[macroquad::main("bruh")]
async fn main() {
    let mut world: World = World::new();

    world.spawn((
        // Add player
        Position(Vec2::new(8.5, 6.5)),
        Velocity(Vec2::new(0.0, 0.0)),
        Rotation(Vec2::from_angle(3.14 / 2.0)),
        Player,
        Active(true),
    ));

    #[rustfmt::skip]
    let mut map: [[u8; 24]; 24] = [
        [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 2, 2, 2, 2, 2, 0, 0, 0, 0, 3, 0, 3, 0, 3, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 2, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 2, 0, 0, 0, 2, 0, 0, 0, 0, 3, 0, 0, 0, 3, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 2, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 2, 2, 0, 2, 2, 0, 0, 0, 0, 3, 0, 3, 0, 3, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 4, 4, 4, 4, 4, 4, 4, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 4, 0, 4, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 4, 0, 0, 0, 0, 5, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 4, 0, 4, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 4, 0, 4, 4, 4, 4, 4, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 4, 4, 4, 4, 4, 4, 4, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
    ];

    world.spawn((Tilemap(map), Active(true)));

    loop {
        let dt = get_frame_time();
        request_new_screen_size(640.0, 480.0);

        // Create toggling tile
        for (_id, (map, actv)) in world.query_mut::<(&mut Tilemap, &Active)>() {
            if actv.0 {
                if (get_time()-get_time().round() > 0 as f64) {map.0[12][11] = 0;}
                else {map.0[12][11] = 4;}
            }
        }

        // Draw world
        draw_rectangle(0.0, 0.0, screen_width(), screen_height()/2.0, BLUE);
        draw_rectangle(0.0, screen_height()/2.0, screen_width(), screen_height()/2.0, BLACK);
        draw_raycaster(0b1_0000_0000, &world);

        // Do player movement
        for (_id, (pos, rot, _)) in world.query_mut::<(&mut Position, &mut Rotation, &Player)>() {
            //if is_key_down(KeyCode::Up) { pos.0.y -= 10.0 * dt }
            //else if is_key_down(KeyCode::Down) { pos.0.y += 10.0 * dt }
            let speed: f32 = 5.0;
            let turnspeed: f32 = 1.5;

            if is_key_down(KeyCode::Up) {
                pos.0 += rot.0 * speed * dt
            } else if is_key_down(KeyCode::Down) {
                pos.0 -= rot.0 * speed * dt
            }

            if is_key_down(KeyCode::LeftShift) || is_key_down(KeyCode::RightShift) {
                if is_key_down(KeyCode::Left) {
                    pos.0 -= rot.0.perp() * speed * dt
                } else if is_key_down(KeyCode::Right) {
                    pos.0 += rot.0.perp() * speed * dt
                }
            } else {
                if is_key_down(KeyCode::Left) {
                    rot.0 = Vec2::from_angle(-turnspeed * dt).rotate(rot.0)
                } else if is_key_down(KeyCode::Right) {
                    rot.0 = Vec2::from_angle(turnspeed * dt).rotate(rot.0)
                }
            }
            
            //println!("Playerpos: {:?}", pos.0)
        }

        // Draw minimap
        draw_tilemap(5.0, &world);

        // Limit to framerate
        let minimum_frame_time = 1. / 144.; // 60 FPS
        let frame_time = get_frame_time();
        //println!("Frame time: {}ms", frame_time * 1000.);
        if frame_time < minimum_frame_time {
                let time_to_sleep = (minimum_frame_time - frame_time) * 1000.;
                //println!("Sleep for {}ms", time_to_sleep);
                std::thread::sleep(std::time::Duration::from_millis(time_to_sleep as u64));
        }

        next_frame().await;
    }
}
