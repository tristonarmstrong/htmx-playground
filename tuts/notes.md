# How to add eslint to neovim

using npm global install `vscode-langservers-extracted`.
This will give your editor access to `vscode-eslint-language-server`.

## add the appropriate packages

```lua
{
    "neovim/nvim-lspconfig",
    config = function()
      require "plugins.configs.lspconfig"
    end, -- Override to setup mason-lspconfig
  },
  {
    "jose-elias-alvarez/null-ls.nvim",
  },
  {
    "MunifTanjim/eslint.nvim",
    config = function()
      require "custom.configs.eslint"
    end,
  },
```

I have a `custom.configs.eslint` file that contains other eslint configs

```lua
local null_ls = require "null-ls"
local eslint = require "eslint"

null_ls.setup()

eslint.setup {
  bin = "eslint", -- or `eslint_d`
  code_actions = {
    enable = true,
    apply_on_save = {
      enable = true,
      types = { "directive", "problem", "suggestion", "layout" },
    },
    disable_rule_comment = {
      enable = true,
      location = "separate_line", -- or `same_line`
    },
  },
  diagnostics = {
    enable = true,
    report_unused_disable_directives = false,
    run_on = "type", -- or `save`
  },
}
```

then in your lspconfig configuration file, add your eslint

```lua
--                                                               here --v
local servers = { "lua_ls", "html", "cssls", "tsserver", "clangd", "eslint" }
local lspConfig = require "lspconfig"

for _, lsp in ipairs(servers) do
  lspConfig[lsp].setup {
    on_attach = M.on_attach,
    capabilities = M.capabilities,
  }
end
```

### Tools to help with custom eslint making

[AST Explorer](https://astexplorer.net/)
