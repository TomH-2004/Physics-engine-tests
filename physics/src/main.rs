use macroquad::prelude::*;

struct Circle {
    x: f32,
    y: f32,
    radius: f32,
    is_dragging: bool,
    drag_offset: (f32, f32),
}

struct Rect {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
}

#[macroquad::main("Circle with Boundaries", window_conf)]
async fn main() {
    let boundaries = vec![
        Rect {
            x: 100.0,
            y: 100.0,
            width: 200.0,
            height: 20.0,
        },
        Rect {
            x: 300.0,
            y: 300.0,
            width: 20.0,
            height: 200.0,
        },
    ];

    let mut circle = Circle {
        x: screen_width() / 2.0,
        y: screen_height() / 2.0,
        radius: 50.0,
        is_dragging: false,
        drag_offset: (0.0, 0.0),
    };

    let mut was_mouse_down = false;

    loop {
        clear_background(RED);

        let mouse_down = is_mouse_button_down(MouseButton::Left);

        if mouse_down && !was_mouse_down {
            
            let mouse_x = mouse_position().0;
            let mouse_y = mouse_position().1;
            let distance = (mouse_x - circle.x).powi(2) + (mouse_y - circle.y).powi(2);
            let radius_squared = circle.radius.powi(2);

            if distance <= radius_squared {
                circle.is_dragging = true;
                circle.drag_offset = (circle.x - mouse_x, circle.y - mouse_y);
            }
        }

        if !mouse_down {
            circle.is_dragging = false;
        }

        if circle.is_dragging {
            
            circle.x = mouse_position().0 + circle.drag_offset.0;
            circle.y = mouse_position().1 + circle.drag_offset.1;
        }

        
        for boundary in &boundaries {
            if circle_in_walls(&circle, boundary) {
                
                circle.x = circle.x.clamp(
                    boundary.x + circle.radius,
                    boundary.x + boundary.width - circle.radius,
                );
                circle.y = circle.y.clamp(
                    boundary.y + circle.radius,
                    boundary.y + boundary.height - circle.radius,
                );
            }
        }

        
        for boundary in &boundaries {
            draw_rectangle(boundary.x, boundary.y, boundary.width, boundary.height, GREEN);
        }

        draw_circle(circle.x, circle.y, circle.radius, WHITE);

        was_mouse_down = mouse_down;

        next_frame().await;
    }
}

fn circle_in_walls(circle: &Circle, wall: &Rect) -> bool {
    circle.x - circle.radius < wall.x + wall.width
        && circle.x + circle.radius > wall.x
        && circle.y - circle.radius < wall.y + wall.height
        && circle.y + circle.radius > wall.y
}
