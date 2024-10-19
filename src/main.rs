use macroquad::prelude::*;
use hecs::*;

// ECS properties
#[derive(Debug)] 
struct Position(Vec2);

#[derive(Debug)] 
struct Velocity(Vec2);

#[derive(Debug)]
struct Rotation(Vec2);

#[derive(Debug)]
struct Player;

struct Active(bool);

struct Tilemap([[u8; 24]; 24]);

#[macroquad::main("bruh")]
async fn main() {
    let mut world: World = World::new();
    
    world.spawn(( // Add player
        Position(Vec2::new(8.0, 6.0)),
        Velocity(Vec2::new(0.0, 0.0)),
        Rotation(Vec2::from_angle(3.14/2.0)),
        Player,
        Active(true)
    ));

    let mut map: [[u8; 24]; 24] = [
        [1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1],
        [1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
        [1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
        [1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
        [1,0,0,0,0,0,2,2,2,2,2,0,0,0,0,3,0,3,0,3,0,0,0,1],
        [1,0,0,0,0,0,2,0,0,0,2,0,0,0,0,0,0,0,0,0,0,0,0,1],
        [1,0,0,0,0,0,2,0,0,0,2,0,0,0,0,3,0,0,0,3,0,0,0,1],
        [1,0,0,0,0,0,2,0,0,0,2,0,0,0,0,0,0,0,0,0,0,0,0,1],
        [1,0,0,0,0,0,2,2,0,2,2,0,0,0,0,3,0,3,0,3,0,0,0,1],
        [1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
        [1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
        [1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
        [1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
        [1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
        [1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
        [1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
        [1,4,4,4,4,4,4,4,4,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
        [1,4,0,4,0,0,0,0,4,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
        [1,4,0,0,0,0,5,0,4,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
        [1,4,0,4,0,0,0,0,4,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
        [1,4,0,4,4,4,4,4,4,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
        [1,4,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
        [1,4,4,4,4,4,4,4,4,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
        [1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1]
    ];

    world.spawn((Tilemap(map), Active(true)));

    loop {
        let dt = get_frame_time();    
        
        // TODO: Actually add raycaster

        let mapscale: f32 = 5.0;

        for (id, (map, active)) in world.query_mut::<(&Tilemap, &Active)>() {
            let mut j = 0;
            for row in map.0 {
                let mut i = 0;
                for tile in row {
                    let tilecolor = match tile {
                        1 => YELLOW,
                        2 => GREEN,
                        3 => BLUE,
                        4 => PINK,
                        5 => ORANGE,
                        _ => BLACK,
                    };
                    draw_rectangle(i as f32 * mapscale, j as f32 * mapscale, mapscale, mapscale, tilecolor);
                    i+=1;
                }
                j+=1;
            }
        }

        for (id, (pos, rot, _)) in world.query_mut::<(&mut Position, &mut Rotation, &Player)>() {
            //if is_key_down(KeyCode::Up) { pos.0.y -= 10.0 * dt }
            //else if is_key_down(KeyCode::Down) { pos.0.y += 10.0 * dt }
            let speed: f32 = 10.0;
            let turnspeed: f32 = 2.0;

            if is_key_down(KeyCode::Up) { pos.0 += rot.0 * speed * dt }
            else if is_key_down(KeyCode::Down) { pos.0 -= rot.0 * speed * dt }

            if is_key_down(KeyCode::LeftShift) || is_key_down(KeyCode::RightShift) {
                if is_key_down(KeyCode::Left) { pos.0 -= rot.0.perp() * speed * dt }
                else if is_key_down(KeyCode::Right) { pos.0 += rot.0.perp() * speed * dt }
            }
            else {
                if is_key_down(KeyCode::Left) { rot.0 = Vec2::from_angle(-turnspeed * dt).rotate(rot.0) }
                else if is_key_down(KeyCode::Right) { rot.0 = Vec2::from_angle(turnspeed * dt).rotate(rot.0) }
            }

            let renderpos = Vec2::new(pos.0.x * mapscale + mapscale/2.0, pos.0.y * mapscale + mapscale/2.0);
            draw_line(renderpos.x, renderpos.y, renderpos.x + rot.0.x*mapscale*1.5, renderpos.y + rot.0.y*mapscale*1.5, mapscale/4.0, WHITE);
            draw_circle(renderpos.x, renderpos.y, mapscale/2.0, RED);
        }

    next_frame().await;
    }
}
