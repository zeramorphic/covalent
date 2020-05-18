use crate::render_backend::RenderBackend;
use crate::display_hints::DisplayHints;
use crate::renderer::Renderer;

/// Construct a Covalent context from the given backend, then executes the application defined by this Covalent context.
/// Only create a single context during the lifetime of your application,
/// and only create this context on the main thread!
pub fn execute(hints: DisplayHints, rb: impl RenderBackend) {
    let r = Renderer {};
    rb.main_loop(hints, r);
}