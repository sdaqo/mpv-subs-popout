# Maintainer: sdaqo <sdaqo.dev@protonmail.com>

pkgname=mpv-subs-popout
pkgver=0.5.3
pkgrel=1
pkgdesc="A package to enable subtitles popout feature in mpv media player with translation features."
arch=('x86_64')
url="https://github.com/sdaqo/mpv-subs-popout"

depends=('cairo>=1.10.0' 'glib2>=2.53' 'gtk3>=3.16.2' 'pango')
source=(https://github.com/sdaqo/mpv-subs-popout/releases/download/v${pkgver}/mpv-subs-popout_${pkgver}-1_amd64.deb)

sha512sums=(
    '760e60e52486c907f0290ff6ef0fa5347857426ce0de83b88c01d02ee796a9c9086138f89fb16d770ad00287c6ed0d6b0d3237d61d6848b8693f6af4ba39b555'   
)

package() {
    cd "${srcdir}"
    tar -xf ${srcdir}/data.tar.xz
    install -m755 -d "${pkgdir}/usr/"
    install -Dm644 "${srcdir}/usr/share/doc/${pkgname}/copyright" "${pkgdir}/usr/share/licenses/${pkgname}/LICENSE"
    rm "${srcdir}/usr/share/doc/${pkgname}/copyright"
    cp -r "${srcdir}/usr/" "${pkgdir}"
    install -Dm755 "${srcdir}/usr/bin/${pkgname}" "${pkgdir}/usr/bin/${pkgname}"
}
