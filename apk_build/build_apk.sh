#!/data/data/com.termux/files/usr/bin/bash
set -e

cargo build --release -p pixie-app
mkdir -p apk_build/lib/arm64-v8a
cp target/release/libpixie_app.so apk_build/lib/arm64-v8a/

cd apk_build
aapt package -f -M AndroidManifest.xml -I android.jar -F pixie-unsigned.apk
aapt add pixie-unsigned.apk lib/arm64-v8a/libpixie_app.so

if [ ! -f debug.keystore ]; then
    keytool -genkeypair -v -keystore debug.keystore -storepass android \
        -alias androiddebugkey -keypass android -keyalg RSA -keysize 2048 \
        -validity 10000 -dname "CN=Pixie Debug,O=Pixie,C=US"
fi

apksigner sign --ks debug.keystore --ks-pass pass:android \
    --out pixie.apk pixie-unsigned.apk

echo "Built: apk_build/pixie.apk"
echo "Install with: termux-open apk_build/pixie.apk"
