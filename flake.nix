{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    microvm.url = "github:microvm-nix/microvm.nix";
    microvm.inputs.nixpkgs.follows = "nixpkgs";
    rust-overlay.url = "github:oxalica/rust-overlay";
    rust-overlay.inputs.nixpkgs.follows = "nixpkgs";
  };

  outputs =
    {
      self,
      nixpkgs,
      microvm,
      rust-overlay,
      ...
    }:
    {
      nixosConfigurations.agent = nixpkgs.lib.nixosSystem {
        system = "x86_64-linux";

        modules = [
          microvm.nixosModules.microvm

          ({ config, lib, pkgs, ... }: {
            microvm = {
              hypervisor = "qemu";
              vcpu = 4;
              mem = 8192;

              interfaces = [
                {
                  type = "user";
                  id = "eth0";
                  mac = "02:00:00:00:00:01";
                }
              ];

              forwardPorts = [
                {
                  from = "host";
                  host.address = "127.0.0.1";
                  host.port = 2222;
                  guest.port = 22;
                }
              ];

              writableStoreOverlay = "/nix/.rw-store";

              volumes = [
                {
                  image = "nix-store-overlay.img";
                  mountPoint = "/nix/.rw-store";
                  size = 8192;
                }
                {
                  image = "workspace.img";
                  mountPoint = "/workspace";
                  size = 16384;
                }
              ];

              shares = [
                {
                  proto = "9p";
                  tag = "repo";
                  source = ".";
                  mountPoint = "/mnt/repo";
                  readOnly = true;
                }
              ];
            };

            users.users.agent = {
              isNormalUser = true;
              extraGroups = [ "wheel" ];
              initialPassword = "agent";
            };

            systemd.tmpfiles.rules = [ "d /workspace 0755 agent users -" ];

            systemd.services.export-repo = {
              wantedBy = [ "multi-user.target" ];
              unitConfig.RequiresMountsFor = [ "/workspace" "/mnt/repo" ];
              serviceConfig = {
                Type = "oneshot";
                User = "agent";
              };
              path = [ pkgs.git pkgs.gnutar ];
              script = ''
                [ -d /workspace/surus ] || {
                  mkdir -p /workspace/surus
                  git -C /mnt/repo archive HEAD | tar -x -C /workspace/surus
                }
              '';
            };

            security.sudo.wheelNeedsPassword = false;

            nixpkgs.config.allowUnfreePredicate = pkg: lib.getName pkg == "claude-code";
            nixpkgs.overlays = [ rust-overlay.overlays.default ];

            environment.systemPackages = with pkgs; [
              git
              curl
              vim
              claude-code
              codex
              pi-coding-agent
              (rust-bin.fromRustupToolchainFile ./rust-toolchain.toml)
              gcc
            ];

            services.postgresql = {
              enable = true;
              package = pkgs.postgresql_18;
              ensureUsers = [
                {
                  name = "agent";
                  ensureClauses.superuser = true;
                }
              ];
              ensureDatabases = [ "agent" ];
            };

            services.openssh.enable = true;
          })
        ];
      };
    };
}
