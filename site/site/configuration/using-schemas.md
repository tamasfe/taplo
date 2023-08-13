# Using Schemas

JSON schemas can be assigned to TOML documents according to the following in priority order starting with the highest priority:

1. set manually in the environment, e.g. [as a CLI flag](../cli/usage/validation.md#using-a-specific-schema) or an IDE setting
1. [schema directives](./directives.md#the-schema-directive) at the top of the document
1. as an URL under the `$schema` key in the root of the document
1. [configuration file rule](./file#rules)
1. default schema set in the [configuration file](./file#schema)
1. contributed by an [extension](./developing-schemas.md#visual-studio-code-extensions) *(Visual Studio Code only)*
1. an association based on a [schema catalog](./developing-schemas.md#publishing)

Extra root CA certificate could be added by specifying with the TAPLO_EXTRA_CA_CERTS environment. The provided paths must be absolute paths. For example, `TAPLO_EXTRA_CA_CERTS=/home/taplo-user/custom-ca.pem`
