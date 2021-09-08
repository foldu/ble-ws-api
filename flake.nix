{
  description = "A very basic flake";

  inputs = {
    flake-utils.url = "github:numtide/flake-utils";
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    naersk = {
      url = "github:nmattia/naersk";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, flake-utils, naersk }: flake-utils.lib.eachDefaultSystem (
    system:
      let
        pkgs = import nixpkgs { inherit system; };
      in
        {
          devShell = pkgs.mkShell {
            nativeBuildInputs = with pkgs; [
              protobuf
            ];
            PROTOC = "${pkgs.protobuf}/bin/protoc";
            PROTOC_INCLUDE = "${pkgs.protobuf}/include";
          };
        }
  );
}
