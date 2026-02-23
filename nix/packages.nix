{
  craneLib,
  pkgs,
  src,
}:
let
  madMeta = craneLib.crateNameFromCargoToml { cargoToml = ../crates/mad/Cargo.toml; };

  commonArgs = {
    inherit src;
    inherit (madMeta) pname version;
    strictDeps = true;
  };

  cargoArtifacts = craneLib.buildDepsOnly commonArgs;

  mad = craneLib.buildPackage (
    commonArgs
    // {
      inherit cargoArtifacts;
      cargoExtraArgs = "-p mad";
      postInstall = ''
        installShellCompletion --cmd mad \
          --bash <($out/bin/mad --bpaf-complete-style-bash) \
          --zsh <($out/bin/mad --bpaf-complete-style-zsh) \
          --fish <($out/bin/mad --bpaf-complete-style-fish)
        $out/bin/mad man > mad.1
        installManPage mad.1
      '';
      nativeBuildInputs = [ pkgs.installShellFiles ];
    }
  );
in
{
  inherit mad;
}
