vim.cmd [[packadd packer.nvim]]

return require('packer').startup(function(use)
	use 'wbthomason/packer.nvim'

	use {
		'nvim-telescope/telescope.nvim', tag='0.1.4',
		requires = {{'nvim-lua/plenary.nvim'}}
	}

	use({
		'folke/tokyonight.nvim',
		config = function()
			vim.cmd('colorscheme tokyonight')
		end
	})
	
	use('nvim-treesitter/nvim-treesitter', {run=':TSUpdate'})
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
end)
