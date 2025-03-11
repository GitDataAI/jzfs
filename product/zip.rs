use std::fs;
use std::io::{Read, Write};
use std::path::Path;
use zip::write::{ExtendedFileOptions, FileOptions};
use zip::{CompressionMethod, ZipWriter};

pub fn compress_directory_to_zip<P: AsRef<Path>, Q: AsRef<Path>>(
    dir_path: P,
    zip_path: Q,
) -> std::io::Result<()> {
    let dir_path = dir_path.as_ref();
    let zip_path = zip_path.as_ref();

    let file = fs::File::create(zip_path)?;
    let mut zip = ZipWriter::new(file);

    let options:FileOptions<ExtendedFileOptions> = FileOptions::default()
        .compression_method(CompressionMethod::Deflated)
        .unix_permissions(0o755);

    for entry in fs::read_dir(dir_path)? {
        let entry = entry?;
        let path = entry.path();
        let metadata = fs::metadata(&path)?;

        if metadata.is_file() {
            let mut file = fs::File::open(&path)?;
            let mut buffer = Vec::new();
            file.read_to_end(&mut buffer)?;

            zip.start_file(
                path.strip_prefix(dir_path)
                    .unwrap()
                    .to_string_lossy()
                    .into_owned(),
                options.clone(),
            )?;
            zip.write_all(&buffer)?;
        } else if metadata.is_dir() {
            compress_directory(&path, &mut zip, &dir_path, &options)?;
        }
    }

    zip.finish()?;
    Ok(())
}

fn compress_directory(
    dir_path: &Path,
    zip: &mut ZipWriter<fs::File>,
    base_dir: &Path,
    options: &FileOptions<ExtendedFileOptions>,
) -> std::io::Result<()> {
    for entry in fs::read_dir(dir_path)? {
        let entry = entry?;
        let path = entry.path();
        let metadata = fs::metadata(&path)?;
        if metadata.is_file() {
            let mut file = fs::File::open(&path)?;
            let mut buffer = Vec::new();
            file.read_to_end(&mut buffer)?;

            zip.start_file(
                path.strip_prefix(base_dir)
                    .unwrap()
                    .to_string_lossy()
                    .into_owned(),
                options.clone(),
            )?;
            zip.write_all(&buffer)?;
        } else if metadata.is_dir() {
            compress_directory(&path, zip, base_dir, options)?;
        }
    }

    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_compress_directory_to_zip() {
        let dir_path = "./data/static";
        let zip_path = "./data/test.zip";

        compress_directory_to_zip(dir_path, zip_path).unwrap();

        assert!(Path::new(zip_path).exists());
    }

}
