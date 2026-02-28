# Android / Termux

Jade can run on Android via **Termux** (or similar) using a Linux binary built for the device architecture.

## Installer formats

| Format | Description |
|--------|-------------|
| **.tar.gz** | Tarball of the `jade` binary for Termux. Unpack and put `jade` in `$PREFIX/bin` or `~/bin`. |
| **.apk** | (Future) Standalone APK that includes the Jade binary and a minimal runner. |

## Build for Termux (tarball)

From a machine with Rust and the Android target installed:

```bash
rustup target add aarch64-linux-android   # for most phones
# or: armv7-linux-androideabi for 32-bit ARM

cd jade-lang
./installers/android/build-termux.sh 0.1.0
```

Output: `dist/installers/android/jade-0.1.0-android-aarch64.tar.gz` (or `armv7`).

Copy the tarball to the device, then in Termux:

```bash
tar xzf jade-0.1.0-android-aarch64.tar.gz
mkdir -p ~/bin
mv jade ~/bin/
chmod +x ~/bin/jade
export PATH="$HOME/bin:$PATH"
jade --version
```

## APK (future)

A standalone .apk that installs the Jade binary and optionally launches a terminal could be added (e.g. using Android NDK and a minimal Java/Kotlin wrapper, or a Termux-style approach). Contributions welcome.
