# Maintainer: Donovan Glover <https://donovan.is/>
pkgname=jd
pkgver=0.1.0
pkgrel=1
pkgdesc="A command line interface for interacting with Johnny Decimal systems"
arch=('x86_64')
url="https://github.com/donovanglover/jd"
# license=('')
depends=('gcc-libs')
makedepends=('cargo')
source=("$pkgname-$pkgver.tar.gz::https://github.com/donovanglover/$pkgname/archive/$pkgver.tar.gz")
sha256sums=('SKIP')

build() {
  cd "$pkgname-$pkgver"

  cargo build --release --locked
}

package() {
  cd "$pkgname-$pkgver"

  install -Dm755 "target/release/jd" "$pkgdir/usr/bin/jd"

  install -Dm644 "target/completions/_jd" "$pkgdir/usr/share/zsh/site-functions/_jd"
  install -Dm644 "target/completions/jd.bash" "$pkgdir/usr/share/bash-completion/completions/jd"
  install -Dm644 "target/completions/jd.fish" "$pkgdir/usr/share/fish/vendor_completions.d/jd.fish"
  install -Dm644 "target/man/jd.1" "$pkgdir/usr/share/man/man1/jd.1"

  install -Dm644 "README.md" "$pkgdir/usr/share/doc/$pkgname/README.md"
  # install -Dm644 "LICENSE" "$pkgdir/usr/share/licenses/$pkgname/LICENSE"
}
