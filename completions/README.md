# Shell Completion Scripts for redstr

This directory contains shell completion scripts for redstr to provide
command-line autocompletion in various shells.

## Installation

### Bash

**System-wide installation:**
```bash
sudo cp redstr.bash /etc/bash_completion.d/redstr
```

**User installation:**
```bash
mkdir -p ~/.local/share/bash-completion/completions
cp redstr.bash ~/.local/share/bash-completion/completions/redstr
```

**Manual loading (add to ~/.bashrc):**
```bash
source /path/to/redstr.bash
```

### Zsh

**System-wide installation:**
```bash
sudo cp redstr.zsh /usr/share/zsh/site-functions/_redstr
```

**User installation:**
```bash
mkdir -p ~/.zsh/completions
cp redstr.zsh ~/.zsh/completions/_redstr
```

Then add to your ~/.zshrc:
```zsh
fpath=(~/.zsh/completions $fpath)
autoload -U compinit && compinit
```

### Fish

**System-wide installation:**
```bash
sudo cp redstr.fish /usr/share/fish/vendor_completions.d/
```

**User installation:**
```bash
mkdir -p ~/.config/fish/completions
cp redstr.fish ~/.config/fish/completions/
```

## Usage

After installation, completions will be available automatically when you
type `redstr` and press Tab:

```bash
$ redstr <TAB>
# Shows all available modes

$ redstr lee<TAB>
# Completes to "leetspeak"

$ redstr sql-<TAB>
# Completes to "sql-comment"
```

## Testing

### Bash
```bash
complete -p redstr
# Should show: complete -F _redstr_completions redstr
```

### Zsh
```zsh
which _redstr
# Should show the path to the completion function
```

### Fish
```fish
complete -c redstr
# Should list all completions
```

## Troubleshooting

### Bash
- Ensure bash-completion package is installed
- Check if other completions work: `ls <TAB>`
- Reload: `source ~/.bashrc`

### Zsh
- Verify fpath includes completion directory: `echo $fpath`
- Rebuild completion cache: `rm ~/.zcompdump && compinit`
- Check for errors: `zsh -xv`

### Fish
- Completions should work immediately after copying
- Reload: `source ~/.config/fish/config.fish`
- Check completion files: `complete -c redstr`

## Customization

You can modify these completion scripts to add custom modes or change
descriptions. After editing, reinstall using the commands above.
