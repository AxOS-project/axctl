pkgname="axctl"
pkgver=0.2.2
pkgrel=1
pkgdesc="A simple command line tool to control AxOS"
arch=("x86_64")
license=("GPL")
makedepends=("cargo" "rust")

build() {
    cargo build --release --locked
}

package() {
    install -Dm755 ${srcdir}/target/release/axctl -t "${pkgdir}/usr/bin/"
}
