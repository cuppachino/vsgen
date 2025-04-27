use std::collections::HashMap;

use itertools::Itertools;
use serde_json::Value;

use crate::{
    data::{
        DotPath,
        DotPathIterator,
        DotToken,
        Grammar,
        Ingredient,
        Manifest,
        Modify,
        PatchIterator,
        Recipe,
        Remove,
        Substitution,
        Template,
    },
    error::Error,
};

pub struct DataGen<'a> {
    /// The manifest to generate recipes from.
    manifest: &'a Manifest,
    default_template: &'a Template,
}

impl<'a> DataGen<'a> {
    /// Create a new [`DataGen`] instance.
    pub fn new(manifest: &'a Manifest) -> Result<Self, Error> {
        // find a template named "default" or default to the first template.
        let default_template = manifest.templates
            .iter()
            .find(|template| template.name == "default")
            .or_else(|| manifest.templates.first())
            .ok_or_else(|| Error::MissingDefaultTemplate)?;

        Ok(Self { manifest, default_template })
    }

    /// Generate the recipes from the manifest.
    pub fn generate(&self) -> Result<Vec<Recipe>, Error> {
        let mut recipes = Vec::<Recipe>::new();

        for grammar in &self.manifest.grammars {
            match grammar.template.as_ref() {
                // If a template is specified, use it to generate the recipe.
                Some(templates) => {
                    for template in templates {
                        let template = self.manifest.find_template(template)?;
                        let generated = grammar.expand(template, &self.manifest.static_props)?;
                        recipes.extend(generated);
                    }
                }
                // If no template is specified, use the default template.
                None => {
                    let template = self.default_template;
                    let generated = grammar.expand(template, &self.manifest.static_props)?;
                    recipes.extend(generated);
                }
            }
        }

        Ok(recipes)
    }
}

impl Manifest {
    /// Find a template by its name. If the template does not exist, return an error.
    ///
    /// If the template is referred to as "default", but there isn't a template with that name,
    /// return the first template in the list.
    pub fn find_template(&self, template_key: &str) -> Result<&Template, Error> {
        self.templates
            .iter()
            .find(|template| template.name == template_key)
            .or_else(|| {
                // if the template was not found and the template was referred to as "default", return the first template.
                if template_key == "default" {
                    self.templates.first()
                } else {
                    None
                }
            })
            .ok_or_else(|| { Error::UnknownTemplate(template_key.to_string()) })
    }
}

impl Grammar {
    /// Expand the grammar into a list of recipes.
    pub fn expand(
        &self,
        template: &Template,
        static_props: &HashMap<String, Value>
    ) -> Result<Vec<Recipe>, Error> {
        let mut recipes = Vec::<Recipe>::new();

        // Generate mappings for the tags.
        let mut tags: Vec<(&str, Vec<String>)> = Vec::new();
        for tag in &self.tags {
            let mut mapped_tag_values = Vec::new();

            for value in &tag.values {
                // map anything prefixed with `@` to the corresponding static property.
                if value.starts_with('@') {
                    let static_value = value.trim_start_matches('@');
                    // If the static value is empty, return an error.
                    if static_value.is_empty() {
                        return Err(Error::InvalidStaticProperty {
                            prop: static_value.to_string(),
                            value: Value::String(value.to_string()),
                        });
                    }
                    // If the static value is not empty, check if it exists in the static properties.
                    if let Some(static_value) = static_props.get(static_value) {
                        match static_value {
                            Value::String(s) => mapped_tag_values.push(s.to_string()),
                            Value::Array(arr) => {
                                for item in arr {
                                    match item {
                                        Value::String(s) => mapped_tag_values.push(s.to_string()),
                                        _ => {
                                            return Err(Error::InvalidStaticProperty {
                                                prop: static_value.to_string(),
                                                value: item.clone(),
                                            });
                                        }
                                    }
                                }
                            }
                            // If the static value is not a string or an array, return an error.
                            _ => {
                                return Err(Error::InvalidStaticProperty {
                                    prop: static_value.to_string(),
                                    value: static_value.clone(),
                                });
                            }
                        }
                    } else {
                        // If the static value does not exist, return an error.
                        return Err(Error::UnknownStaticProperty(static_value.to_string()));
                    }
                } else {
                    // Otherwise, just push the value as is.
                    mapped_tag_values.push(value.clone());
                }
            }

            tags.push((tag.name.as_str(), mapped_tag_values));
        }

        // Create a new iterator for the patches.
        let mut patch_iter = PatchIterator::new(&tags);

        for patch in &mut patch_iter {
            println!("Patch: {:?}", patch);

            // Create a new recipe, starting from the target template, and apply the patch to it.
            let mut recipe: Value = serde_json
                ::to_value(template.recipe.clone())
                .map_err(Error::Json)?;

            // Apply removals
            for remove in &self.remove {
                remove.apply(&mut recipe)?;
            }

            // Apply modifications
            for modify in &self.modify {
                modify.apply(&mut recipe)?;
            }

            let mut recipe: Recipe = serde_json::from_value(recipe).map_err(Error::Json)?;

            // Apply substitutions
            for substitution in &patch {
                substitution.apply(&mut recipe)?;
            }

            println!("Recipe: {:#?}", recipe);

            // Finish the recipe.
            recipes.push(recipe);
        }

        Ok(recipes)
    }
}

impl<'a> Substitution<'a> {
    /// Apply the substitution to a recipe.
    pub fn apply(&self, recipe: &mut Recipe) -> Result<(), Error> {
        // Replace all instances of "%target%" with the value.
        let target = format!("%{}%", self.target);
        let target = target.as_str();
        recipe.ingredients.values_mut().for_each(|ingredient| {
            ingredient.replace_mut(target, self.value);
        });
        recipe.output.replace_mut(target, self.value);
        recipe.pattern = recipe.pattern.replace(target, self.value);
        recipe.rest.values_mut().for_each(|value| {
            value.replace_mut(target, self.value);
        });
        Ok(())
    }
}

impl Modify {
    /// Apply the modification to a JSON value.
    pub fn apply(&self, value: &mut Value) -> Result<(), Error> {
        let mut current = value;
        let mut tokens = DotPathIterator::new(&self.path).peekable();

        while let Some(token) = tokens.next() {
            match token {
                DotToken::Property(prop) => {
                    // Check if we're at the last token.
                    if tokens.peek().is_none() {
                        if let Value::Object(obj) = current {
                            obj.insert(prop.to_string(), self.value.clone());
                            return Ok(());
                        } else {
                            return Err(Error::ExpectedObjectToSetProperty {
                                path: self.path.to_string(),
                                prop: prop.to_string(),
                            });
                        }
                    } else {
                        // If we're not at the last token, navigate to the next token.
                        if let Value::Object(obj) = current {
                            current = obj.get_mut(prop).ok_or_else(|| {
                                Error::UnknownPropertyInObjectPath {
                                    path: self.path.to_string(),
                                    prop: prop.to_string(),
                                }
                            })?;
                        } else {
                            return Err(Error::ExpectedObjectToSetProperty {
                                path: self.path.to_string(),
                                prop: prop.to_string(),
                            });
                        }
                    }
                }
                DotToken::Index(index) => {
                    if tokens.peek().is_none() {
                        // If we're at the last token, set the value.
                        if let Value::Array(arr) = current {
                            if index >= arr.len() {
                                return Err(Error::IndexOutOfBounds {
                                    index,
                                    len: arr.len(),
                                    path: self.path.to_string(),
                                });
                            }
                            arr[index] = self.value.clone();
                            return Ok(());
                        } else {
                            return Err(Error::ExpectedArrayToSetIndex {
                                index,
                                path: self.path.to_string(),
                            });
                        }
                    } else {
                        // If we're not at the last token, navigate to the next token.
                        if let Value::Array(arr) = current {
                            if index >= arr.len() {
                                return Err(Error::IndexOutOfBounds {
                                    index,
                                    len: arr.len(),
                                    path: self.path.to_string(),
                                });
                            }
                            current = &mut arr[index];
                        } else {
                            return Err(Error::ExpectedArrayToSetIndex {
                                index,
                                path: self.path.to_string(),
                            });
                        }
                    }
                }
                DotToken::Wildcard => {
                    match current {
                        Value::Object(obj) => {
                            for v in obj.values_mut() {
                                let sub_path = DotPath(tokens.clone().join("."));
                                let sub_modify = Modify {
                                    path: sub_path,
                                    value: self.value.clone(),
                                };
                                sub_modify.apply(v)?;
                            }
                        }
                        Value::Array(arr) => {
                            let len = arr.len();
                            for (i, v) in arr.iter_mut().enumerate() {
                                if i < len {
                                    let sub_path = DotPath(tokens.clone().join("."));
                                    let sub_modify = Modify {
                                        path: sub_path,
                                        value: self.value.clone(),
                                    };
                                    sub_modify.apply(v)?;
                                } else {
                                    return Err(Error::IndexOutOfBounds {
                                        path: self.path.to_string(),
                                        index: i,
                                        len: arr.len(),
                                    });
                                }
                            }
                        }
                        _ => {
                            return Err(Error::ExpectedWildcardToSetProperty {
                                path: self.path.to_string(),
                                value: current.clone(),
                            });
                        }
                    }

                    return Ok(());
                }
            }
        }

        Ok(())
    }
}

impl Remove {
    /// Apply the removal to a JSON value.
    pub fn apply(&self, value: &mut Value) -> Result<(), Error> {
        let mut current = value;
        let mut tokens = DotPathIterator::new(&self.0).peekable();

        while let Some(token) = tokens.next() {
            match token {
                DotToken::Property(prop) => {
                    if tokens.peek().is_none() {
                        if let Value::Object(obj) = current {
                            obj.remove(prop);
                            return Ok(());
                        } else {
                            return Err(Error::ExpectedObjectToRemoveProperty {
                                path: self.0.to_string(),
                                prop: prop.to_string(),
                            });
                        }
                    } else {
                        if let Value::Object(obj) = current {
                            current = obj.get_mut(prop).ok_or_else(|| {
                                Error::UnknownPropertyInObjectPath {
                                    path: self.0.to_string(),
                                    prop: prop.to_string(),
                                }
                            })?;
                        } else {
                            return Err(Error::ExpectedObjectToRemoveProperty {
                                path: self.0.to_string(),
                                prop: prop.to_string(),
                            });
                        }
                    }
                }
                DotToken::Index(index) => {
                    if tokens.peek().is_none() {
                        if let Value::Array(arr) = current {
                            if index >= arr.len() {
                                return Err(Error::IndexOutOfBounds {
                                    index,
                                    len: arr.len(),
                                    path: self.0.to_string(),
                                });
                            }
                            arr.remove(index);
                            return Ok(());
                        } else {
                            return Err(Error::ExpectedArrayToRemoveIndex {
                                path: self.0.to_string(),
                                index,
                            });
                        }
                    } else {
                        if let Value::Array(arr) = current {
                            if index >= arr.len() {
                                return Err(Error::IndexOutOfBounds {
                                    index,
                                    len: arr.len(),
                                    path: self.0.to_string(),
                                });
                            }
                            current = &mut arr[index];
                        } else {
                            return Err(Error::ExpectedArrayToRemoveIndex {
                                path: self.0.to_string(),
                                index,
                            });
                        }
                    }
                }
                DotToken::Wildcard => {
                    match current {
                        Value::Object(obj) => {
                            for v in obj.values_mut() {
                                let sub_path = DotPath(tokens.clone().join("."));
                                let sub_remove = Remove(sub_path);
                                sub_remove.apply(v)?;
                            }
                        }
                        Value::Array(arr) => {
                            for v in arr.iter_mut() {
                                let sub_path = DotPath(tokens.clone().join("."));
                                let sub_remove = Remove(sub_path);
                                sub_remove.apply(v)?;
                            }
                        }
                        _ => {
                            return Err(Error::ExpectedWildcardToRemoveProperty {
                                path: self.0.to_string(),
                                value: current.clone(),
                            });
                        }
                    }

                    return Ok(());
                }
            }
        }

        Ok(())
    }
}

pub trait ReplaceMut {
    fn replace_mut(&mut self, target: &str, value: &str);
}

impl ReplaceMut for Value {
    fn replace_mut(&mut self, target: &str, replacement: &str) {
        match self {
            Value::String(s) => {
                *s = s.replace(target, replacement);
            }
            Value::Array(arr) => {
                for a in arr {
                    a.replace_mut(target, replacement);
                }
            }
            Value::Object(obj) => {
                for (_, v) in obj.iter_mut() {
                    v.replace_mut(target, replacement);
                }
            }
            _ => {}
        }
    }
}

impl ReplaceMut for Ingredient {
    /// Replace all instances of `target` with `value` in the ingredient.
    fn replace_mut(&mut self, target: &str, value: &str) {
        self.code = self.code.replace(target, value);
        self.item_type = self.item_type.replace(target, value);
        if let Some(name) = &mut self.name {
            *name = name.replace(target, value);
        }
        for variant in &mut self.skip_variants {
            *variant = variant.replace(target, value);
        }
        for (_, v) in &mut self.rest {
            v.replace_mut(target, value);
        }
    }
}
