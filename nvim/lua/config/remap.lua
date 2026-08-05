vim.opt.tabstop = 8
vim.opt.softtabstop = 8
vim.opt.shiftwidth = 8
vim.g.mapleader = " "
vim.keymap.set("n", "<leader>pv", vim.cmd.Ex)
vim.keymap.set("n", "<C-h>", "<C-w>h")
vim.keymap.set("n", "<C-j>", "<C-w>j")
vim.keymap.set("n", "<C-k>", "<C-w>k")
vim.keymap.set("n", "<C-l>", "<C-w>l")
vim.keymap.set("n", "<leader>h", function() vim.cmd("edit" .. vim.fn.expand("%:p:r") .. ".h") end)
vim.keymap.set("n", "<leader>c", function() vim.cmd("edit" .. vim.fn.expand("%:p:r") .. ".c") end)
