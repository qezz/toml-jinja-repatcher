# Toml Jinja re-patcher

Applies jinja template holes to a "new" toml file based on the "old" template.

## Usage

```
cargo run -- --from /path/to/updated/file.toml --into /path/to/jinja/template.toml.j2
```

Reads the template in `--into`, "remembers" the template holes, applies these holes to `--from` file.
Overwrites the `--into` template with the updated one.

## Example

Having jinja2 template

```j2
[section]
hello = "{{ object }}"
```

and a new toml file

```toml
[section]
hello = "cats"
new_key = "abc"
```

produces the updated jinja2 template

```j2
[section]
hello = "{{ object }}"
new_key = "abc"
```
