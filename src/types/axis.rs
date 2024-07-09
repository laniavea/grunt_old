use numtoa::NumToA;

use crate::types::Axis;
use crate::types::AxisExportType;

#[derive(Debug, Clone)]
pub enum AxisError {
    InvalidRange,
    InsufficientElements,
    NonIncreasingValues,
    NonPositiveStep,
}

impl std::fmt::Display for AxisError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AxisError::InvalidRange => write!(f, "Coordinates for the end the model must be greater than the start ones"),
            AxisError::InsufficientElements => write!(f, "Vec must contain at least two (for edges) or one (for blocks) elements"),
            AxisError::NonIncreasingValues => write!(f, "Coordinates inside the vec must constatnly increase"),
            AxisError::NonPositiveStep => write!(f, "Step must be bigger than zero"),
        }
    }
}

impl std::error::Error for AxisError {}

impl Default for Axis {
    fn default() -> Axis {
        Axis::new()
    }
}

impl Axis {
    pub fn new() -> Axis {
        let axis = Axis::calculate_axis(0.0, 10.0, 1.0);
        Axis {
            start: 0.0,
            end: 10.0,
            step: Some(1.0),
            blocks_count: 10,
            centers: Axis::centers_from_edges(&axis),
            axis,
        }
    }

    pub fn generate_axis<T: Into<f32>>(start: T, end: T, step: Option<T>) -> Result<Axis, AxisError> {
        let (start, end) = (start.into(), end.into());
        if start >= end {
            return Err(AxisError::InvalidRange);
        }
        let step = step.map(|step| step.into());

        Axis::generate_axis_common(start, end, step, false)
    }

    pub fn generate_axis_hard<T: Into<f32>>(start: T, end: T, step: Option<T>) -> Result<Axis, AxisError> {
        let (start, end) = (start.into(), end.into());
        if start > end {
            return Err(AxisError::InvalidRange);
        }
        let step = step.map(|step| step.into());

        Axis::generate_axis_common(start, end, step, true)
    }

    fn generate_axis_common (start: f32, end: f32, step: Option<f32>, is_hard: bool) -> Result<Axis, AxisError> {
        let new_step = step.unwrap_or(1.0);
        if new_step <= 0.0 {
            return Err(AxisError::NonPositiveStep)
        }

        let start = (start * 1000.0).round() / 1000.0;
        let end = (end * 1000.0).round() / 1000.0;

        let mut axis = Axis::calculate_axis(start, end, new_step);

        if is_hard && axis[axis.len() - 1] != end {
            axis.push(end)
        } else if axis.len() < 2{
            return Err(AxisError::InsufficientElements)
        }

        Ok(Axis {
            start,
            end,
            step,
            blocks_count: axis.len() - 1,
            centers: Axis::centers_from_edges(&axis),
            axis,
        })
    }

    pub fn create_from_edges(axis: Vec<f32>) -> Result<Axis, AxisError> {
        if axis.len() < 2 {
            return Err(AxisError::InsufficientElements);
        }

        let mut pr_el = axis[0];

        for i in axis[1..].iter() {
            if *i <= pr_el {
                return Err(AxisError::NonIncreasingValues);
            }
            pr_el = *i;
        }

        Ok(Axis{
            start: axis[0],
            end: axis[axis.len() - 1],
            step: None,
            blocks_count: axis.len() - 1,
            centers: Axis::centers_from_edges(&axis),
            axis,
        })
    }

    /// Creates vec of axis based on its limits and step, end value may be excluded
    fn calculate_axis(start: f32, end: f32, step: f32) -> Vec<f32> {
        (0..(((end-start)/step * 1000.0).round() / 1000.0 + 1.0).floor() as i32)
            .map(|num| ((start + num as f32 * step) * 1000.0).round() / 1000.0)
            .collect()
    }

    fn centers_from_edges(edges_vec: &[f32]) -> Vec<f32> {
        let mut pr_elem: f32 = edges_vec[0];
        let mut centers: Vec<f32> = Vec::with_capacity(edges_vec.len() - 1);
        for elem in edges_vec[1..].iter() {
            centers.push((((pr_elem + elem) / 2.0) * 1000.0).round() / 1000.0);
            pr_elem = *elem;
        }
        centers
    }
}

// TODO: Recreate this method or remove
impl Axis {
    pub fn find_element_smaller(&self, _target: f32) -> Option<usize> {
        Some(5)
    }

    pub fn convert_to_perp_ax(_pos: f32, angle: f32) {
        let _new_angle = ((180.0 - angle) * 1000.0).round() / 1000.0;

    }
}

impl Axis {
    fn export_num_ax(&self, result: &mut String) {
        let mut buf = [0u8; 20];
        for i in 0..self.blocks_count{
            *result += i.numtoa_str(10, &mut buf);
            *result += ",";
        }
        *result += (self.blocks_count).numtoa_str(10, &mut buf)
    }

    // TODO: Scale f64 values for better accuracy
    fn export_scale_ax(&self, result: &mut String, scale_value: f32) {
        let scale_value = scale_value as f64;
        for i in self.axis[..self.axis.len() - 1].iter() {
            *result += &((*i as f64) * scale_value).to_string();
            *result += ",";
        }
        *result += &((self.axis[self.axis.len() - 1] as f64) * scale_value).to_string();
    }

    fn export_self_ax(&self, result: &mut String) {
        for i in self.axis[..self.axis.len() - 1].iter() {
            *result += &i.to_string();
            *result += ",";
        }
        *result += &self.axis[self.axis.len() - 1].to_string();
    }

    pub fn export_axis(&self, export_type: &AxisExportType, result: &mut String) {
        match export_type {
            AxisExportType::AsNum => self.export_num_ax(result),
            AxisExportType::AsSelf => self.export_self_ax(result),
            AxisExportType::Scale(scale_param) => self.export_scale_ax(result, *scale_param),
            AxisExportType::CustomAxis(ax) => Axis::create_from_edges(ax.to_vec()).unwrap().export_self_ax(result),
        }
    }
}

impl Axis {
    /// Return all information about axis
    pub fn get_full_axis(&self) -> (f32, f32, Option<f32>, &Vec<f32>) {
        (self.start, self.end, self.step, &self.axis)
    }

    /// Return start coord of first block
    pub fn start(&self) -> f32 {
        self.start
    }

    /// Returns end coord of last block
    pub fn end(&self) -> f32 {
        self.end
    }

    /// Return step value if axis was generated
    pub fn step(&self) -> Option<f32> {
        self.step
    }

    /// Returns number of blocks inside axis (-1 from axis len)
    pub fn blocks_count(&self) -> usize {
        self.blocks_count
    }

    /// Returns ref to axis vec
    pub fn axis(&self) -> &Vec<f32> {
        &self.axis
    }

    /// Returns ref to centers vec
    pub fn centers(&self) -> &Vec<f32> {
        &self.centers
    }
}
