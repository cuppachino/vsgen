use std::collections::HashMap;
use derive_more::Display;
use serde::{ Deserialize, Serialize };

#[derive(Serialize, Deserialize, Debug)]
pub struct Manifest {
    /// The output file name.
    pub output: String,
    /// Static properties that can be referenced through tags in template recipes.
    #[serde(rename = "static", default)]
    pub static_props: HashMap<String, serde_json::Value>,
    /// A list of templates each representing the Vintage Story recipe structure.
    pub templates: Vec<Template>,
    /// A list of grammars to apply to the template recipe to generate final recipes. Each grammar
    /// has the potential to create multiple recipes representing variants of the same recipe.
    pub grammars: Vec<Grammar>,
}

/// A template recipe that can be used to generate multiple variants of a recipe using short-hand
/// syntax and tags.
#[derive(Serialize, Deserialize, Debug)]
pub struct Template {
    pub name: String,
    #[serde(flatten)]
    pub recipe: Recipe,
}

/// A crafting recipe for Vintage Story.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Recipe {
    /// The ingredient pattern for the recipe.
    #[serde(rename = "ingredientPattern")]
    pub pattern: String,
    pub ingredients: HashMap<char, Ingredient>,
    pub width: u8,
    pub height: u8,
    pub output: Ingredient,
    #[serde(flatten)]
    pub rest: HashMap<String, serde_json::Value>,
}

/// A grammar is a set of rules that can be applied to a template recipe to generate multiple
/// variants of the same recipe.
#[derive(Serialize, Deserialize, Debug)]
pub struct Grammar {
    /// The name(s) of the [`Template`] to apply the grammar to.
    pub template: Option<OneOrMany<String>>,
    /// Tags mapping tag names to their values.
    pub tags: Vec<Tag>,
    /// Properties to remove from the recipe.
    #[serde(default)]
    pub remove: Vec<Remove>,
    /// Properties to create or replace in the recipe.
    #[serde(default)]
    pub modify: Vec<Modify>,
    #[serde(flatten)]
    pub rest: HashMap<String, serde_json::Value>,
}

/// An ingredient in a recipe.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Ingredient {
    #[serde(rename = "type")]
    pub item_type: String,
    pub code: String,
    pub name: Option<String>,
    #[serde(default)]
    pub skip_variants: Vec<String>,
    #[serde(flatten)]
    pub rest: HashMap<String, serde_json::Value>,
}

/// Maps a tag name to its values in a recipe.
#[derive(Serialize, Deserialize, Debug)]
pub struct Tag {
    /// The name of the tag.
    pub name: String,
    /// The value of the tag.
    pub values: Vec<String>,
}

/// A path to a property in the recipe to remove.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Remove(pub DotPath);

/// A modification to a property in the recipe.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Modify {
    /// The path to the property.
    pub path: DotPath,
    /// The new value for the property.
    pub value: serde_json::Value,
}

/// A path to a property in the data structure.
/// - `.` is used to separate nested properties.
/// - `*` is used to match any property at that level.
/// - or indexes bro.
#[derive(Serialize, Deserialize, Debug, Display, Clone)]
pub struct DotPath(pub String);

impl DotPath {
    pub fn tokenize(&self) -> Vec<DotToken> {
        self.0
            .split('.')
            .map(|s| {
                match s {
                    "*" => DotToken::Wildcard,
                    _ => if let Ok(index) = s.parse::<usize>() {
                        DotToken::Index(index)
                    } else {
                        DotToken::Property(s)
                    }
                }
            })
            .collect()
    }
}

#[derive(Clone)]
pub struct DotPathIterator<'a> {
    tokens: Vec<DotToken<'a>>,
    index: usize,
}

impl<'a> DotPathIterator<'a> {
    pub fn new(dot_path: &'a DotPath) -> Self {
        Self { tokens: dot_path.tokenize(), index: 0 }
    }
}

impl<'a> Iterator for DotPathIterator<'a> {
    type Item = DotToken<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.tokens.len() {
            let token = self.tokens[self.index].clone();
            self.index += 1;
            Some(token)
        } else {
            None
        }
    }
}

impl std::fmt::Display for DotPathIterator<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, token) in self.tokens.iter().enumerate() {
            if i > 0 {
                write!(f, ".")?;
            }
            write!(f, "{}", token)?;
        }
        Ok(())
    }
}

/// The type of token in a [DotPath].
#[derive(Clone, Debug)]
pub enum DotToken<'a> {
    /// A property name in the data structure.
    Property(&'a str),
    /// A wildcard that matches any property at that level.
    Wildcard,
    /// An array index.
    Index(usize),
}

impl std::fmt::Display for DotToken<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DotToken::Property(name) => write!(f, "{}", name),
            DotToken::Wildcard => write!(f, "*"),
            DotToken::Index(index) => write!(f, "{}", index),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum OneOrMany<T> {
    /// A single value.
    One(T),
    /// A list of values.
    Many(Vec<T>),
}

impl<T> OneOrMany<T> {
    /// Create an iterator over the values in the `OneOrMany`.
    pub fn iter(&self) -> OneOrManyIterator<T> {
        OneOrManyIterator { inner: self, index: 0 }
    }
}

impl<'a, T> IntoIterator for &'a OneOrMany<T> {
    type Item = &'a T;
    type IntoIter = OneOrManyIterator<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

/// An iterator over the values in a `OneOrMany`.
pub struct OneOrManyIterator<'a, T> {
    inner: &'a OneOrMany<T>,
    index: usize,
}

impl<'a, T> Iterator for OneOrManyIterator<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        match self.inner {
            OneOrMany::One(value) => {
                if self.index == 0 {
                    self.index += 1;
                    Some(value)
                } else {
                    None
                }
            }
            OneOrMany::Many(values) => {
                if self.index < values.len() {
                    let value = &values[self.index];
                    self.index += 1;
                    Some(value)
                } else {
                    None
                }
            }
        }
    }
}

/// A substitution is a mapping of a tag to its value.
/// The `target` is the string to be replaced, and the `value` is the value to replace it with.
#[derive(Debug)]
pub struct Substitution<'a> {
    pub target: &'a str,
    pub value: &'a str,
}

/// A patch is a list of substitutions to be applied to a recipe.
/// Each substitution is a mapping of a tag to its current-iteration value.
pub type Patch<'a> = Vec<Substitution<'a>>;

/// An iterator over all possible patches for a set of tags.
/// Each patch is a combination of the values for each tag.
pub struct PatchIterator<'a> {
    tags: &'a [(&'a str, Vec<String>)],
    indices: Vec<usize>,
    is_done: bool,
}

impl<'a> PatchIterator<'a> {
    /// Create a new `PatchIterator` for the given tags.
    pub fn new(tags: &'a [(&'a str, Vec<String>)]) -> Self {
        let indices = vec![0; tags.len()];
        Self { tags, indices, is_done: false }
    }

    fn increment_indices(&mut self) -> bool {
        for i in (0..self.indices.len()).rev() {
            self.indices[i] += 1;
            // Stop if the index is within bounds.
            if self.indices[i] < self.tags[i].1.len() {
                return true;
            } else {
                // Otherwise, reset and move to the next index.
                self.indices[i] = 0;
            }
        }
        false
    }
}

impl<'a> Iterator for PatchIterator<'a> {
    type Item = Patch<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.is_done {
            return None;
        }

        // Build the patch for the current iteration.
        let patch: Patch = self.tags
            .iter()
            .zip(self.indices.iter())
            .map(|((target, variants), &index)| Substitution {
                target,
                value: &variants[index],
            })
            .collect();

        if !self.increment_indices() {
            self.is_done = true;
        }

        Some(patch)
    }
}
