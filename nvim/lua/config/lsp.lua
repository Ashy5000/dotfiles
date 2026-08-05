vim.lsp.inlay_hint.enable(true);

local bufnr = vim.api.nvim_get_current_buf()
vim.keymap.set(
  "n", 
  "<leader>a", 
  function()
    vim.cmd.RustLsp('codeAction') -- supports rust-analyzer's grouping
    -- or vim.lsp.buf.codeAction() if you don't want grouping.
  end,
  { silent = true, buffer = bufnr }
)
vim.keymap.set(
  "n", 
  "K",  -- Override Neovim's built-in hover keymap with rustaceanvim's hover actions
  function()
    vim.cmd.RustLsp({'hover', 'actions'})
  end,
  { silent = true, buffer = bufnr }
)

vim.lsp.config('clangd', {
        cmd = {
          "clangd",
          "--header-insertion=never",
          "--completion-style=detailed",
          "--rename-file-limit=0",
          "--background-index",
          "--background-index-priority=normal",
        },
        filetypes = { "c", "cpp", "objc", "objcpp" },
})
vim.lsp.enable('clangd')

zig_root = function(bufnr, on_dir)
	on_dir(vim.fs.root(bufnr, { 'build.zig', '.git' }))
end

vim.lsp.config('zls', {
  cmd = { "zls" },
  filetypes = { "zig", "zir" },
  root_dir = zig_root,
  single_file_support = true,
})
vim.lsp.enable('zls')

vim.diagnostic.enable = true
vim.diagnostic.config({
	virtual_lines = false,
	signs = false,
})

vim.keymap.set("n", "gd", vim.lsp.buf.declaration)
vim.keymap.set("n", "<leader>fo", vim.lsp.buf.format)
vim.keymap.set("n", "<leader>fr", vim.lsp.buf.rename)
vim.keymap.set("n", "<leader>e", function() vim.diagnostic.open_float(nil, {focus=false}) end)

local cmp = require'cmp'

cmp.setup({
snippet = {
-- REQUIRED - you must specify a snippet engine
expand = function(args)
-- vim.fn["vsnip#anonymous"](args.body) -- For `vsnip` users.
-- require('luasnip').lsp_expand(args.body) -- For `luasnip` users.
-- require('snippy').expand_snippet(args.body) -- For `snippy` users.
-- vim.fn["UltiSnips#Anon"](args.body) -- For `ultisnips` users.
vim.snippet.expand(args.body) -- For native neovim snippets (Neovim v0.10+)

-- For `mini.snippets` users:
-- local insert = MiniSnippets.config.expand.insert or MiniSnippets.default_insert
-- insert({ body = args.body }) -- Insert at cursor
-- cmp.resubscribe({ "TextChangedI", "TextChangedP" })
-- require("cmp.config").set_onetime({ sources = {} })
end,
},
window = {
-- completion = cmp.config.window.bordered(),
-- documentation = cmp.config.window.bordered(),
},
mapping = cmp.mapping.preset.insert({
['<C-b>'] = cmp.mapping.scroll_docs(-4),
['<C-f>'] = cmp.mapping.scroll_docs(4),
['<C-Space>'] = cmp.mapping.complete(),
['<C-e>'] = cmp.mapping.abort(),
['<CR>'] = cmp.mapping.confirm({ select = true }), -- Accept currently selected item. Set `select` to `false` to only confirm explicitly selected items.
}),
sources = cmp.config.sources({
{ name = 'nvim_lsp' },
{ name = 'vsnip' }, -- For vsnip users.
-- { name = 'luasnip' }, -- For luasnip users.
-- { name = 'ultisnips' }, -- For ultisnips users.
-- { name = 'snippy' }, -- For snippy users.
}, {
{ name = 'buffer' },
})
})

-- To use git you need to install the plugin petertriho/cmp-git and uncomment lines below
-- Set configuration for specific filetype.
--[[ cmp.setup.filetype('gitcommit', {
sources = cmp.config.sources({
{ name = 'git' },
}, {
{ name = 'buffer' },
})
})
require("cmp_git").setup() ]]-- 

-- Use buffer source for `/` and `?` (if you enabled `native_menu`, this won't work anymore).
cmp.setup.cmdline({ '/', '?' }, {
mapping = cmp.mapping.preset.cmdline(),
sources = {
{ name = 'buffer' }
}
})

-- Use cmdline & path source for ':' (if you enabled `native_menu`, this won't work anymore).
cmp.setup.cmdline(':', {
mapping = cmp.mapping.preset.cmdline(),
sources = cmp.config.sources({
{ name = 'path' }
}, {
{ name = 'cmdline' }
}),
matching = { disallow_symbol_nonprefix_matching = false }
})

-- Set up lspconfig.
local capabilities = require('cmp_nvim_lsp').default_capabilities()

vim.lsp.config('hls', {
	filetypes = { 'haskell', 'lhaskell', 'cabal' },
})
vim.lsp.enable('hls')

vim.api.nvim_create_autocmd("BufWritePre", {
	pattern = "*",
	callback = function(args)
		vim.lsp.buf.format()
	end,
})
