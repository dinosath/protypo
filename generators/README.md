# Generators Documentation

## Overview

This documentation provides an overview of the generators available in the Protypo project and guides on how to extend them.

A generator in Protypo is a collection of templates, configuration files, and resources that define the code generation structure.

### Structure

A generator typically follows this structure:

```
/generator
├── Generator.yaml
├── files/
├── templates/
├── values.yaml
└── entities/
```

* `Generator.yaml`: Contains the metadata for the generator, such as version, description, dependencies, and maintainers.
* `files/`: Contains files that are copied directly to the output directory.
* `templates/`: Holds template files that are used to generate code.
* `values.yaml`: Defines default values used within the templates, which can be overridden by command-line arguments or schema files.
* `entities/`: Stores the JSON schema files that describe the entities used in the templates.

#### Sample Generator

Here’s an example of a generator configuration for Rust projects using the spring-rs framework:

```yaml
apiVersion: v1
name: commons
version: 0.0.1
description: Package containing common macros for handling jsonschema files. Use in tera templates.
keywords:
  - commons
  - jsonschema
  - commons
  - tera
sources:
  - https://github.com/dinosath/protypo
maintainers:
  - name: Konstantinos Athanasiou
    email: dinosath0@gmail.com
```


## Available Generators

* `jdl`
* `jsonschema-commons`
* `loco`
* `loco-react-admin`
* `react-admin`
* `seaography`
* `seaorm-entities`
* `shuttle`
* `spring-rs`
* `spring-rs-react-admin`

## Generation

During generation the values are all merged in one object from the deepest to the oldest in the hierarchy.

The templates will be used to generate code based on the provided values and entities.

### Loading Order
Generators are loaded from the deepest to the highest in the directory tree. If there is a template with the same name in a higher order directory, it will override the one in the lower order directory.  In the context of the templates, the entities will be loaded in the same depth-first approach and will be merged with the oldest.