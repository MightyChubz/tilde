#[cfg(test)]
mod tests {
    use sdl2::pixels::Color;

    #[test]
    fn it_works() {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();

        let window = video_subsystem
            .window("rust-sdl2 demo", 800, 600)
            .position_centered()
            .build()
            .unwrap();
        let mut canvas = window.into_canvas().build().unwrap();

        canvas.set_draw_color(Color::RGB(0, 255, 255));
        canvas.clear();
        canvas.present();
        std::thread::sleep(std::time::Duration::new(0, 1_000_000_000u32 / 60));
    }
}
