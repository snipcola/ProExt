use mint::{Vector2, Vector3};

#[derive(Clone, Copy)]
pub struct View {
    pub matrix: [[f32; 4]; 4]
}

impl View {
    pub fn world_to_screen(&self, pos: Vector3<f32>, to_pos: &mut Vector2<f32>, ((_, _), (x, y)): ((i32, i32), (i32, i32))) -> bool {
        let view = self.matrix[3][0] * pos.x + self.matrix[3][1] * pos.y + self.matrix[3][2] * pos.z + self.matrix[3][3]; 
        
        if view <= 0.01 {
            return false;
        }

        let sight_x = x as f32 / 2.0;
        let sight_y = y as f32 / 2.0;

        to_pos.x = sight_x + (self.matrix[0][0] * pos.x + self.matrix[0][1] * pos.y + self.matrix[0][2] * pos.z + self.matrix[0][3]) / view * sight_x;
        to_pos.y = sight_y - (self.matrix[1][0] * pos.x + self.matrix[1][1] * pos.y + self.matrix[1][2] * pos.z + self.matrix[1][3]) / view * sight_y;

        return true;
    }
}