# https://devenv.sh/getting-started
{ pkgs, ... }:
{
  languages.rust.enable = true;

  services.rabbitmq.enable = true;

  # https://devenv.sh/packages/
  packages = with pkgs; [
    cargo-audit
    cargo-auditable
    cargo-insta # https://github.com/mitsuhiko/insta
    cargo-machete
    cargo-mutants
    cargo-outdated
    cargo-semver-checks
    cargo-tarpaulin
    cargo-watch
    bacon
    codespell
    git
    just
    onefetch
    trunk
  ];

  # https://devenv.sh/git-hooks/
  git-hooks.hooks = {
    # Files
    check-symlinks.enable = true;
    # Rust
    rustfmt.enable = true;
    # Nix
    deadnix.enable = true;
    nil.enable = true;
    nixfmt-rfc-style.enable = true;
    statix.enable = true;
    # Shell
    shellcheck.enable = true;
    # TOML
    taplo.enable = true;
    check-toml.enable = true;
    # YAML
    check-yaml.enable = true;
    yamlfmt = {
      enable = true;
      settings.lint-only = false;
    };
    # Misc. formats
    check-json.enable = true;
    denofmt.enable = true;
    markdownlint = {
      enable = true;
      settings.configuration = {
        # https://github.com/DavidAnson/markdownlint/blob/main/schema/.markdownlint.jsonc
        MD013 = {
          # MD013/line-length : Line length : https://github.com/DavidAnson/markdownlint/blob/v0.40.0/doc/md013.md
          line_length = 100;
          code_blocks = false;
          tables = false;
          headings = false;
        };
      };
    };
    hadolint.enable = true; # Dockerfiles
    # Hyperlinks
    check-vcs-permalinks.enable = true;
    # EditorConfig
    editorconfig-checker.enable = true; # https://EditorConfig.org
    end-of-file-fixer.enable = true;
    trim-trailing-whitespace.enable = true;
    mixed-line-endings.enable = true;
    # Git
    commitizen.enable = true; # https://www.conventionalcommits.org/en/v1.0.0/#summary
    check-merge-conflicts.enable = true;
    check-added-large-files.enable = true;
    # Security
    ripsecrets.enable = true;
    detect-private-keys.enable = true;
  };

  enterShell = ''
    # set -x  # for debugging

    onefetch --no-color-palette --no-art --no-title
    echo "Run 'just' often, to prevent CI failures."
    echo ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
  '';
}
