# Use cmd.exe for Windows recipes so Windows-only commands work even when
# `just` is launched from Git Bash/MSYS.
set windows-shell := ["cmd.exe", "/c"]

alias i := install
alias r := run
alias c := clean
alias t := typecheck

PKG_MANAGER := "pnpm"

install:
    {{PKG_MANAGER}} install

run:
    {{PKG_MANAGER}} run dev

typecheck:
    {{PKG_MANAGER}} run typecheck

[unix]
clean:
    rm -rf node_modules

[windows]
clean:
    if exist node_modules rmdir /s /q node_modules