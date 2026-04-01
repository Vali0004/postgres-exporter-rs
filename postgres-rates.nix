{ lib
, rustPlatform }:

rustPlatform.buildRustPackage rec {
  pname = "postgres-rates";
  version = "0.1.0";

  src = ./.;

  cargoLock = {
    lockFile = ./Cargo.lock;
  };

  meta = with lib; {
    description = "Prometheus postgres connection exporter";
    license = licenses.mit;
    platforms = platforms.linux;
  };
}
