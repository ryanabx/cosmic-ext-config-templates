# cosmic-ext-config-templates: cosmic-config template program for the COSMIC desktop!

This program will allow you to create template files that can apply COSMIC config settings en-masse! The motivation to make this program was so that I could have different panel templates and apply them at any time.

## Install

```shell
cargo install --git https://github.com/ryanabx/cosmic-ext-config-templates
```

## Usage

```shell
cosmic-ext-config-templates --help
```

### Example 1 (Generate a panel template)

```shell
cosmic-ext-config-templates generate -o ./my_panel.ron panel
```

### Example 2 (Load a template from a file)

```shell
cosmic-ext-config-templates load-file ./my_panel.ron
```

### Example 3 (Load a premade Ubuntu panel template)

```shell
cosmic-ext-config-templates load-template ubuntu-panel
```

## Contributing

All contributions are welcome, whether it be adding new base templates or anything else you might think of! This project is licensed under the [GPL-3.0](LICENSE) license!