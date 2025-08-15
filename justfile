build:
  cargo run -p xtask -- bundle wave_compositor --release
  rm -rf ~/.vst/wave_compositor.vst3
  cp -r target/bundled/wave_compositor.vst3 ~/.vst/wave_compositor.vst3

build-bin:
  cargo build wave_compositor --release
