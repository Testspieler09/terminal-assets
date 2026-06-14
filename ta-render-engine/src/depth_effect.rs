#[derive(Clone)]
pub enum DepthEffect {
    /// Recessed into the screen. Layers shift: left channels right, right channels left.
    Inset { strength: u32 },
    /// Popping out of the screen. Layers shift: left channels left, right channels right.
    PopOut { strength: u32 },
    /// Full control: provide explicit (dx, dy) per z-index.
    Custom(Vec<(i32, (i32, i32))>), // (z_index, (dx, dy))
}

impl DepthEffect {
    pub fn offset_for(&self, z: i32) -> (i32, i32) {
        match self {
            DepthEffect::Inset { strength } => {
                let s = *strength as i32;
                // negative z shifts right, positive z shifts left (recessed)
                (-z * s, 0)
            }
            DepthEffect::PopOut { strength } => {
                let s = *strength as i32;
                // negative z shifts left, positive z shifts right (popping out)
                (z * s, 0)
            }
            DepthEffect::Custom(offsets) => offsets
                .iter()
                .find(|(zi, _)| *zi == z)
                .map(|(_, offset)| *offset)
                .unwrap_or((0, 0)),
        }
    }
}
