//===============================================================

//===============================================================

pub struct CameraOrthographic {
    pub left: f32,
    pub right: f32,
    pub bottom: f32,
    pub top: f32,
    pub z_near: f32,
    pub z_far: f32,
}
impl Default for CameraOrthographic {
    fn default() -> Self {
        Self {
            left: 0.,
            right: 600.,
            bottom: 0.,
            top: 400.,
            z_near: 0.,
            z_far: 10000.,
        }
    }
}
impl CameraOrthographic {
    pub fn new_sized(width: f32, height: f32, z_near: f32, z_far: f32) -> Self {
        let half_width = width / 2.;
        let half_height = height / 2.;
        Self {
            left: -half_width,
            right: half_width,
            bottom: -half_height,
            top: half_height,
            z_near,
            z_far,
        }
    }

    pub fn update(&mut self, left: f32, right: f32, bottom: f32, top: f32) {
        self.left = left;
        self.right = right;
        self.bottom = bottom;
        self.top = top;
    }

    pub fn update_sized(&mut self, width: f32, height: f32) {
        let half_width = width / 2.;
        let half_height = height / 2.;

        self.left = -half_width;
        self.right = half_width;
        self.bottom = -half_height;
        self.top = half_height;
    }

    pub fn get_projection(&self) -> glam::Mat4 {
        glam::Mat4::orthographic_lh(
            self.left,
            self.right,
            self.bottom,
            self.top,
            self.z_near,
            self.z_far,
        )
    }
    pub fn get_projection_transform(&self, pos: glam::Vec2, rotation: glam::Quat) -> glam::Mat4 {
        let projection_matrix = self.get_projection();
        let transform_matrix = glam::Mat4::from_rotation_translation(rotation, pos.extend(0.));

        projection_matrix * transform_matrix
    }
}

//===============================================================

pub struct CameraPerspective {
    pub up: glam::Vec3,
    pub aspect: f32,
    pub fovy: f32,
    pub z_near: f32,
    pub z_far: f32,
}
impl Default for CameraPerspective {
    fn default() -> Self {
        Self {
            up: glam::Vec3::Y,
            aspect: 1.77777777778,
            fovy: 45.,
            z_near: 0.1,
            z_far: 10000.,
        }
    }
}
impl CameraPerspective {
    pub fn get_projection(&self) -> glam::Mat4 {
        glam::Mat4::perspective_lh(self.fovy, self.aspect, self.z_near, self.z_far)
    }

    pub fn get_projection_transform(
        &self,
        position: glam::Vec3,
        rotation: glam::Quat,
    ) -> glam::Mat4 {
        let forward = (rotation * glam::Vec3::Z).normalize();

        let projection_matrix = self.get_projection();
        let view_matrix = glam::Mat4::look_at_lh(position, position + forward, self.up);

        projection_matrix * view_matrix
    }

    pub fn get_projection_target(&self, position: glam::Vec3, target: glam::Vec3) -> glam::Mat4 {
        let projection_matrix = self.get_projection();
        let view_matrix = glam::Mat4::look_at_lh(position, target, self.up);

        projection_matrix * view_matrix
    }
}

//===============================================================

//===============================================================
