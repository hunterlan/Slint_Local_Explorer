use rfd::FileDialog;

pub fn file_pick() -> Option<String> {
    let folder = FileDialog::new()
        .pick_folder();

    match folder {
        Some(folder_data) => {
            Some(folder_data.to_str().unwrap().to_string())
        },
        None => {None}
    }
}