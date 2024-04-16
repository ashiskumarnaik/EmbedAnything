use std::{collections::HashSet, io::Error, path::PathBuf};

use regex::Regex;
use walkdir::WalkDir;

pub struct FileParser {
    pub files: Vec<String>,
}

impl Default for FileParser {
    fn default() -> Self {
        Self::new()
    }
}

impl FileParser {
    pub fn new() -> Self {
        Self { files: Vec::new() }
    }

    pub fn get_pdf_files(&mut self, directory_path: &PathBuf) -> Result<Vec<String>, Error> {
        let pdf_extension_regex = Regex::new(r#"\.pdf$"#).unwrap();

        let pdf_files: Vec<String> = WalkDir::new(directory_path)
            .into_iter()
            .filter_map(|entry| entry.ok())
            .filter(|entry| entry.file_type().is_file())
            .filter(|entry| pdf_extension_regex.is_match(entry.file_name().to_str().unwrap_or("")))
            .map(|entry| {
                let absolute_path = entry
                    .path()
                    .canonicalize()
                    .unwrap_or_else(|_| entry.path().to_path_buf());
                absolute_path.to_string_lossy().to_string()
            })
            .collect();
        self.files = pdf_files;

        Ok(self.files.clone())
    }

    pub fn get_image_paths(&mut self, directory_path: &PathBuf) -> Result<Vec<String>, Error> {
        let image_regex = Regex::new(r".*\.(png|jpg|jpeg|gif|bmp|tiff|webp)$").unwrap();

        let image_paths: Vec<String> = WalkDir::new(directory_path)
            .into_iter()
            .filter_map(|entry| entry.ok())
            .filter(|entry| entry.file_type().is_file())
            .filter(|entry| image_regex.is_match(entry.file_name().to_str().unwrap_or("")))
            .map(|entry| {
                let absolute_path = entry
                    .path()
                    .canonicalize()
                    .unwrap_or_else(|_| entry.path().to_path_buf());
                absolute_path.to_string_lossy().to_string()
            })
            .collect();

        self.files = image_paths;
        Ok(self.files.clone())
    }

    pub fn get_files_to_index(&self, indexed_files: &HashSet<String>) -> Vec<String> {
        let files = self
            .files
            .iter()
            .filter(|file| !indexed_files.contains(*file))
            .map(|f| f.to_string())
            .collect::<Vec<_>>();
        files
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use tempdir::TempDir;

    #[test]
    fn test_get_pdf_files() {
        let temp_dir = TempDir::new("example").unwrap();
        let pdf_file = temp_dir.path().join("test.pdf");
        let _image_file = temp_dir.path().join("image.jpg");

        File::create(&pdf_file).unwrap();

        let mut file_parser = FileParser::new();
        let pdf_files = file_parser
            .get_pdf_files(&PathBuf::from(temp_dir.path()))
            .unwrap();
        assert_eq!(pdf_files.len(), 1);
        assert_eq!(pdf_files[0], pdf_file.canonicalize().unwrap().to_string_lossy().to_string());
    }

    #[test]
    fn test_get_image_paths() {
        let temp_dir = TempDir::new("example").unwrap();
        let image_file = temp_dir.path().join("image.jpg");
        let _pdf_file = temp_dir.path().join("test.pdf");
        File::create(&image_file).unwrap();

        let mut file_parser = FileParser::new();
        let image_files = file_parser
            .get_image_paths(&PathBuf::from(temp_dir.path()))
            .unwrap();
        assert_eq!(image_files.len(), 1);
        assert_eq!(image_files[0], image_file.canonicalize().unwrap().to_string_lossy().to_string());
    }

    #[test]
    fn test_get_files_to_index() {
        let temp_dir = TempDir::new("example").unwrap();
        let pdf_file = temp_dir.path().join("test.pdf");
        let image_file = temp_dir.path().join("test.jpg");
        File::create(&pdf_file).unwrap();
        File::create(&image_file).unwrap();

        let mut file_parser = FileParser::new();
        file_parser
            .get_pdf_files(&PathBuf::from(temp_dir.path()))
            .unwrap();
        file_parser
            .get_image_paths(&PathBuf::from(temp_dir.path()))
            .unwrap();

        let indexed_files = vec![pdf_file.to_string_lossy().to_string()];
        let files_to_index = file_parser.get_files_to_index(&indexed_files.into_iter().collect());
        assert_eq!(files_to_index.len(), 1);
        assert_eq!(files_to_index[0], image_file.canonicalize().unwrap().to_string_lossy().to_string());
    }
}
