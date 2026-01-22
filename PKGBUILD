# Maintainer: Stando Contributors
pkgname=stando
pkgver=0.1.0
pkgrel=1
pkgdesc="A next-gen selection based search system for Linux"
arch=('x86_64')
url="https://github.com/stando/stando"
license=('MIT')
depends=('gtk4' 'libadwaita' 'pkg-config')
makedepends=('rust' 'cargo')
source=("$pkgname-$pkgver.tar.gz")
sha256sums=('SKIP')

build() {
    cd "$srcdir/$pkgname-$pkgver"
    cargo build --release
}

package() {
    cd "$srcdir/$pkgname-$pkgver"
    
    # Install binary
    install -Dm755 target/release/stando "$pkgdir/usr/bin/stando"
    
    # Install desktop file
    install -Dm644 com.stando.App.desktop "$pkgdir/usr/share/applications/com.stando.App.desktop"
    
    # Install icon (if available)
    # install -Dm644 assets/icon.png "$pkgdir/usr/share/pixmaps/stando.png"
}
