{
  projectRootFile = "flake.nix";
  programs = {
    nixfmt.enable = true;
    deadnix.enable = true;
    statix.enable = true;
    taplo.enable = true;
    rumdl-format.enable = true;
    yamlfmt.enable = true;
    rustfmt.enable = true;
  };
}
