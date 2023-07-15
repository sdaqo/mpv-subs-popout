# Maintainer: sdaqo <sdaqo.dev@protonmail.com>

pkgname=mpv-subs-popout
pkgver=0.2.0
pkgrel=1
pkgdesc="A package to enable subtitles popout feature in mpv media player"
arch=('x86_64')
url="https://github.com/sdaqo/mpv-subs-popout"

depends=('cairo>=1.10.0' 'glib2>=2.53' 'gtk3>=3.16.2' 'pango')

source=(https://github.com/sdaqo/mpv-subs-popout/releases/latest/download/mpv-subs-popout_${pkgver}_amd64.deb)

sha512sums=(
    '3d86f442fc6e4690ff40d9c5ce11b83e174554405371350080c7cb3e1df830e02dad48c12c19e0ece7ac752cab5fa8dc010a2462ee6ed935cc67b6af272c0263'   
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
