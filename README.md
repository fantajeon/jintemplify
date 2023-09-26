# Jintemplify

`jintemplify` is a versatile tool that enables users to combine Jinja2 templates with YAML variables, producing files in any desired format. The application also supports a plugin system based on shell scripts, allowing users to extend its functionality with familiar scripting techniques.

## Features

- **Template Rendering**: Use Jinja2 templates to define the structure of your file.
- **Variable Support**: Combine your templates with YAML-defined variables.
- **Plugin System (Shell Script Based)**: Extend the application's functionality with custom shell-script-based plugins. This allows for a wide range of extensibility using familiar scripting methods.

# Advanced Templating Features

## Accessing Environment Variables and Command Line Arguments

Within Jinja2 templates, you can use the vars object to access environment variables or any values passed using the --env command line option. This provides a seamless way to incorporate dynamic values into your templates based on the environment or runtime conditions.

```jinja2
{{ vars.my_environment_variable }}
{{ vars.my_cli_argument }}
```

In the above example, `my_environment_variable` could be an environment variable, and `my_cli_argument` could be a value passed via `--env`.

## Plugins

For those looking to extend the application's functionality with plugins, here's the basic structure for the plugin configuration:

```yaml
function_name:
  params:
    - name: parameter_name
  script: your_shell_script_command_here
```

In this structure:

- `function_name` is the name of the function you're adding, which can be directly called within your Jinja2 templates.
- `params` lists the parameters your function requires.
- `script` contains the shell command that the function will execute when called.es

## Filters

With `jintemplify`, you're not limited to just basic Jinja2 templating. We've introduced specialized filters and functions to provide more flexibility:

- **Reading Files Directly**: With the read_file function, you can directly read the contents of a file into your Jinja2 template. This is especially useful for including large chunks of data or content without manually copying them into the template.

- **Reading From Strings**: If you have data embedded within your templates as strings, you can convert them into usable Jinja2 objects with the following filters:
  - `from_read_json`: Parse a JSON string and convert it to a Jinja2 object.
  - `from_read_yaml`: Parse a YAML string and convert it to a Jinja2 object.
  - `from_read_toml`: Parse a TOML string and convert it to a Jinja2 object.

By using these filters, you can seamlessly integrate inline data within your templates and then manipulate them using Jinja2's powerful templating capabilities.

### Example: Using my_read_file in plugin.yaml.j2 with JSON Parsing

One of the powerful combinations you can use in `jintemplify` is to read a file directly and then parse its content. Here's a quick example:

```yaml
{# plugin.yaml.j2 #}
my_read_file:
  params:
    - name: file_path
      description: file path
  script: cat $(file_path)

```

```jinja
{# main.yaml.j2 #}
{% set conf = my_read_file(file_path='./examples/test.json') | from_json %}
{{conf.hello}}
...
```

In this example, we're using the `read_file` function to read the contents of `test.json`. We then utilize the `from_json` filter to parse the read JSON string, converting it into a usable Jinja2 object. This allows you to directly access properties of the JSON, like `conf.hello` in the example above.

# Installation

## Using Cargo

If you have Rust and Cargo installed, you can easily install `jintemplify` using:

```bash
cargo instll jintemplify
```

## Manual Installation

1. Clone the repository:

```
git clone https://github.com/your_username/jintemplify.git
```

2. Navigate to the project directory and build using Cargo:

```
cd jintemplify
cargo build --release
```

## Usage

```bash
jintemplify -t <template_path> -v <variables_path> --plugin <plugin_path>
```

- `-t, --template`: Specify the main Jinja2 template file.
- `-v, --variables`: Specify the YAML variables template (optional).
- `--plugin`: Specify the plugin YAML file to extend functionality.

For more detailed information on each parameter, run:

```bash
jintemplify --help
```

## Development

To add new filters, modify `filter.rs`. For adding or modifying plugins, see `plugin.rs`.

## Contributing

Pull requests are welcome. For major changes, please open an issue first to discuss what you would like to change.

## License

[MIT](https://choosealicense.com/licenses/mit/)
