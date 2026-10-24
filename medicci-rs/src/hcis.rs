//! Qué paciente tiene abierto el médico en HCIS, leído del título de su ventana.
//!
//! HCIS pone el NHC del paciente abierto en el título («HCIS · P0129798 ·
//! MARÍA ALONSO»), sea una página en el navegador o un programa de Windows.
//! La burbuja lo lee cada segundo y cambia de paciente sola: el médico no
//! escribe nada y HCIS no se toca. Si el HCIS del hospital lo pone de otra
//! manera, se cambia el patrón con MEDICCI_TITULO en medicci.conf.

use regex::Regex;

/// HCIS, y en el mismo título un NHC: «P» o «G» y siete cifras.
pub const PATRON: &str = r"HCIS\b.*?\b([PG]\d{7})\b";

/// El NHC que hay en un título, si es de HCIS.
pub fn nhc_en(patron: &Regex, titulo: &str) -> Option<String> {
    patron.captures(titulo).and_then(|c| c.get(1)).map(|m| m.as_str().to_string())
}

/// El paciente de la ventana de HCIS que está más arriba: la última que ha
/// usado el médico (Windows las da de la de encima a la de abajo).
pub fn paciente_abierto(patron: &Regex) -> Option<String> {
    titulos().iter().find_map(|t| nhc_en(patron, t))
}

#[cfg(windows)]
mod win {
    use std::ffi::c_void;
    pub type Hwnd = *mut c_void;
    #[link(name = "user32")]
    extern "system" {
        pub fn EnumWindows(cb: unsafe extern "system" fn(Hwnd, isize) -> i32, l: isize) -> i32;
        pub fn GetWindowTextW(h: Hwnd, texto: *mut u16, n: i32) -> i32;
        pub fn IsWindowVisible(h: Hwnd) -> i32;
    }
}

/// Los títulos de las ventanas visibles, de la de encima a la de abajo.
#[cfg(windows)]
fn titulos() -> Vec<String> {
    unsafe extern "system" fn una(h: win::Hwnd, lista: isize) -> i32 {
        let lista = &mut *(lista as *mut Vec<String>);
        if win::IsWindowVisible(h) != 0 {
            let mut texto = [0u16; 512];
            let n = win::GetWindowTextW(h, texto.as_mut_ptr(), texto.len() as i32);
            if n > 0 {
                lista.push(String::from_utf16_lossy(&texto[..n as usize]));
            }
        }
        1
    }
    let mut lista: Vec<String> = Vec::new();
    unsafe {
        win::EnumWindows(una, &mut lista as *mut Vec<String> as isize);
    }
    lista
}

/// Fuera de Windows no se mira: el médico escribe el NHC.
#[cfg(not(windows))]
fn titulos() -> Vec<String> {
    Vec::new()
}

#[cfg(test)]
mod pruebas {
    use super::*;

    #[test]
    fn el_nhc_sale_del_titulo_de_hcis() {
        let p = Regex::new(PATRON).unwrap();
        assert_eq!(nhc_en(&p, "HCIS · P0129798 · MARÍA ALONSO - Google Chrome").as_deref(), Some("P0129798"));
        assert_eq!(nhc_en(&p, "HCIS · G0000199 · ANTONIO VEGA").as_deref(), Some("G0000199"));
        assert_eq!(nhc_en(&p, "HCIS · Consultas externas - Google Chrome"), None);
        assert_eq!(nhc_en(&p, "Correo · P0129798"), None, "solo de HCIS");
    }

    #[test]
    #[cfg(windows)]
    fn windows_da_titulos() {
        assert!(!titulos().is_empty());
    }
}
