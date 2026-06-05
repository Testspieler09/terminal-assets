use ta_render_engine::models::Dimensions;

pub(crate) mod ctr;
pub(crate) mod t09;

pub(crate) trait Image {
    fn get_content_name() -> String;
    fn get_dimensions() -> Dimensions;
    fn build_buffer();
}

pub(crate) trait Video {
    fn get_content_name() -> String;
    fn get_dimensions() -> Dimensions;
    fn get_num_frames() -> usize;
    fn get_frame_rate() -> usize;
    fn build_buffer(frame: usize);
}
