use flac::metadata::get_vorbis_comment;
use std::{ffi::OsStr, path::PathBuf};
use structopt::StructOpt;

#[derive(StructOpt)]
struct Opts {
    /// Files to process
    #[structopt(name = "FILE", parse(from_os_str))]
    files: Vec<PathBuf>,
}

fn main() {
    let opts = Opts::from_args();

    for filename in opts.files {
        if filename.extension() != Some(OsStr::new("flac")) {
            continue;
        }

        let metadata = get_vorbis_comment(filename.to_str().unwrap()).unwrap();

        let artist = metadata.comments.get("ARTIST");
        let title = metadata.comments.get("TITLE");

        match artist.zip(title) {
            Some((artist, title)) => {
                let mut rename = filename.clone();
                rename.set_file_name(format!("{} - {}.flac", artist, title));
                if filename == rename {
                    continue;
                }

                println!(
                    "Renaming {} to {}",
                    filename.file_name().unwrap().to_string_lossy(),
                    rename.file_name().unwrap().to_string_lossy()
                );
                std::fs::rename(filename, rename).unwrap();
            }
            None => {
                println!(
                    "{} did not have an artist or title",
                    filename.to_string_lossy()
                );
            }
        }
    }
}
