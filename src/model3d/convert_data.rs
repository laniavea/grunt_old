use crate::model3d::Model3D;
use crate::model2d::Model2D;
use crate::types::generation_params::Params2D;
use crate::types::Axis; 

impl Model3D {
    pub fn get_by_num(&self, x: usize, y: usize) -> Result<Vec<i32>, &'static str> {
        if self.model.is_empty() { return Err("Model doesn't exists in object") };
        if self.model.len() < x || self.model[0].len() < y { return Err("X or Y out of bounds") };

        Ok(self.model[x][y].clone())
    }

    pub fn to_model_2d_by_angle(&self, pos_x: f32, angle: f32, resolution: usize) -> Result<Model2D, &'static str> {
        let angle = (angle * 1000.0).round() / 1000.0;
        let is_acute = angle < 90.0;

        if angle <= 0.0 || angle >= 180.0 {
            return Err("Angle must be between 0.0 and 180.0 degrees")
        }

        let x_ax_obj = self.params.x_axis();
        let y_ax_obj = self.params.y_axis();
        let x_ax = self.params.x_axis().axis();
        let pos_y = self.params.y_axis().axis()[0];
        let y_ax_size = self.params.y_axis().blocks_count() as f32;

        let end_x = if is_acute {
            x_ax[0]
        } else {
            x_ax[x_ax.len()-1]
        };
        
        let (delt_y, delt_x) = if is_acute {
            if pos_x < end_x {
                return Err("pos_x must be bigger than start of axis with acute angle");
            }

            let angle_tan = angle.to_radians().tan();
            let mut delt_x = pos_x - end_x;
            if y_ax_size / delt_x <  angle_tan {
                delt_x = y_ax_size / angle_tan;
            }

            (angle_tan * delt_x, delt_x)
        } else {
            if pos_x > end_x {
                return Err("pos_x must be smaller than end of axis with obtuse angle");
            }
            let angle_tan = (-angle).to_radians().tan();
            let mut delt_x = end_x - pos_x;
            if y_ax_size / delt_x <  angle_tan {
                delt_x = y_ax_size / angle_tan;
            }

            (angle_tan * delt_x, delt_x)
        };

        let (mut now_x, mut now_y) = (pos_x, pos_y);
        let (mut delt_x, delt_y) = (delt_x / (resolution - 1) as f32, delt_y / ((resolution - 1) as f32));

        if is_acute {
            delt_x = -delt_x
        }

        let mut nums_x: Vec<usize> = Vec::with_capacity(resolution);
        let mut nums_y: Vec<usize> = Vec::with_capacity(resolution);

        for _ in 0..(resolution) {
            nums_x.push(x_ax_obj.find_element_smaller(now_x).unwrap());
            nums_y.push(y_ax_obj.find_element_smaller(now_y).unwrap());

            now_x = ((now_x + delt_x) * 1000.0).round() / 1000.0;
            now_y = ((now_y + delt_y) * 1000.0).round() / 1000.0;
        }

        self.form_2d_by_nums(nums_x, nums_y)
    }

    pub fn form_2d_by_nums(&self, nums_x: Vec<usize>, nums_y: Vec<usize>) -> Result<Model2D, &'static str> {
        if nums_x.len() != nums_y.len() {
            return Err("Vectors cords_x and cords_y must be with same size")
        }

        let source_model = &self.model;
        let source_model_mask = &self.model_mask;

        // Borders format is Z->Y->X, using borders because model or mask can be empty
        let borders_model_size_z = self.borders.len();
        let source_model_size_y = self.borders[0].len();
        let source_model_size_x = self.borders[0][0].len();

        let model_ex: bool = !self.model.is_empty();
        let mask_ex: bool = !self.model_mask.is_empty();

        let mut borders: Vec<Vec<i32>> = Vec::with_capacity(nums_x.len());
        let mut model: Vec<Vec<i32>> = Vec::with_capacity(if model_ex {nums_x.len()} else {0});
        let mut model_mask: Vec<Vec<u8>> = Vec::with_capacity(if mask_ex {nums_x.len()} else {0});
        let mut x_ax: Vec<f32> = Vec::with_capacity(nums_x.len());

        for (num, x_num) in nums_x.iter().enumerate() {
            let y_num = nums_y[num];

            if y_num >= source_model_size_y || *x_num >= source_model_size_x {
                return Err("Invalid coodinates: it must be smaller than model size")
            }

            x_ax.push(num as f32);

            let mut borders_temp_vec: Vec<i32> = Vec::with_capacity(borders_model_size_z);
            for layer in &self.borders {
                borders_temp_vec.push(layer[y_num][*x_num]);
            }
            borders.push(borders_temp_vec);

            if model_ex {
                model.push(source_model[*x_num][y_num].clone());
            }

            if mask_ex {
                model_mask.push(source_model_mask[*x_num][y_num].clone());
            }
        }

        let mut params = Params2D::new();
        params.set_x_axis(Axis::create_from_edges(x_ax).unwrap());

        Ok(Model2D::new(
            model,
            model_mask,
            borders,
            params,
        ))
    }
}
