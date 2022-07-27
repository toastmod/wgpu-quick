use winit::event_loop;


/// Starts wgpu either with `pollster::block_on` or by adding a canvas element in WASM.
/// (Specifying canvas name coming soon)
#[macro_export]
macro_rules! start_wgpu {
    ($window:expr, $event_loop:expr, $runfn:expr) => {
    #[cfg(not(target_arch = "wasm32"))]
    {
        // Temporarily avoid srgb formats for the swapchain on the web
        pollster::block_on($runfn($event_loop, $window));
    }
    #[cfg(target_arch = "wasm32")]
    {
        std::panic::set_hook(Box::new(console_error_panic_hook::hook));
        console_log::init().expect("could not initialize logger");
        use winit::platform::web::WindowExtWebSys;
        // On wasm, append the canvas to the document body
        web_sys::window()
            .and_then(|win| win.document())
            .and_then(|doc| doc.body())
            .and_then(|body| {
                body.append_child(&web_sys::Element::from(window.canvas()))
                    .ok()
            })
            .expect("couldn't append canvas to document body");
        wasm_bindgen_futures::spawn_local($runfn($event_loop, $window));
    }
    };
}

pub use start_wgpu;


// pub fn start_wgpu<T>(window: &winit::window::Window, event_loop: &winit::event_loop::EventLoop<T>, run: &mut dyn FnMut(winit::window::Window, winit::event_loop::EventLoop<T>)) {

// }