use std::collections::HashMap;

use ta_render_engine::Scene;

pub(crate) mod ctr;
pub(crate) mod t09;

pub fn all_scenes() -> HashMap<String, Box<dyn Scene>> {
    let scenes: Vec<Box<dyn Scene>> = vec![Box::new(ctr::CtrScene), Box::new(t09::T09Scene)];
    scenes
        .into_iter()
        .map(|s| (s.name().to_string(), s))
        .collect()
}
