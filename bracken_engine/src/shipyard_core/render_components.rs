//===============================================================

use brackens_tools::renderer::render_tools;
use shipyard::Unique;

//===============================================================

#[derive(Unique)]
pub struct RenderPassTools(pub(crate) render_tools::RenderPassTools);

#[derive(Unique)]
pub struct ClearColor(pub [f64; 3]);

//===============================================================
