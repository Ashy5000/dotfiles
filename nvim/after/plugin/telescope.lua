local builtin = require('telescope.builtin')
vim.keymap.set('n', '<leader>ff', builtin.find_files, {})
vim.keymap.set('n', '<leader>pf', builtin.git_files, {})
vim.keymap.set('n', '<leader>ps', function()
	builtin.grep_string({ search = vim.fn.input("Grep > ") });
end)

vim.keymap.set("n", "<space>fb", ":Telescope file_browser path=%:p:h select_buffer=true<CR>")

require("telescope").setup {
	extensions = {
		file_browser = {
			-- disables netrw and use telescope-file-browser in its place
			hijack_netrw = true,
		},
	},
}

require("telescope").load_extension "file_browser"
