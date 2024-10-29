# cosmic-ext-config-templates: cosmic-config template program for the COSMIC desktop!

This program will allow you to create template files that can apply COSMIC config settings en-masse! The motivation to make this program was so that I could have different panel templates and apply them at any time.

## Install

```shell
cargo install cosmic-ext-config-templates
```

## Usage

```shell
cosmic-ext-config-templates --help
```

### Example 1 (Generate a panel template)

```shell
cosmic-ext-config-templates generate ./my_panel.ron com.system76.CosmicPanel com.system76.CosmicPanel.Dock com.system76.CosmicPanel.Panel com.system76.CosmicPanelButton
```

### Example 2 (Load a panel template from a file)

```shell
cosmic-ext-config-templates load-file ./my_panel.ron
```

### Example 3 (Load a premade Ubuntu panel template)

```shell
cosmic-ext-config-templates load-template ubuntu-panel
```