{
  flake,
  toml,
  system,
}:
{
  lib,
  config,
  pkgs,
  ...
}:
let
  inherit (lib)
    mkOption
    types
    mkIf
    mkEnableOption
    ;

  mkSubmoduleOption =
    sub-cfg:
    mkOption {
      default = { };
      type = types.submodule { options = sub-cfg; };
    };

  name = toml.package.name;

  cfg = config.services.${name};
in
{
  options.services.${name} = {
    enable = mkEnableOption "to enable ${name}";

    package = mkOption {
      type = types.package;
      description = "The package to use";
      default = flake.packages.${system}.default;
    };

    user = mkOption {
      type = types.str;
      description = "The user to run as.";
      default = name;
    };

    group = mkOption {
      type = types.str;
      description = "The group to run as.";
      default = name;
    };

    settings = mkSubmoduleOption {
      general = mkSubmoduleOption {
        github_api_token = mkOption {
          type = types.nullOr types.str;
          description = "The github api token to use. Specify a file with '@:path'";
        };

        database_path = mkOption {
          type = types.str;
          description = "The path of the sqlite database";
          default = "/var/lib/${name}/database.db";
        };
      };

      notifications = mkOption {
        type = types.listOf (
          types.submodule {
            options = {
              name = mkOption {
                type = types.str;
                description = "The name of this notification";
              };

              kind = mkOption {
                type = types.enum [ "email" ];
                description = "The notification kind";
              };

              smtp_host = mkOption {
                type = types.str;
                description = "The smtp host. e.g.: mail.example.com";
              };

              smtp_port = mkOption {
                type = types.port;
                description = "The port to use";
                default = 465;
              };

              envelope_from = mkOption {
                type = types.str;
                description = "The email to send from";
              };

              login_username = mkOption {
                type = types.str;
                description = "The name to login";
              };

              login_password = mkOption {
                type = types.str;
                description = "The password to login with. File can be specified with '@:path'";
              };
            };
          }
        );
      };

      subscriptions = mkOption {
        type = types.listOf (
          types.submodule {
            options = {
              via = mkOption {
                type = types.str;
                description = "Via what notification this should be sent. This is the name of a notification.";
              };

              recipient = mkOption {
                type = types.str;
                description = "The recipient to send this notification to";
              };

              feed_name = mkOption {
                type = types.str;
                description = "The name of the feed to listen on";
              };

              packages = mkOption {
                type = types.listOf types.str;
                description = "A list of packages to listen";
              };
            };
          }
        );
      };

      feeds = mkOption {
        type = types.listOf (
          types.submodule {
            options = {
              name = mkOption {
                type = types.str;
                description = "The name of the feed";
              };

              delay_minutes = mkOption {
                type = types.ints.positive;
                description = "The delay to fetch the feed again";
              };

              source = mkOption {
                type = types.enum [
                  "custom"
                  "nixpkgs"
                ];
                description = "The kind of source.";
              };

              kind = mkOption {
                type = types.enum [
                  "github_api"
                  "github_atom"
                ];
                description = "The kind of feed api to use";
              };

              branch = mkOption {
                type = types.nullOr types.str;
                description = "The branch to fetch. If this is null, this is `name`";
              };

              repo_owner = mkOption {
                type = types.nullOr types.str;
                description = "The owner of the repo. If kind == 'custom', you need to set this.";
              };

              repo_name = mkOption {
                type = types.nullOr types.str;
                description = "The name of the repo. If kind == 'custom', you need to set this.";
              };
            };
          }
        );
      };
    };
  };

  config =
    let
      inherit (lib) filterAttrs mapAttrs;

      removeNulls =
        v:
        if builtins.isAttrs v then
          mapAttrs (_: removeNulls) (filterAttrs (_: v: v != null) v)
        else if builtins.isList v then
          builtins.filter (x: x != null) (map removeNulls v)
        else
          v;

      cfgFile = pkgs.writers.writeTOML "config.toml" (removeNulls cfg.settings);
    in
    mkIf (cfg.enable) {
      users.users.${cfg.user} = {
        isSystemUser = true;
        group = cfg.group;
      };

      users.groups.${cfg.group} = { };

      systemd.services.${name} = {
        description = "${name}: notify yourself of changes";
        wantedBy = [ "multi-user.target" ];
        serviceConfig = {
          ExecStart = "${lib.getExe cfg.package} -c ${cfgFile}";
          Restart = "on-failure";

          StateDirectory = "${name}";
          WorkingDirectory = "/var/lib/${name}";
          User = cfg.user;
        };
      };
    };
}
