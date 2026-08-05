require('nvim-treesitter').install { 'c', 'lua', 'zig' }

vim.api.nvim_create_autocmd('FileType', {
	pattern = { 'c', 'h', 'lua', 'zig', 'make' },
	callback = function() vim.treesitter.start() end
})
