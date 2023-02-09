let
  pkgs = import <nixpkgs> {  };
in
  with pkgs;
  mkShell {
    buildInputs = [
      pkgconfig
      openssl
      sass
      glib
      cairo
      pango
      atk
      gdk-pixbuf
      libsoup
      gtk3
      webkitgtk
      librsvg
      patchelf
      trunk
      dbus
      nodePackages_latest.pnpm
    ];
    WEBKIT_DISABLE_COMPOSITING_MODE = 1;
}
