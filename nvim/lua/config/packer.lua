vim.cmd [[packadd packer.nvim]]

return require('packer').startup(function(use)
	use 'wbthomason/packer.nvim'

	use ({
		"nvim-telescope/telescope-file-browser.nvim",
		requires = { "nvim-telescope/telescope.nvim", "nvim-lua/plenary.nvim" }
	})

	use({
		'folke/tokyonight.nvim',
		config = function()
			vim.cmd('colorscheme tokyonight')
		end
	})
	
	use({ 'nvim-treesitter/nvim-treesitter', branch = "main", {run=':TSUpdate'} })
	use('theprimeagen/harpoon')
	use('mbbill/undotree')
	use('neovim/nvim-lspconfig')
	use('mrcjkb/rustaceanvim')
	use('hrsh7th/cmp-nvim-lsp')
	use('hrsh7th/cmp-buffer')
	use('hrsh7th/cmp-path')
	use('hrsh7th/cmp-cmdline')
	use('hrsh7th/nvim-cmp')
	use('junegunn/goyo.vim')
	use('glacambre/firenvim')
	-- use('MunifTanjim/nui.nvim')
	-- use('rcarriga/nvim-notify')
	-- use('folke/noice.nvim')
	use('theprimeagen/vim-be-good')
	use('mrcjkb/haskell-tools.nvim')
	use('RaafatTurki/hex.nvim')
	use({
		'nvim-lualine/lualine.nvim',
		requires = { 'nvim-tree/nvim-web-devicons', opt = true }
	})
end)
