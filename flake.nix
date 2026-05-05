{
  description = "A minimal rust environment";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs"; # nixpkgsのバージョンを統一
    };
  };

  outputs = { self, nixpkgs, rust-overlay }: let
    system = "x86_64-linux";
    
    # rust-overlayを適用したpkgsを準備する
    pkgs = import nixpkgs {
      inherit system;
      overlays = [ (import rust-overlay) ];
    };
    
    name = "term_video";

    # rust-toolchain.toml を読み込んでツールチェーンを定義する
    rustToolchain = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;

  in {
    devShells.${system}.default = pkgs.mkShell {
      inherit name;
      
      # 個別の cargo や rustc の代わりに、生成したツールチェーンを指定する
      buildInputs = [
        rustToolchain
      ];

      # RUST_SRC_PATHをオーバーレイで生成された環境に合わせる
      env.RUST_SRC_PATH = "${rustToolchain}/lib/rustlib/src/rust/library";
    };
  };
}
