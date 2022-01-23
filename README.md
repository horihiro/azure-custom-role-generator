# azure-custom-role-generator
This is a command line tool generating a template of Azure Custom Role definition from Azure REST API's endpoint and HTTP method in the Azure CLI.

> **_NOTE:_** The actions in the generated definition by this command are based on logs the `az` requests to Azure Resource Manager directly.

---

# Usage

```sh
$ az < subcommand, arguments, and options > --debug 2>&1 | custom-role-generator
{
  "AssignableScopes": [
    "/subscriptions/..."
  ],
  "actions": [
    "Some.Provider/.../..."
  ],
  "dataActions": [],
  "description": "Generated by custom-role-generator",
  "isCustom": true,
  "name": "<REPLACE_THIS>",
  "notActions": [],
  "notDataActions": []
}
```