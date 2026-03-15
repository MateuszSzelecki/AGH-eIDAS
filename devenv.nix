{
  pkgs,
  lib,
  config,
  inputs,
  ...
}: {
  # https://devenv.sh/packages/
  packages = [pkgs.nodejs pkgs.pnpm pkgs.nest-cli];
}
