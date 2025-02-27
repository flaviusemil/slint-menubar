use std::path::PathBuf;
use rfd::FileDialog;

slint::include_modules!();

fn select_file() -> Option<PathBuf> {
    if let Some(file) = FileDialog::new()
        .pick_file() {
        Some(file)
    } else { None }
}

fn main() {
    let ui = MainWindow::new().unwrap();

    ui.on_menu_item_clicked(|menu_item|{
        match menu_item.as_str() {
            "Open" => {
                let file_path = select_file();
                println!("{:?} file selected!", file_path);
            }
            &_ => {
                println!("{} was clicked!", menu_item);
            }
        }
    });

    ui.run().unwrap();
}
