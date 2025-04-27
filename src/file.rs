use std::{ collections::VecDeque, path::PathBuf };

use crate::{ data::Manifest, error::Error };

/// A trait that provides an iterator over the contents of a list of input files and directories.
pub trait ManifestIter<'a> {
    fn iter_manifests(self) -> ManifestIterator<'a>;
}

impl<'a> ManifestIter<'a> for &'a Vec<PathBuf> {
    fn iter_manifests(self) -> ManifestIterator<'a> {
        ManifestIterator { current: 0, paths: self, stack: Default::default() }
    }
}

/// An iterator that traverses a list of input files and directories
/// and yields the contents of each file as an [`Manifest`] object.
pub struct ManifestIterator<'a> {
    current: usize,
    paths: &'a Vec<PathBuf>,
    stack: VecDeque<PathBuf>,
}

impl Iterator for ManifestIterator<'_> {
    type Item = Result<Manifest, crate::error::Error>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            // Exit early
            if self.current >= self.paths.len() && self.stack.is_empty() {
                return None;
            }

            // Grab the next path to check
            let path = if self.stack.is_empty() {
                let path = &self.paths[self.current];
                self.current += 1;
                path
            } else {
                self.stack.front()?
            };

            // Check if the path is a file
            if path.is_file() {
                let content = Manifest::try_from(path);
                self.stack.pop_front();

                // Return the content of the file
                return Some(content);
            }

            // Check if the path is a directory
            if path.is_dir() {
                let entries = std::fs::read_dir(path).ok()?;
                self.stack.pop_front();

                for entry in entries {
                    let entry = entry.ok()?;
                    self.stack.push_back(entry.path());
                }

                // Continue to the next iteration to process the directory
                continue;
            }

            self.stack.pop_front();
        }
    }
}

impl TryFrom<&PathBuf> for Manifest {
    type Error = crate::error::Error;

    fn try_from(path: &PathBuf) -> Result<Self, Self::Error> {
        let input = std::fs::read_to_string(path).map_err(Error::Io)?;
        let input: Manifest = serde_json::from_str(&input).map_err(Error::Json)?;
        Ok(input)
    }
}
