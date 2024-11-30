local M = {}

M.init = function(day, impl, example, input)
    vim.cmd.edit(impl)
    vim.cmd 'Copilot disable' -- no spoilers please

    vim.cmd.vsplit()
    vim.cmd(string.format("terminal zsh -c 'git ls-files | entr cargo test --package day%d -- --nocapture --test-threads=1'", day))
    vim.cmd 'normal G' -- Scroll to bottom on new output

    -- back to main window
    vim.cmd 'wincmd h'
    vim.cmd 'wincmd 10>'

    vim.cmd(string.format('DebugExe target/debug/day%d', day))

    function build_and_debug(input) 
        vim.cmd('wa')
        vim.cmd(string.format('!cargo build --package day%d', day))
        if vim.v.shell_error == 0 then
            vim.fn.feedkeys('<CR>') -- get rid of the press any key prompt
            vim.cmd(string.format('DebugArgs %s', input))
            require'dap'.continue()
        end
    end

    vim.keymap.set('n', '<leader>da', function() build_and_debug(input) end)
    vim.keymap.set('n', '<leader>dA', function() build_and_debug(example) end)

    local overseer = require'overseer'
    overseer.register_template {
        name = "solve",
        builder = function(params) 
            local args = {
                "run",
                '--package', 'day' .. day,
                "--", params.input
            }
            if params.release then
                table.insert(args, 2, "--release")
            end

            return {
                cmd = {'cargo'},
                args = args,
                strategy = {
                    'toggleterm',
                    use_shell = true, -- Needed for xclip to work correctly
                    auto_scroll = true,
                    on_create = function()
                        vim.keymap.set('n', 'q', ':q<CR>', {buffer = true, silent = true})
                        -- Open in new tab
                        vim.keymap.set('n', 't', '<C-w>T<CR>', {buffer = true})
                        -- https://github.com/stevearc/overseer.nvim/issues/186
                        vim.cmd("stopinsert")
                    end,
                },
            }
        end,
        params = {
            input = {
                type = "string",
            },
            release = {
                type = "boolean",
                default = false,
                optional = true,
            }
        },
    }
    overseer.add_template_hook({ name = "solve" }, function(task_defn, util)
        util.remove_component(task_defn, "on_output_quickfix")
    end)

    function save_and_solve(params) 
        vim.cmd'wa'
        overseer.run_template({name = "solve", params = params})
    end

    vim.keymap.set('n', '<leader>a', function() save_and_solve({input = input}) end)
    vim.keymap.set('n', '<leader>A', function() save_and_solve({input = example}) end)

    vim.keymap.del('n', '<leader>or')
    vim.keymap.del('n', '<leader>ot')

    vim.keymap.set('n', '<leader>o', function() save_and_solve({input = input, release = true}) end)
    vim.keymap.set('n', '<leader>O', function() save_and_solve({input = example, release = true}) end)

    vim.keymap.set('n', '<leader>ca', function() 
        vim.cmd(string.format(":!cargo add %s --package day%d", vim.fn.input("package(s) to add: "), day))
    end)
end

return M
