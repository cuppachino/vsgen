# vsgen

**vsgen** is a CLI tool for generating [Vintage Story](https://www.vintagestory.at/) recipes from customizable grammars and recipe templates.

---

## Quickstart

1. Download the latest release from the [Releases page](https://github.com/cuppachino/vsgen/releases).
2. Run `vsgen` to generate recipes in bulk.

```bash
./vsgen --help
```

---

## Example

Given the **static** variants, `glass` and `metal`:

<details>
  <summary>shovel.json::static</summary>

```json
"static": {
    "glass": [
        "clear",
        "green"
    ],
    "metal": [
        "copper",
        "gold",
        "silver"
    ]
}
```

</details>

And the following **grammars**, `"default"` (implied) and `"simple"`, using `@` to reference static props, and `*` as a wildcard:

<details>
  <summary>shovel.json::grammars</summary>

```json
"grammars": [
    {
        "tags": [
            {
                "name": "metal",
                "values": [
                    "@metal"
                ]
            },
            {
                "name": "glass",
                "values": [
                    "@glass"
                ]
            }
        ]
    },
    {
        "name": "simple",
        "tags": [],
        "remove": [
            "output.attributes"
        ],
        "modify": [
            {
                "path": "ingredients.M.code",
                "value": "metalnailsandstrips-*"
            }
        ]
    }
],
```

</details>

And a **template**:

<details>
  <summary>shovel.json::template</summary>

```json
"templates": [
    {
        "name": "default",
        "ingredientPattern": "S_,MH,T_",
        "ingredients": {
            "T": {
                "type": "item",
                "code": "shovelhead-*",
                "name": "material",
                "skipVariants": [
                    "chert",
                    "granite",
                    "andesite",
                    "basalt",
                    "obsidian",
                    "peridotite",
                    "flint"
                ]
            },
            "M": {
                "type": "item",
                "code": "metalnailsandstrips-%metal%-%glass%"
            },
            "H": {
                "type": "item",
                "code": "hammer-*",
                "isTool": true,
                "toolDurabilityCost": 10
            },
            "S": {
                "type": "item",
                "code": "stick"
            }
        },
        "copyAttributesFrom": "S",
        "width": 2,
        "height": 3,
        "output": {
            "type": "item",
            "code": "shovel-fine-{material}",
            "quantity": 1,
            "attributes": {
                "stripsTexture": "game:block/metal/ingot/%metal%"
            }
        }
    }
]
```

</details>

Running `vsgen` produces:

<details>
  <summary>shovel.gen.json</summary>

```json
[
    {
        "ingredientPattern": "S_,MH,T_",
        "ingredients": {
            "S": {
                "type": "item",
                "code": "stick",
                "name": null,
                "skipVariants": []
            },
            "M": {
                "type": "item",
                "code": "metalnailsandstrips-copper-clear",
                "name": null,
                "skipVariants": []
            },
            "H": {
                "type": "item",
                "code": "hammer-*",
                "name": null,
                "skipVariants": [],
                "toolDurabilityCost": 10,
                "isTool": true
            },
            "T": {
                "type": "item",
                "code": "shovelhead-*",
                "name": "material",
                "skipVariants": [
                    "chert",
                    "granite",
                    "andesite",
                    "basalt",
                    "obsidian",
                    "peridotite",
                    "flint"
                ]
            }
        },
        "width": 2,
        "height": 3,
        "output": {
            "type": "item",
            "code": "shovel-fine-{material}",
            "name": null,
            "skipVariants": [],
            "attributes": {
                "stripsTexture": "game:block/metal/ingot/copper"
            },
            "quantity": 1
        },
        "copyAttributesFrom": "S"
    },
    {
        "ingredientPattern": "S_,MH,T_",
        "ingredients": {
            "T": {
                "type": "item",
                "code": "shovelhead-*",
                "name": "material",
                "skipVariants": [
                    "chert",
                    "granite",
                    "andesite",
                    "basalt",
                    "obsidian",
                    "peridotite",
                    "flint"
                ]
            },
            "S": {
                "type": "item",
                "code": "stick",
                "name": null,
                "skipVariants": []
            },
            "M": {
                "type": "item",
                "code": "metalnailsandstrips-copper-green",
                "name": null,
                "skipVariants": []
            },
            "H": {
                "type": "item",
                "code": "hammer-*",
                "name": null,
                "skipVariants": [],
                "toolDurabilityCost": 10,
                "isTool": true
            }
        },
        "width": 2,
        "height": 3,
        "output": {
            "type": "item",
            "code": "shovel-fine-{material}",
            "name": null,
            "skipVariants": [],
            "quantity": 1,
            "attributes": {
                "stripsTexture": "game:block/metal/ingot/copper"
            }
        },
        "copyAttributesFrom": "S"
    },
    {
        "ingredientPattern": "S_,MH,T_",
        "ingredients": {
            "S": {
                "type": "item",
                "code": "stick",
                "name": null,
                "skipVariants": []
            },
            "H": {
                "type": "item",
                "code": "hammer-*",
                "name": null,
                "skipVariants": [],
                "isTool": true,
                "toolDurabilityCost": 10
            },
            "M": {
                "type": "item",
                "code": "metalnailsandstrips-gold-clear",
                "name": null,
                "skipVariants": []
            },
            "T": {
                "type": "item",
                "code": "shovelhead-*",
                "name": "material",
                "skipVariants": [
                    "chert",
                    "granite",
                    "andesite",
                    "basalt",
                    "obsidian",
                    "peridotite",
                    "flint"
                ]
            }
        },
        "width": 2,
        "height": 3,
        "output": {
            "type": "item",
            "code": "shovel-fine-{material}",
            "name": null,
            "skipVariants": [],
            "attributes": {
                "stripsTexture": "game:block/metal/ingot/gold"
            },
            "quantity": 1
        },
        "copyAttributesFrom": "S"
    },
    {
        "ingredientPattern": "S_,MH,T_",
        "ingredients": {
            "H": {
                "type": "item",
                "code": "hammer-*",
                "name": null,
                "skipVariants": [],
                "isTool": true,
                "toolDurabilityCost": 10
            },
            "S": {
                "type": "item",
                "code": "stick",
                "name": null,
                "skipVariants": []
            },
            "T": {
                "type": "item",
                "code": "shovelhead-*",
                "name": "material",
                "skipVariants": [
                    "chert",
                    "granite",
                    "andesite",
                    "basalt",
                    "obsidian",
                    "peridotite",
                    "flint"
                ]
            },
            "M": {
                "type": "item",
                "code": "metalnailsandstrips-gold-green",
                "name": null,
                "skipVariants": []
            }
        },
        "width": 2,
        "height": 3,
        "output": {
            "type": "item",
            "code": "shovel-fine-{material}",
            "name": null,
            "skipVariants": [],
            "attributes": {
                "stripsTexture": "game:block/metal/ingot/gold"
            },
            "quantity": 1
        },
        "copyAttributesFrom": "S"
    },
    {
        "ingredientPattern": "S_,MH,T_",
        "ingredients": {
            "S": {
                "type": "item",
                "code": "stick",
                "name": null,
                "skipVariants": []
            },
            "T": {
                "type": "item",
                "code": "shovelhead-*",
                "name": "material",
                "skipVariants": [
                    "chert",
                    "granite",
                    "andesite",
                    "basalt",
                    "obsidian",
                    "peridotite",
                    "flint"
                ]
            },
            "M": {
                "type": "item",
                "code": "metalnailsandstrips-silver-clear",
                "name": null,
                "skipVariants": []
            },
            "H": {
                "type": "item",
                "code": "hammer-*",
                "name": null,
                "skipVariants": [],
                "isTool": true,
                "toolDurabilityCost": 10
            }
        },
        "width": 2,
        "height": 3,
        "output": {
            "type": "item",
            "code": "shovel-fine-{material}",
            "name": null,
            "skipVariants": [],
            "quantity": 1,
            "attributes": {
                "stripsTexture": "game:block/metal/ingot/silver"
            }
        },
        "copyAttributesFrom": "S"
    },
    {
        "ingredientPattern": "S_,MH,T_",
        "ingredients": {
            "M": {
                "type": "item",
                "code": "metalnailsandstrips-silver-green",
                "name": null,
                "skipVariants": []
            },
            "H": {
                "type": "item",
                "code": "hammer-*",
                "name": null,
                "skipVariants": [],
                "isTool": true,
                "toolDurabilityCost": 10
            },
            "S": {
                "type": "item",
                "code": "stick",
                "name": null,
                "skipVariants": []
            },
            "T": {
                "type": "item",
                "code": "shovelhead-*",
                "name": "material",
                "skipVariants": [
                    "chert",
                    "granite",
                    "andesite",
                    "basalt",
                    "obsidian",
                    "peridotite",
                    "flint"
                ]
            }
        },
        "width": 2,
        "height": 3,
        "output": {
            "type": "item",
            "code": "shovel-fine-{material}",
            "name": null,
            "skipVariants": [],
            "quantity": 1,
            "attributes": {
                "stripsTexture": "game:block/metal/ingot/silver"
            }
        },
        "copyAttributesFrom": "S"
    },
    {
        "ingredientPattern": "S_,MH,T_",
        "ingredients": {
            "H": {
                "type": "item",
                "code": "hammer-*",
                "name": null,
                "skipVariants": [],
                "isTool": true,
                "toolDurabilityCost": 10
            },
            "M": {
                "type": "item",
                "code": "metalnailsandstrips-*",
                "name": null,
                "skipVariants": []
            },
            "T": {
                "type": "item",
                "code": "shovelhead-*",
                "name": "material",
                "skipVariants": [
                    "chert",
                    "granite",
                    "andesite",
                    "basalt",
                    "obsidian",
                    "peridotite",
                    "flint"
                ]
            },
            "S": {
                "type": "item",
                "code": "stick",
                "name": null,
                "skipVariants": []
            }
        },
        "width": 2,
        "height": 3,
        "output": {
            "type": "item",
            "code": "shovel-fine-{material}",
            "name": null,
            "skipVariants": [],
            "quantity": 1
        },
        "copyAttributesFrom": "S"
    }
]
```
</details>

---

## Features

- 📚 Grammar-based input for generating many recipes at once
- 🧩 Templating system to flexibly structure output
- ⚡ Lightweight CLI workflow, perfect for automation
- 🛠️ Designed for modding Vintage Story

---

## Development

Want to build from source?

```bash
git clone https://github.com/cuppachino/vsgen.git
cd vsgen
cargo build --release
```

---

## License

MIT License.  
See [`LICENSE`](https://github.com/cuppachino/vsgen/blob/master/LICENSE) for details.
