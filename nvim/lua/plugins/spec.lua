return {
	{
		"folke/tokyonight.nvim",
		lazy = false,
		priority = 1000,
		config = function()
			vim.cmd('colorscheme tokyonight')
		end,
	},
	{
		"nvim-telescope/telescope-file-browser.nvim",
		lazy = true,
		dependencies = {
			"nvim-telescope/telescope.nvim",
			"nvim-lua/plenary.nvim",
		},
	},
	{
		"theprimeagen/harpoon",
		lazy = true,
	},
	{ "mbbill/undotree", lazy = false, },
	{ "neovim/nvim-lspconfig" },
	{
		"nvim-lualine/lualine.nvim",
		dependencies = { "nvim-tree/nvim-web-devicons" },
	},
	{ "nvim-treesitter/nvim-treesitter", branch = "main", },
	{ "hrsh7th/nvim-cmp", lazy = false, },
	{ "hrsh7th/cmp-nvim-lsp", lazy = false, },
	{ "hrsh7th/cmp-buffer", lazy = false, },
	{ "hrsh7th/cmp-path", lazy = false, },
	{ "hrsh7th/cmp-cmdline", lazy = false, },
}
