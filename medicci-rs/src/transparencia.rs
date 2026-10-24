//! En Windows, las ventanas del chat y del resumen ampliado (ventanas
//! secundarias de egui con glow) no salen transparentes: lo que no se pinta
//! sale negro. Se veía un recuadro negro donde estaba el chat al plegarlo y un
//! marco negro alrededor de los paneles (el sitio de la sombra).
//!
//! Se arregla haciéndolas ventanas «en capas» con el negro puro como color
//! clave: lo negro no se dibuja y deja pasar los clics. En la paleta de
//! Medicci no hay negro puro, así que no desaparece nada de la interfaz.
//! winit vuelve a tocar el estilo al cambiar si la ventana deja pasar los
//! clics, así que se comprueba en cada fotograma.

/// Pone el negro transparente en las ventanas de este programa con alguno de
/// esos títulos. Devuelve cuántas ha encontrado.
#[cfg(windows)]
pub fn negro_transparente(titulos: &[&str]) -> usize {
    use win::*;
    unsafe extern "system" fn una(h: Hwnd, datos: isize) -> i32 {
        let (titulos, n) = &mut *(datos as *mut (&[&str], usize));
        let mut pid = 0u32;
        GetWindowThreadProcessId(h, &mut pid);
        if pid != GetCurrentProcessId() {
            return 1;
        }
        let mut texto = [0u16; 128];
        let largo = GetWindowTextW(h, texto.as_mut_ptr(), texto.len() as i32);
        let titulo = String::from_utf16_lossy(&texto[..largo.max(0) as usize]);
        if titulos.iter().any(|t| *t == titulo) {
            *n += 1;
            let estilo = GetWindowLongPtrW(h, GWL_EXSTYLE);
            if estilo & WS_EX_LAYERED == 0 {
                SetWindowLongPtrW(h, GWL_EXSTYLE, estilo | WS_EX_LAYERED);
            }
            let (mut clave, mut alfa, mut modo) = (0u32, 0u8, 0u32);
            let ya = GetLayeredWindowAttributes(h, &mut clave, &mut alfa, &mut modo) != 0
                && modo & LWA_COLORKEY != 0 && clave == 0;
            if !ya {
                SetLayeredWindowAttributes(h, 0, 255, LWA_COLORKEY);
            }
        }
        1
    }
    let mut datos: (&[&str], usize) = (titulos, 0);
    unsafe {
        EnumWindows(una, &mut datos as *mut (&[&str], usize) as isize);
    }
    datos.1
}

#[cfg(not(windows))]
pub fn negro_transparente(_titulos: &[&str]) -> usize {
    0
}

#[cfg(windows)]
mod win {
    use std::ffi::c_void;
    pub type Hwnd = *mut c_void;
    pub const GWL_EXSTYLE: i32 = -20;
    pub const WS_EX_LAYERED: isize = 0x0008_0000;
    pub const LWA_COLORKEY: u32 = 0x1;
    #[link(name = "user32")]
    extern "system" {
        pub fn EnumWindows(cb: unsafe extern "system" fn(Hwnd, isize) -> i32, l: isize) -> i32;
        pub fn GetWindowTextW(h: Hwnd, texto: *mut u16, n: i32) -> i32;
        pub fn GetWindowThreadProcessId(h: Hwnd, pid: *mut u32) -> u32;
        pub fn GetWindowLongPtrW(h: Hwnd, i: i32) -> isize;
        pub fn SetWindowLongPtrW(h: Hwnd, i: i32, v: isize) -> isize;
        pub fn GetLayeredWindowAttributes(h: Hwnd, clave: *mut u32, alfa: *mut u8, modo: *mut u32) -> i32;
        pub fn SetLayeredWindowAttributes(h: Hwnd, clave: u32, alfa: u8, modo: u32) -> i32;
    }
    #[link(name = "kernel32")]
    extern "system" {
        pub fn GetCurrentProcessId() -> u32;
    }
}
