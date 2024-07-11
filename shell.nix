{pkgs ? import <nixpkgs> {}}:

with pkgs;
mkShell {
  packages = [cargo];
  buildInputs = [rustc pkg-config];
  nativeBuildInputs = [openssl gtk3];
}
