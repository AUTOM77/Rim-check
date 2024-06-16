use kdam::{tqdm, BarExt, Column, RichProgress, Spinner};

fn move_files(files: Vec<String>, progress_bar: &mut RichProgress) -> std::io::Result<()> {
    for file in files {
        let path = std::path::Path::new(&file).with_extension("parquet");
        if path.exists() {
            std::fs::remove_file(&path)?;
        }
        let _ = progress_bar.update(1);
    }
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let raw: toml::Value = std::fs::read_to_string("config.toml")?.parse()?;
    let files: Vec<String> = raw["checked_file"].as_array()
        .expect("checked_file should be an array").iter()
        .flat_map(|group| group.as_array()
            .expect("Each file vec should be an array")
            .iter()
            .map(|v| v.as_str().expect("Each file path should be a string").to_string())
        )
        .collect();

    let total_files: usize = files.len();

    let mut progress_bar = RichProgress::new(
        tqdm!(total = total_files),
        vec![
            Column::Spinner(Spinner::new(
                &["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"],
                80.0,
                1.0,
            )),
            Column::Text("[bold blue]Removing".to_owned()),
            Column::Animation,
            Column::Percentage(1),
            Column::Text("•".to_owned()),
            Column::CountTotal,
            Column::Text("•".to_owned()),
            Column::Rate,
            Column::Text("•".to_owned()),
            Column::RemainingTime,
        ],
    );

    progress_bar.write("Starting file removal")?;

    move_files(files, &mut progress_bar)?;
    progress_bar.write("File removal complete")?;
    Ok(())
}
