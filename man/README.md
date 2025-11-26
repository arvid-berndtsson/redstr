# Man Page for redstr

This directory contains the manual page for the redstr command-line tool.

## Installation

### System-wide installation
```bash
sudo mkdir -p /usr/local/man/man1
sudo cp redstr.1 /usr/local/man/man1/
sudo mandb
```

### Debian/Ubuntu package installation
The man page is automatically installed when using the Debian package:
```bash
sudo dpkg -i redstr_*.deb
```

### Manual installation (user-specific)
```bash
mkdir -p ~/.local/share/man/man1
cp redstr.1 ~/.local/share/man/man1/
```

Then add to your shell configuration (~/.bashrc, ~/.zshrc, etc.):
```bash
export MANPATH="$HOME/.local/share/man:$MANPATH"
```

## Usage

After installation, view the man page with:
```bash
man redstr
```

You can also view it directly without installation:
```bash
man ./redstr.1
```

## Sections

The man page includes:
- **NAME** - Brief description
- **SYNOPSIS** - Command syntax
- **DESCRIPTION** - Detailed description
- **OPTIONS** - Command-line options
- **MODES** - All transformation modes
- **EXAMPLES** - Usage examples
- **USE CASES** - Red/Blue/Purple team scenarios
- **INTEGRATION** - Tool integration information
- **RESPONSIBLE USE** - Security and ethics
- **FILES** - Related files
- **BUGS** - Bug reporting
- **AUTHOR** - Author information
- **COPYRIGHT** - License information
- **SEE ALSO** - Related documentation

## Generating Other Formats

### HTML
```bash
man2html redstr.1 > redstr.html
```

### PDF (requires groff)
```bash
groff -man -Tpdf redstr.1 > redstr.pdf
```

### Plain text
```bash
man ./redstr.1 | col -b > redstr.txt
```

## Testing

Check the man page for errors:
```bash
# Check for formatting issues
man --warnings ./redstr.1

# View in terminal
man ./redstr.1

# Search for specific content
man ./redstr.1 | grep -i "phishing"
```

## Updating

When updating the man page:
1. Edit redstr.1
2. Verify formatting: `man --warnings ./redstr.1`
3. Test display: `man ./redstr.1`
4. Reinstall: `sudo cp redstr.1 /usr/local/man/man1/ && sudo mandb`

## Man Page Format

The man page uses groff/troff formatting:
- `.TH` - Title header
- `.SH` - Section header
- `.TP` - Tagged paragraph (for options)
- `.IP` - Indented paragraph (for lists)
- `.B` - Bold text
- `.I` - Italic text
- `.RS/.RE` - Indent/outdent
- `.PP` - Paragraph break

For more information on man page formatting, see:
```bash
man 7 groff_man
man 7 man
```
