pub mod random_border;
pub mod model_generation;
pub mod export_borders_axis_params;
pub mod export_model_mask;

// Default difficulty is max. You can increase value, but this willn't make next generation harder.

const RANDOM_BORDER_DIFFICULTY: usize = 10; // Default: 10
const RANDOM_BORDER_TESTS: usize = 3; // Default: 3

const MODEL_GENERATION_DIFFICULTY: usize = 4; // Default: 4
const MODEL_GENERATION_TESTS: usize = 3; // Default: 3

const EXPORT_BORDERS_AXIS_PARAMS_DIFFICULTY: usize = 6; // Default: 6
const EXPORT_BORDERS_AXIS_PARAMS_TESTS: usize = 3; // Default: 3

const EXPORT_MODEL_MASK_DIFFICULTY: usize = 4; // Default: 4
const EXPORT_MODEL_MASK_TESTS: usize = 3; // Default: 3

fn main() {
    println!("Running benchmarks...");
    let random_border_dur = random_border::bench(RANDOM_BORDER_DIFFICULTY, RANDOM_BORDER_TESTS);

    let (model_gen_all, model_gen_full, model_gen_mask, model_gen_model) = 
        model_generation::bench(MODEL_GENERATION_DIFFICULTY, MODEL_GENERATION_TESTS);

    let (export_bap_all, export_bap_num, export_bap_scale) = 
        export_borders_axis_params::bench(EXPORT_BORDERS_AXIS_PARAMS_DIFFICULTY, EXPORT_BORDERS_AXIS_PARAMS_TESTS);

    let (export_mm_all, export_mm_model, export_mm_mask) =
        export_model_mask::bench(EXPORT_MODEL_MASK_DIFFICULTY, EXPORT_MODEL_MASK_TESTS);

    println!("\n\nRandom border average duration
        difficulty: {RANDOM_BORDER_DIFFICULTY}, tests: {RANDOM_BORDER_TESTS}:
        All time: {:.2?}\n\n", random_border_dur);

    println!("Model generation average duration
        difficulty: {MODEL_GENERATION_DIFFICULTY}, tests: {MODEL_GENERATION_TESTS}:
        All time: {:.2?}, Full model: {:.2?}, : Mask only: {:.2?}, Model only: {:.2?}\n\n",
        model_gen_all, model_gen_full, model_gen_mask, model_gen_model);

    println!("Export borders, axis, params average duration
        difficulty: {EXPORT_BORDERS_AXIS_PARAMS_DIFFICULTY}, tests: {EXPORT_BORDERS_AXIS_PARAMS_TESTS}:
        All time: {:.2?}, Num axis: {:.2?}, Scale axis: {:.2?}\n\n",
        export_bap_all, export_bap_num, export_bap_scale);

    println!("Export model, mask average duration
        difficulty: {EXPORT_MODEL_MASK_DIFFICULTY}, tests: {EXPORT_MODEL_MASK_TESTS}:
        All time: {:.2?}, Model: {:.2?}, Mask: {:.2?}\n\n",
        export_mm_all, export_mm_model, export_mm_mask);
}
