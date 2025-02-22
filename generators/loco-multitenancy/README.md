## spring-rs multitenancy generator

### Overview

The spring-rs multitenancy generator helps you set up a multitenancy architecture for your spring-rs based application. 
It supports column-based multitenancy and provides configurations to manage multiple tenants efficiently.

### Key Components

1. **Configuration Files**:
    - `values.yaml`: Contains the configuration for the multitenancy setup.
    - `app.toml`: Contains additional configurations for the web server and database connections.

2. **Database Setup**:
    - `console.sql`: SQL script to set up your database schema and insert initial data.

### Configuration Details

#### `values.yaml`

This file configures the multitenancy settings for your application.

```yaml
application:
  multitenancy:
    enabled: true
    type: "column"
    entity-alias: company
```

- `enabled`: Enable or disable multitenancy.
- `type`: Type of multitenancy. Currently supports "column".
- `entity-alias`: Alias for the tenant entity.

### Summary

1. **Clone the Repository**: Clone the generator repository to your local machine.
2. **Configure `values.yaml`**: Update the `values.yaml` file with your desired multitenancy settings.
3. **Configure `app.toml`**: Update the `app.toml` file with your web server and database configurations.
4. **Run Database Scripts**: Execute the SQL scripts in `console.sql` to set up your database.
5. **Integrate with Your Application**: Integrate the generated configurations and scripts with your Spring application.