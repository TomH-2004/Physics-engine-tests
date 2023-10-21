use macroquad::prelude::*;

struct Circle {
    x: f32,
    y: f32,
    radius: f32,
    is_dragging: bool,
    drag_offset: (f32, f32),
}

struct Wall {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
}

const SCREEN_WIDTH: f32 = 640.0;
const SCREEN_HEIGHT: f32 = 480.0;
const BACKGROUND_COLOR: Color = RED;
const WALL_COLOR: Color = BLUE;

#[macroquad::main("Circle with Walls", window_conf)]
async fn main() {
    let mut circle = Circle {
        x: SCREEN_WIDTH / 2.0,
        y: SCREEN_HEIGHT / 2.0,
        radius: 50.0,
        is_dragging: false,
        drag_offset: (0.0, 0.0),
    };

    let walls = vec![
        Wall {
            x: 100.0,
            y: 100.0,
            width: 200.0,
            height: 20.0,
        },
        Wall {
            x: 300.0,
            y: 300.0,
            width: 20.0,
            height: 200.0,
        },
    ];

    loop {
        clear_background(BACKGROUND_COLOR);

        for wall in &walls {
            draw_rectangle(wall.x, wall.y, wall.width, wall.height, WALL_COLOR);
        }

        let mouse_x = mouse_position().0;
        let mouse_y = mouse_position().1;

        if is_mouse_button_down(MouseButton::Left) {
            if !circle.is_dragging {
                if is_point_in_circle(mouse_x, mouse_y, &circle) {
                    circle.is_dragging = true;
                    circle.drag_offset = (circle.x - mouse_x, circle.y - mouse_y);
                }
            }
        } else {
            circle.is_dragging = false;
        }

        if circle.is_dragging {
            let new_x = mouse_x + circle.drag_offset.0;
            let new_y = mouse_y + circle.drag_offset.1;

            
            for wall in &walls {
                if is_circle_rect_collision(&circle, wall) {
                    
                    let response = circle_wall_collision_response(&circle, wall);
                    circle.x += response.0;
                    circle.y += response.1;
                }
            }

            
            circle.x = new_x.clamp(circle.radius, SCREEN_WIDTH - circle.radius);
            circle.y = new_y.clamp(circle.radius, SCREEN_HEIGHT - circle.radius);
        }

        draw_circle(circle.x, circle.y, circle.radius, WHITE);

        next_frame().await;
    }
}

fn is_point_in_circle(px: f32, py: f32, circle: &Circle) -> bool {
    let dx = px - circle.x;
    let dy = py - circle.y;
    dx * dx + dy * dy <= circle.radius * circle.radius
}

fn is_circle_rect_collision(circle: &Circle, rect: &Wall) -> bool {
    let closest_x = circle.x.clamp(rect.x, rect.x + rect.width);
    let closest_y = circle.y.clamp(rect.y, rect.y + rect.height);

    let distance_x = circle.x - closest_x;
    let distance_y = circle.y - closest_y;

    (distance_x * distance_x + distance_y * distance_y) <= (circle.radius * circle.radius)
}

fn circle_wall_collision_response(circle: &Circle, wall: &Wall) -> (f32, f32) {
    let dx = circle.x - wall.x.max(circle.x.min(wall.x + wall.width));
    let dy = circle.y - wall.y.max(circle.y.min(wall.y + wall.height));
    let distance = dx * dx + dy * dy;

    if distance < circle.radius * circle.radius {
        
        let magnitude = distance.sqrt();
        let overlap = circle.radius - magnitude;
        let response_x = dx * (overlap / magnitude);
        let response_y = dy * (overlap / magnitude);
        (response_x, response_y)
    } else {
        (0.0, 0.0)
    }
}
