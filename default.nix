{ pkgs ? import <nixpkgs> {} }:

pkgs.rustPlatform.buildRustPackage {
  pname = "mrfetch";
  version = "2.0.0";

  src = builtins.fetchGit {
    url = "https://git.xorycode.dev/Xorycode/mrfetch";
  };

  cargoSha256 = "sha256-halN9q+8jKVRqBasewuVReAajs1FQ9ckbma4F6tB22Y=";

  meta = with pkgs.lib; {
    description = "A small, light nitch-inspired fetch written in Rust.";
    homepage = "https://git.xorycode.dev/Xorycode/mrfetch";
  };
}
