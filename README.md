# cosmic-ext-config-templates: cosmic-config template program for the COSMIC desktop!

This program will allow you to create template files that can apply COSMIC config settings en-masse! The motivation to make this program was so that I could have different panel templates and apply them at any time.

## Usage

```shell
cargo run -- --help
```

### Example 1 (Generate a panel template)

```shell
cargo run -- generate ./my_panel.ron com.system76.CosmicPanel com.system76.CosmicPanel.Dock com.system76.CosmicPanel.Panel com.system76.CosmicPanelButton
```

### Example 2 (Load a panel template from a file)

```shell
cargo run -- load-file ./my_panel.ron
```

### Example 3 (Load a premade Ubuntu panel template)

```shell
cargo run -- load-template ubuntu-panel
```