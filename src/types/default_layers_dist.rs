use rand::Rng; 

use crate::types::LayersDist;

impl Default for LayersDist {
    fn default() -> LayersDist {
        LayersDist::new()
    }
}

impl LayersDist {
    pub fn new() -> LayersDist {
        LayersDist {
            layers_num: 3,
            max_layer_size: 100,
            min_layer_size:70,
            layers_sum: 240,
            layers_dist: vec![70, 90, 80],
            layers_dist_summed: vec![70, 160, 240],
        }
    }

    pub fn create_from_vec(layers_dist: Vec<i32>) -> Result<LayersDist, &'static str> {
        if layers_dist.is_empty() {
            return Err("Distibution of layers vec must contain at least one value");
        }

        if layers_dist.len() >= 255 {
            return Err("Program do not support models with more than 255 layers")
        }

        let (mut min_layer_size, mut max_layer_size, mut layers_sum) = (i32::MAX, 0i32, 0i32);
        let mut layers_dist_summed: Vec<i32> = vec![];

        for el in &layers_dist {
            if *el < min_layer_size { min_layer_size = *el }
            if *el > max_layer_size { max_layer_size = *el }
            if *el <= 0 { return Err("Elements should be bigger than zero") }

            layers_sum = layers_sum.checked_add(*el).ok_or("Problem with calculating sum of layers: i32 overflow")?;
            layers_dist_summed.push(layers_sum);
        }

        Ok(LayersDist {
            layers_num: layers_dist.len() as u8,
            max_layer_size,
            min_layer_size,
            layers_sum,
            layers_dist,
            layers_dist_summed,
        })
    }

    pub fn generate_from_params(
        layers_num: u8,
        min_layer_size: i32,
        max_layer_size: i32,
        layers_sum: Option<i32>
    ) -> Result<LayersDist, &'static str> {
        let layers = LayersDist::generate_layers_dist_vec(layers_num, min_layer_size, max_layer_size, layers_sum);
        match layers {
            Err(err) => Err(err),
            Ok(layers) => {
                let mut layers_summed:Vec<i32> = Vec::with_capacity(layers.len());
                for (i, e) in layers.iter().enumerate(){
                    if i != 0 {
                        layers_summed.push(layers_summed[i-1] + *e)
                    } else {
                        layers_summed.push(*e)
                    }
                }
                let layers_summed = (0..layers.len()).map(|i| if i > 0 {layers[i-1]} else {0}).collect();
                Ok(LayersDist {
                    layers_num,
                    min_layer_size,
                    max_layer_size,
                    layers_sum: layers_sum.unwrap_or(layers.clone().iter().sum()),
                    layers_dist: layers,
                    layers_dist_summed: layers_summed,
                })
            }
        }
    }

pub fn generate_layers_dist_vec(
        layers_num: u8,
        min_layer_size: i32,
        max_layer_size: i32,
        layers_sum: Option<i32>
    ) -> Result<Vec<i32>, &'static str> {
        match LayersDist::validate_params(layers_num, min_layer_size, max_layer_size, layers_sum) {
            Ok(_) => (),
            Err(err) => return Err(err),
        }

        if min_layer_size == max_layer_size {
            return Ok((0..layers_num).map(|_| max_layer_size).collect());
        }

        let mut rng = rand::thread_rng();
        let mut layers: Vec<i32> = (0..layers_num).map(|_| rng.gen_range(min_layer_size..max_layer_size)).collect();

        if layers_sum.is_none() { return Ok(layers); }
        let layers_sum = layers_sum.unwrap();

        let mut points;
        let mut znak = true;
        if layers_sum > layers.iter().sum() {
            points = layers_sum - layers.iter().sum::<i32>();
        }
        else {
            znak = false;
            points = layers.iter().sum::<i32>() - layers_sum;
        }
        let mut tries: i32 = 0;

        while points != 0 && tries <= 10000 {
            let avg_layer_mod = points / layers_num as i32;

            for layer in layers.iter_mut() {
                if znak {
                    if points + *layer <= max_layer_size {
                        *layer += points;
                        points = 0;
                        break;
                    }

                    let now_mod = if avg_layer_mod == 0 {
                        1
                    } else {
                        rng.gen_range(0..avg_layer_mod*2)
                    };

                    if *layer + now_mod <= max_layer_size {
                        *layer += now_mod;
                        points -= now_mod;
                    }
                } else {
                    if layer.checked_sub(points).unwrap_or(0) >= min_layer_size {
                        *layer -= points;
                        points = 0;
                        break;
                    }

                    let now_mod = if avg_layer_mod == 0 {
                        1
                    } else {
                        rng.gen_range(0..avg_layer_mod*2)
                    };

                    if layer.checked_sub(now_mod).unwrap_or(0) >= min_layer_size {
                        *layer -= now_mod;
                        points = points.checked_sub(now_mod).unwrap_or(0);
                    }
                }
            }
            tries += 1;
        }

        if points != 0 { 
            return Err("Something really gone wrong, this is program's fault, just try another arguments") 
        }
        Ok(layers)
    }

    // function params are named by LayersDist's first letters, e.g. ln - (l)ayers_(n)um
    fn validate_params(ln: u8, min_ls: i32, max_ls: i32, ls: Option<i32>) -> Result<(), &'static str> {
        if min_ls > max_ls {
            return Err("Max layer's size must be bigger than min layer's size")
        }

        if max_ls == 0 || ln == 0 || min_ls == 0 {
            return Err("Any argument must be bigger than zero")
        }

        if ls.is_some() {
            let ls = ls.unwrap();

            if ls == 0 {
                return Err("Any argument must be bigger than zero")
            }
            if max_ls * (ln as i32) < ls {
                return Err("Impossible: (max layer's size) * (number of layers) must be bigger than (layers sum)");
            }

            if min_ls * (ln as i32) > ls {
                return Err("Impossible: (min layer's size) * (number of layers) must be smaller than (layers sum)");
            }
        }

        Ok(())
    }
}

impl LayersDist {
    pub fn get_full_data(&self) -> (u8, i32, i32, i32, &Vec<i32>) {
        (self.layers_num, self.max_layer_size, self.min_layer_size, self.layers_sum, &self.layers_dist)
    }

    pub fn get_layers_num(&self) -> u8 {
        self.layers_num
    }

    pub fn get_layer_max_min_sizes(&self) -> (i32, i32) {
        (self.max_layer_size, self.min_layer_size)
    }

    pub fn get_layers_sum(&self) -> i32 {
        self.layers_sum
    }

    pub fn get_layers_dist(&self) -> &Vec<i32> {
        &self.layers_dist
    }

    pub fn get_layers_dist_summed(&self) -> &Vec<i32> {
        &self.layers_dist_summed
    }

    pub fn get_layers_count(&self) -> usize {
        self.layers_dist.len()
    }
}
