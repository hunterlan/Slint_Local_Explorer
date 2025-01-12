use crate::utils::file_pick_handler::file_pick;

mod utils;

slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    let main_window = MainWindow::new()?;

    main_window.global::<FilePicker>().on_pick(|| {
        let picked_path = file_pick();
        match picked_path {
            Some(picked_path) => {
                picked_path.into()
            }
            None => {
                "Path is not exists or access denied.".into()
            }
        }
    });

    main_window.run()
}