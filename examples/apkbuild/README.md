# Result

```bash
./test.sh
```

APKBUILD file:

```APKBUILD
pkgname=jintemplify
pkgver=0.1.8
pkgrel=0
pkgdesc="Template tool generating formats using Jinja2 & YAML"
url=https://github.com/fantajeon/jintemplify
arch="all"
license=MIT
depends=""
makedepends="cargo"
install=""
subpackages=""
#source="$pkgname-$pkgver::git+https://github.com/fantajeon/jintemplify#tag=v$pkgver"

build() {
    cp -a /workspace/* .
    cargo clean
    cargo build --release
}

package() {
    echo "try package $pkgdir"
    install -Dm755 target/release/jintemplify "$pkgdir/usr/bin/jintemplify"
}

sha512sums="SKIP"
```

There is an actual example at apline_build in .github/workflows/main.yaml for CI.
