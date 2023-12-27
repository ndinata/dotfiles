" Plugins
" -------------------------------------------
call plug#begin(stdpath('data') . '/plugged')

" Sample configs:
" https://github.com/n3wborn/nvim
" https://github.com/mrnugget/vimconfig
" https://github.com/lucax88x/configs/tree/master/dotfiles/.config/nvim
" https://github.com/JoosepAlviste/dotfiles/tree/master/config/nvim
" https://github.com/shaunsingh/nix-darwin-dotfiles
" https://github.com/astronvim/astronvim
" https://github.com/NvChad/NvChad
" https://github.com/VapourNvim/VapourNvim

" General
Plug 'jiangmiao/auto-pairs'
Plug 'preservim/nerdtree'
Plug 'tpope/vim-commentary'
Plug 'tpope/vim-fugitive'
Plug 'airblade/vim-gitgutter'
Plug 'terryma/vim-multiple-cursors'
Plug 'tpope/vim-surround'
Plug 'dense-analysis/ale'
Plug 'vim-airline/vim-airline'
Plug 'nvim-treesitter/nvim-treesitter', {'do': ':TSUpdate'}
Plug 'NoahTheDuke/vim-just'
" Plug 'nvim-telescope/telescope.nvim'
" Plug 'b3nj5m1n/kommentary'
" Plug 'junegunn/fzf', { 'do': { -> fzf#install() } }
" Plug 'junegunn/fzf.vim'
" Plug 'iamcco/markdown-preview.nvim'
" https://github.com/wbthomason/packer.nvim
" https://github.com/neovim/nvim-lspconfig
" https://github.com/glepnir/lspsaga.nvim (lsp plugin)
" https://github.com/stevearc/aerial.nvim (symbol skimming + navigation)
" https://github.com/kyazdani42/nvim-tree.lua
" https://github.com/APZelos/blamer.nvim
" https://github.com/folke/trouble.nvim — diagnostic, search etc. list
" https://github.com/lukas-reineke/indent-blankline.nvim
" https://github.com/norcalli/nvim-colorizer.lua
" https://github.com/akinsho/toggleterm.nvim
" https://github.com/pwntester/octo.nvim — github integration
" https://github.com/jose-elias-alvarez/nvim-lsp-ts-utils
" https://github.com/mattn/emmet-vim
" https://github.com/simrat39/rust-tools.nvim

" Plug 'wellle/targets.vim'
" Plug ctrlpvim/ctrlp.vim
" Plug 'wincent/command-t'
" Plug 'neoclide/vim-jsx-improve'
" Plug 'easymotion/vim-easymotion'
" Plug 'honza/vim-snippets'
" Plug 'mechatroner/rainbow_csv'
" Plug 'hail2u/vim-css3-syntax'

" Colour schemes
Plug 'joshdick/onedark.vim'
Plug 'sainnhe/edge'
" Plug 'dracula/vim', { 'as': 'dracula' }
" Plug 'arzg/vim-colors-xcode'
" Plug 'rktjmp/lush.nvim'
" Plug 'ellisonleao/gruvbox.nvim'

" Language syntax
Plug 'vim-python/python-syntax'
Plug 'pangloss/vim-javascript'
Plug 'maxmellon/vim-jsx-pretty'
Plug 'SolaWing/vim-objc-syntax'
Plug 'vim-ruby/vim-ruby'
Plug 'arzg/vim-swift'
Plug 'keith/swift.vim'
" Plug 'peitalin/vim-jsx-typescript'
" Plug 'leafgarland/typescript-vim'

" Formatter/fixer/linter
Plug 'rhysd/vim-clang-format', {
  \ 'for': ['c', 'cpp', 'objc']
  \ }
Plug 'prettier/vim-prettier', {
  \ 'do': 'yarn install',
  \ 'for': ['javascript', 'typescript', 'css', 'json', 'graphql', 'markdown', 'html', 'yaml']
  \ }
Plug 'ruanyl/vim-sort-imports', {
  \ 'for': ['javascript', 'typescript']
  \ }

call plug#end()


" Settings
" ---------------------------------------------
" update current directory on switching buffers
set autochdir

" set command-line height to 2 lines
set cmdheight=2

set completeopt=menu,menuone,preview,noinsert

" copy to the clipboard when yanking
set clipboard=unnamed

" use spaces instead of tabs
set expandtab

" hides a buffer when it's abandoned
set hidden

" case-insensitive search
set ignorecase

" display invisible characters
set list
set listchars=tab:>\ ,trail:-,extends:>,precedes:<,nbsp:+

" allow manually triggering ALE's completion with <C-x><C-o>
set omnifunc=ale#completion#OmniFunc

" enable mouse
set mouse=a

" load new buffer without creating a swapfile
set noswapfile

" display line numbers
set number

" show vertical context by 3 lines
set scrolloff=3

" indent to next multiple of 'shiftwidth' (which is by 2 spaces)
set shiftround
set shiftwidth=2

set shortmess+=c

" highlight matching {[()]}
set showmatch

" show horizontal context
set sidescrolloff=3

" always show column next to line number
set signcolumn=yes

" case-sensitive search if any uppercase characters
set smartcase

set smartindent

" tab key indents by 2 spaces
set softtabstop=2

" split new panes to the bottom and right
set splitbelow
set splitright

set termguicolors

" set delay before updates happen (ms)
set updatetime=100

" disable saving/restoring local options & mappings
set viewoptions-=options

" Start terminal in Insert mode
au TermOpen term://* startinsert

" Automatically save and load folds
au BufWinLeave *.* mkview
au BufWinEnter *.* silent! loadview


" Key Mappings
" -----------------------------------------
" Switching between windows: Ctrl + h,j,k,l
nnoremap <C-h> <C-w>h
nnoremap <C-j> <C-w>j
nnoremap <C-k> <C-w>k
nnoremap <C-l> <C-w>l


" Switching between buffers: Shift + left,right
nnoremap <S-Left> :bprev<CR>
nnoremap <S-Right> :bnext<CR>


" Open new empty buffer: <leader> t
" nnoremap <leader>t :enew<CR>


" Toggle selected fold: Space
nnoremap <silent> <Space> @=(foldlevel('.') ? 'za' : "\<Space>")<CR>
vnoremap <Space> zf


" Turn terminal to normal mode: Esc
tnoremap <Esc> <C-\><C-n>


" Use . in visual mode to execute the dot command on each selected line
xnoremap . :normal .<CR>


" Move/drag lines
" https://vim.fandom.com/wiki/Moving_lines_up_or_down
" Why the funny symbols? https://stackoverflow.com/a/15399297
nnoremap ∆ :m .+1<CR>==
nnoremap ˚ :m .-2<CR>==
vnoremap ∆ :m '>+1<CR>gv=gv
vnoremap ˚ :m '<-2<CR>gv=gv


" Plugin Settings
" -------------------------------------------
" b3nj5m1n/kommentary
" lua << EOF
" require('kommentary.config').configure_language("default", {
"    prefer_single_line_comments = true,
" })
" EOF
" nmap <C-_> gcc
" vmap <C-_> gc

"  dense-analysis/ale
let g:ale_linters = { 'python': ['flake8', 'mypy', 'pyright'], 'rust': ['analyzer', 'cargo'] }
let g:ale_fixers = {
\ 'css': ['prettier'],
\ 'html': ['prettier'],
\ 'javascript': ['prettier'],
\ 'typescript': ['prettier'],
\ 'json': ['prettier'],
\ 'python': ['black', 'isort'],
\ 'swift': ['swiftformat'],
\ 'rust': ['rustfmt'],
\}
let g:ale_fix_on_save = 1
let g:ale_completion_enabled = 1
let g:ale_completion_delay = 50
let g:ale_completion_autoimport = 1
let g:ale_floating_preview = 1
let g:ale_hover_to_floating_preview = 1
let g:ale_detail_to_floating_preview = 1
let g:ale_close_preview_on_insert = 1

" line-length = 80 chars
" let g:ale_python_black_options = "-l 80"

" let g:ale_cursor_detail = 1
" let g:ale_hover_cursor = 1
" let g:ale_swift_swiftformat_options = '
"   \ --indent 2
"   \ --indentcase true
"   \ --maxwidth 120
"   \ --wrapcollections before-first
"   \ '

nnoremap ]e :ALENextWrap<CR>
nnoremap [e :ALEPreviousWrap<CR>

nnoremap md :ALEDetail<CR>
nnoremap mh :ALEHover<CR>

inoremap <silent><expr> <Tab>
  \ pumvisible() ? "\<C-n>" : "\<TAB>"
inoremap <silent><expr> <S-Tab>
  \ pumvisible() ? "\<C-p>" : "\<S-TAB>"
inoremap <silent><expr> <CR>
  \ pumvisible() ? "\<C-y>" : "\<CR>"


" junegunn/fzf
" hide statusline for cleaner look
" au! FileType fzf
" au FileType fzf set laststatus=0 noshowmode noruler
"   \| au BufLeave <buffer> set laststatus=2 showmode ruler
" let g:fzf_action = {
"   \ 'ctrl-t': 'vsplit',
"   \ 'ctrl-s': 'split',
"   \ }
" nnoremap <C-p> :FZF<CR>


" vim-airline/vim-airline
let g:airline#extensions#tabline#enabled = 1
let g:airline#extensions#tabline#formatter = 'unique_tail'


" nvim-treesitter/nvim-treesitter
lua <<EOF
require'nvim-treesitter.configs'.setup {
  ensure_installed = {
    "bash",
    "c",
    "cpp",
    "css",
    "fish",
    "html",
    "java",
    "javascript",
    "json",
    "jsonc",
    "kotlin",
    "lua",
    "python",
    "regex",
    "ruby",
    "rust",
    "toml",
    "tsx",
    "typescript",
    "vim",
    "yaml",
  },
  highlight = {
    enable = true,              -- false will disable the whole extension
  },
}
EOF


" preservim/nerdtree
" close vim if NERDTree is last open window
au BufEnter * if (winnr("$") == 1 &&
  \ exists("b:NERDTree") &&
  \ b:NERDTree.isTabTree()) | q | endif

let NERDTreeShowHidden = 1    " show hidden (dot) files
let NERDTreeMinimalUI = 1     " hide Help message

nnoremap <C-b> :NERDTreeToggle<CR>
nnoremap <C-f> :NERDTreeFind<CR>


" tpope/vim-commentary
" map to <C-_> because vim registers <C-/> as <C-_>
nmap <C-_> gcc
vmap <C-_> gc


" airblade/vim-gitgutter
nmap ghs :GitGutterStageHunk<CR>
nmap ghu :GitGutterUndoHunk<CR>
nmap ghp :GitGutterPreviewHunk<CR>
nmap ghf :GitGutterFold<CR>


" dracula/vim
" let g:dracula_italic = 0


" vim-python/python-syntax
let g:python_highlight_all = 1
let g:python_highlight_file_headers_as_comments = 1


" pangloss/vim-javascript
let g:javascript_plugin_jsdoc = 1
let g:javascript_plugin_flow = 1


" maxmellon/vim-jsx-pretty
let g:vim_jsx_pretty_colorful_config = 1


" rhysd/vim-clang-format
let g:clang_format#auto_format = 1
let g:clang_format#code_style = 'mozilla'


" prettier/vim-prettier
let g:prettier#autoformat = 1
let g:prettier#autoformat_require_pragma = 0


" arzg/vim-colors-xcode
" let g:xcodedark_match_paren_style = 1
" let g:xcodedark_dim_punctuation = 0


" sainnhe/edge
let g:edge_style = 'default'
let g:edge_enable_italic = 0
let g:edge_disable_italic_comment = 1

colorscheme edge
