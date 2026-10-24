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
    ventanas().iter().find_map(|(_, t)| nhc_en(patron, t))
}

/// Dónde va la burbuja: la zona útil (sin la barra de tareas) del monitor en
/// el que está HCIS, en píxeles de verdad [izquierda, arriba, derecha, abajo].
/// Sin HCIS abierto, la del monitor principal. Con dos pantallas, así la
/// burbuja está siempre junto a HCIS.
#[cfg(windows)]
pub fn zona(patron: Option<&Regex>) -> Option<[i32; 4]> {
    let hcis = patron.and_then(|p| ventanas().into_iter().find(|(_, t)| p.is_match(t)));
    let mut info = win::InfoMonitor { tam: std::mem::size_of::<win::InfoMonitor>() as u32,
                                      ..Default::default() };
    unsafe {
        let monitor = match hcis {
            Some((h, _)) => win::MonitorFromWindow(h as win::Hwnd, win::AL_PRINCIPAL),
            None => win::MonitorFromPoint(win::Punto { x: 0, y: 0 }, win::AL_PRINCIPAL),
        };
        if monitor.is_null() || win::GetMonitorInfoW(monitor, &mut info) == 0 {
            return None;
        }
    }
    let r = info.trabajo;
    Some([r.izq, r.arr, r.der, r.aba])
}

#[cfg(not(windows))]
pub fn zona(_patron: Option<&Regex>) -> Option<[i32; 4]> {
    None
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
        pub fn MonitorFromWindow(h: Hwnd, si_no: u32) -> *mut c_void;
        pub fn MonitorFromPoint(p: Punto, si_no: u32) -> *mut c_void;
        pub fn GetMonitorInfoW(m: *mut c_void, info: *mut InfoMonitor) -> i32;
    }
    pub const AL_PRINCIPAL: u32 = 1;            // MONITOR_DEFAULTTOPRIMARY
    #[repr(C)]
    pub struct Punto {
        pub x: i32,
        pub y: i32,
    }
    #[repr(C)]
    #[derive(Default, Clone, Copy)]
    pub struct Rect {
        pub izq: i32,
        pub arr: i32,
        pub der: i32,
        pub aba: i32,
    }
    #[repr(C)]
    #[derive(Default)]
    pub struct InfoMonitor {
        pub tam: u32,
        pub monitor: Rect,
        pub trabajo: Rect,
        pub banderas: u32,
    }
}

/// Las ventanas visibles con título, de la de encima a la de abajo.
#[cfg(windows)]
fn ventanas() -> Vec<(isize, String)> {
    unsafe extern "system" fn una(h: win::Hwnd, lista: isize) -> i32 {
        let lista = &mut *(lista as *mut Vec<(isize, String)>);
        if win::IsWindowVisible(h) != 0 {
            let mut texto = [0u16; 512];
            let n = win::GetWindowTextW(h, texto.as_mut_ptr(), texto.len() as i32);
            if n > 0 {
                lista.push((h as isize, String::from_utf16_lossy(&texto[..n as usize])));
            }
        }
        1
    }
    let mut lista: Vec<(isize, String)> = Vec::new();
    unsafe {
        win::EnumWindows(una, &mut lista as *mut Vec<(isize, String)> as isize);
    }
    lista
}

/// Fuera de Windows no se mira: el médico escribe el NHC.
#[cfg(not(windows))]
fn ventanas() -> Vec<(isize, String)> {
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
    fn windows_da_titulos_y_la_zona_del_monitor() {
        assert!(!ventanas().is_empty());
        let [izq, arr, der, aba] = zona(None).expect("hay monitor principal");
        assert!(der > izq && aba > arr);
    }
}
