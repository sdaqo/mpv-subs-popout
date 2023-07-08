# Maintainer: Your Name <your_email@example.com>

pkgname=mpv-subs-popout
pkgver=0.1.0
pkgrel=1
pkgdesc="A package to enable subtitles popout feature in mpv media player"
arch=('x86_64')
url="https://github.com/sdaqo/mpv-subs-popout"

depends=('cairo>=1.10.0' 'glib2>=2.53' 'gtk3>=3.16.2' 'pango')

source=(https://github.com/sdaqo/mpv-subs-popout/releases/latest/download/mpv-subs-popout_${pkgver}_amd64.deb)

sha512sums=(
    '6a0c31a0e1d25b05b50899deda95d0d388adff885ee9ccca57073f2b5affc8381e33e7ff0b26ebcf89221803863b90ee7a08684987bd9cef18db432533ec7965'   
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
