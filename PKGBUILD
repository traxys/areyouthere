pkgname="areyouthere"
pkgver=0.1.0
pkgrel=1
pkgdesc="An health server to do things"
url="https://github.com/traxys/areyouthere"
arch=("any")
license=("GPL")
makedepends=("rust" "git")
optdepends=("systemd: using the service file")
depends=()
source=('git+https://github.com/traxys/areyouthere.git')
sha256sums=('SKIP')

build() {
	cd "${srcdir}/$pkgname"
	cargo build --release
}
package() {
	cd "${srcdir}/areyouthere"
	install -Dm 755 "target/release/$pkgname" "$pkgdir/usr/bin/$pkgname"
	install -Dm 644 "areyouthere.service" "${pkgdir}/etc/systemd/system/areyouthere.service"
}
