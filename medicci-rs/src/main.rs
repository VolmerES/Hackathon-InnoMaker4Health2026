//! Medicci · lector de historia clínica.
//!
//! Una burbuja con un médico, siempre abajo a la derecha y encima del HCE.
//! Al pulsarla se despliega el chat: arriba se escribe el número del
//! paciente y abajo se pregunta. Resumen pone en el chat una tarjeta con
//! pestañas; Ampliar la abre en una ventana grande, en mitad de la pantalla.
//!
//!     medicci                       # abre con la base de al lado
//!     medicci --db ruta.db --paciente 417
//!     medicci --servidor http://localhost:8000 --paciente P0129798
//!
//! Con --servidor, la burbuja es la otra cara del servidor de Medicci: la
//! tarjeta «antes de que entre» y las respuestas de la IA local llegan de allí.
//! Si el servidor no responde, sigue sola con su base.

#![cfg_attr(windows, windows_subsystem = "windows")]

mod datos;
mod lector;
mod servidor;

use datos::{CampoFicha, Datos, Documento, Hecho, Paciente};
use eframe::egui;
use egui::emath::{easing, TSTransform};
use egui::{pos2, vec2, Align, Color32, FontFamily, FontId, Layout, Pos2, Rect, RichText,
           Rounding, Sense, Shape, Stroke, UiBuilder, Vec2};
use lector::{Lector, Respuesta, ValorHallado};
use servidor::{FilaRed, Llega, Servidor};
use std::path::PathBuf;

// medidas, en puntos
const BOLA: f32 = 56.0;                     // diámetro de la burbuja
const PANEL: Vec2 = vec2(420.0, 640.0);     // el chat desplegado
const HUECO: f32 = 12.0;                    // entre el chat y la burbuja
const AIRE: f32 = 16.0;                     // borde transparente donde cabe la sombra
const MARGEN: f32 = 24.0;                   // de la burbuja al borde derecho
#[cfg(windows)]
const BAJO: f32 = 72.0;                     // de la burbuja al borde de abajo (barra de tareas)
#[cfg(not(windows))]
const BAJO: f32 = 24.0;
const HOJA: Vec2 = vec2(1240.0, 780.0);     // la ventana del resumen ampliado
const PIE: f32 = 104.0;                     // acciones rápidas y redactor, al pie del chat
/// Cada cuánto se pinta durante una animación (sin vsync, ~60 por segundo).
const FOTOGRAMA: std::time::Duration = std::time::Duration::from_millis(16);

// la paleta de Medicci
const VERDE: Color32 = Color32::from_rgb(0x00, 0x9b, 0x6b);
const VERDE_OSC: Color32 = Color32::from_rgb(0x00, 0x70, 0x4d);
const VERDE_2: Color32 = Color32::from_rgb(0x0a, 0x6b, 0x4a);
const VERDE_CLARO: Color32 = Color32::from_rgb(0xbf, 0xe6, 0xd3);
const FONDO: Color32 = Color32::from_rgb(0xf5, 0xf5, 0xf5);
const BLANCO: Color32 = Color32::from_rgb(0xff, 0xff, 0xff);
const LINEA: Color32 = Color32::from_rgb(0xd7, 0xd7, 0xd7);
const LINEA_SUAVE: Color32 = Color32::from_rgb(0xec, 0xec, 0xec);
const ORO: Color32 = Color32::from_rgb(0xf3, 0xc4, 0x18);
const ORO_SUAVE: Color32 = Color32::from_rgb(0xff, 0xf8, 0xe1);
const ROJO: Color32 = Color32::from_rgb(0xa0, 0x1b, 0x1b);
const GRIS: Color32 = Color32::from_rgb(0x6e, 0x74, 0x77);
const TINTA: Color32 = Color32::from_rgb(0x22, 0x22, 0x22);
const TINTA_2: Color32 = Color32::from_rgb(0x44, 0x4a, 0x4d);
const MIO: Color32 = Color32::from_rgb(0xe5, 0xf1, 0xe9);
const CITA_FONDO: Color32 = Color32::from_rgb(0xff, 0xfb, 0xea);
const FICHA_FONDO: Color32 = Color32::from_rgb(0xf3, 0xf3, 0xf3);

// colores con significado: cada tipo de dato, el suyo, y siempre con título
const ALERTA: Color32 = Color32::from_rgb(0x9b, 0x1c, 0x1c);
const ALERTA_TITULO: Color32 = Color32::from_rgb(0x7a, 0x14, 0x14);
const ALERTA_FONDO: Color32 = Color32::from_rgb(0xfd, 0xed, 0xed);
const ALERTA_BORDE: Color32 = Color32::from_rgb(0xf3, 0xc4, 0xc4);
const CHOQUE: Color32 = Color32::from_rgb(0x8a, 0x4b, 0x00);       // no coincide con la derivación
const CHOQUE_TACHADO: Color32 = Color32::from_rgb(0x6b, 0x55, 0x36);
const CHOQUE_FONDO: Color32 = Color32::from_rgb(0xff, 0xf6, 0xe8);
const CHOQUE_BORDE: Color32 = Color32::from_rgb(0xf1, 0xd3, 0xa8);
const PENDIENTE: Color32 = Color32::from_rgb(0x1e, 0x4e, 0x8c);
const PENDIENTE_FONDO: Color32 = Color32::from_rgb(0xee, 0xf4, 0xfc);
const PENDIENTE_BORDE: Color32 = Color32::from_rgb(0xc9, 0xda, 0xf0);
const NO_CONSTA_FONDO: Color32 = Color32::from_rgb(0xf7, 0xf8, 0xfa);
const NO_CONSTA_BORDE: Color32 = Color32::from_rgb(0x9a, 0xa6, 0xb2);

/// Las pestañas de la tarjeta de resumen del chat.
#[derive(Clone, Copy, PartialEq)]
enum Pestana {
    Resumen,
    Historial,
    Pruebas,
    Medicacion,
}

/// Lo que se pulsa dentro de la tarjeta de resumen o de la ventana ampliada.
enum Accion {
    Pestana(Pestana),
    Ampliar,
    Tema(String),
}

/// Cada trozo de un mensaje del chat.
enum Parte {
    Texto(String),
    Valor(String),
    Cita(String),
    Fuente(String),
    Aviso(String),
    Rotulo(String),
}

struct Mensaje {
    mio: bool,
    partes: Vec<Parte>,
    tarjeta: bool,                  // en vez de partes, la tarjeta de resumen
}

impl Mensaje {
    fn mio(t: impl Into<String>) -> Self {
        Self { mio: true, partes: vec![Parte::Texto(t.into())], tarjeta: false }
    }
    fn suyo(partes: Vec<Parte>) -> Self {
        Self { mio: false, partes, tarjeta: false }
    }
    fn tarjeta() -> Self {
        Self { mio: false, partes: Vec::new(), tarjeta: true }
    }
}

/// Un dato suelto del resumen: lo que se dice, un detalle y de dónde sale.
struct Punto {
    titulo: String,
    detalle: Option<String>,
    documento: Option<String>,
}

/// Una fila de la tabla de analíticas: la penúltima y la última.
struct FilaLab {
    nombre: String,
    unidad: String,
    antes: Option<String>,
    ahora: Option<String>,
}

/// Lo que enseñan la tarjeta del chat y la ventana ampliada, ya leído de la base.
struct Resumen {
    paciente: Paciente,
    documentos: Vec<Documento>,
    desde: String,
    hasta: String,
    campos: Vec<CampoFicha>,
    motivo: Option<CampoFicha>,
    alertas: Vec<Punto>,            // seguridad y datos de alarma: lo primero
    contradicciones: Vec<Hecho>,
    faltas: Vec<Hecho>,
    pruebas: Vec<Hecho>,
    historia: Vec<Hecho>,
    no_consta: Vec<String>,
    marcados: Vec<String>,          // documentos con algo de riesgo: punto rojo en la historia
    labs: Vec<FilaLab>,
    labs_docs: (Option<(String, String)>, Option<(String, String)>),
    hb: Vec<ValorHallado>,
    hechos: Vec<Hecho>,
}

impl Resumen {
    fn de(&self, bloque: &str) -> Vec<&CampoFicha> {
        self.campos.iter().filter(|c| c.bloque == bloque).collect()
    }
    fn de_seccion(&self, seccion: &str) -> Vec<&CampoFicha> {
        self.campos.iter()
            .filter(|c| c.bloque == "por_que_viene" && c.seccion.as_deref() == Some(seccion))
            .collect()
    }
    /// Lo que se le dice al paciente o a quien le acompaña: vida y cabecera destacadas.
    fn para_la_consulta(&self) -> Vec<&CampoFicha> {
        self.campos.iter()
            .filter(|c| c.bloque == "vida" && c.destacado && !sin_dato(&c.valor))
            .collect()
    }
    fn documento(&self, id: &str) -> Option<&Documento> {
        self.documentos.iter().find(|d| d.id == id)
    }
    /// Lo que dice un documento, en una línea: su primer hecho, o su tipo.
    fn linea_de(&self, d: &Documento) -> String {
        self.hechos.iter()
            .find(|h| h.documento.as_deref() == Some(d.id.as_str()))
            .map(|h| cabeza_y_resto(&h.afirmacion).0)
            .unwrap_or_else(|| d.tipo.clone())
    }
}

/// Parte una afirmación en lo principal, que va en negrita, y el resto.
fn cabeza_y_resto(t: &str) -> (String, Option<String>) {
    for sep in [": ", ". "] {
        if let Some((a, b)) = t.split_once(sep) {
            let n = a.chars().count();
            if (12..=90).contains(&n) && !b.trim().is_empty() {
                let a = if sep == ". " { format!("{}.", a) } else { a.to_string() };
                let mut resto = b.trim().chars();
                let resto = resto.next().map(|c| c.to_uppercase().collect::<String>() + resto.as_str());
                return (a, resto);
            }
        }
    }
    (t.to_string(), None)
}

/// Los parámetros de la tabla de analíticas, en este orden.
const TABLA_LAB: &[&str] = &["Hemoglobina", "VCM", "Ferritina", "IST", "Creatinina",
                             "Filtrado glomerular", "PCR", "INR", "GOT", "GPT"];

struct Medicci {
    datos: Datos,
    usuario: String,
    abierto: bool,
    t: f32,                         // 0 = burbuja, 1 = chat desplegado
    marcos: u8,                     // fotogramas de la burbuja, hasta pedir «siempre encima»
    chat_encima: u8,                // ver sube_un_momento
    chat_visto: (bool, u8),         // última visibilidad del chat y fotogramas que faltan
    resumen_visto: (bool, u8),      // lo mismo con el resumen
    chat_subida: f64,
    resumen_al_frente: bool,        // traer delante la ventana del resumen ya abierta
    resumen_encima: u8,             // 0: por subir, 1: subida un momento, 2: ya en su nivel
    resumen_subida: f64,            // cuándo subió
    pedido: Option<(Vec2, Pos2)>,
    intentos: u8,
    espera: u8,
    paciente: Option<Paciente>,
    lector: Option<Lector>,
    hilo: Vec<Mensaje>,
    entrada: String,
    ident: String,
    bajar: bool,
    foco_ident: bool,
    foco_chat: bool,
    resumen: Option<Resumen>,
    pestana: Pestana,               // la pestaña abierta en la tarjeta del chat
    ampliado: bool,                 // la ventana del resumen ampliado, abierta
    copiado: f64,                   // cuándo se copió el resumen a la nota clínica
    resumen_t: f32,
    resumen_pos: Option<Pos2>,
    pendiente: Option<String>,      // un tema pulsado en el resumen
    // conectada al servidor de Medicci (con --servidor)
    red: Option<Servidor>,
    sesion: Option<String>,
    filas: i64,
    pedidos: Vec<Pedido>,           // se mandan en update(), que es donde hay contexto
    esperando: bool,                // el último mensaje es «Buscando…»
    tarjeta_vista: Option<Vec<String>>,
    refresco: f64,
    filas_red: Vec<FilaRed>,            // los datos del paciente, del servidor
    alertas_red: Vec<(String, Vec<String>)>, // la tarjeta del servidor: texto y sus fuentes
    resumen_auto: bool,             // al abrir el paciente, el resumen sale solo
    subir: bool,                    // el chat se enseña desde arriba (el resumen recién abierto)
}

/// Lo que la burbuja le pide al servidor.
enum Pedido {
    Abrir(String),
    Preguntar(String),
    Tarjeta,
    Filas,
    Registro(&'static str),         // lo que hace el médico en pantalla, para el registro
}

/// Quita las referencias del texto («[C3]»): ya van debajo, como fuentes.
fn sin_referencias(t: &str) -> String {
    regex::Regex::new(r"\s*\[[A-Z]\d+\]").map(|re| re.replace_all(t, "").to_string()).unwrap_or_else(|_| t.into())
}

fn recorta(t: &str, max: usize) -> String {
    if t.chars().count() <= max { t.to_string() } else { t.chars().take(max).collect::<String>() + "…" }
}

fn trazo(ancho: f32, color: Color32) -> Stroke {
    Stroke::new(ancho, color)
}

fn negrita() -> FontFamily {
    FontFamily::Name("negrita".into())
}

fn texto(t: impl Into<String>, tam: f32, color: Color32, gruesa: bool) -> RichText {
    let r = RichText::new(t).size(tam).color(color);
    if gruesa { r.family(negrita()) } else { r }
}

fn sin_dato(valor: &str) -> bool {
    let v = valor.to_lowercase();
    v.starts_with("no consta") || v.starts_with("sin registro")
}

fn año(fecha: &str) -> String {
    fecha.chars().take(4).collect()
}

/// Los hechos que aún no han salido en otra sección.
fn nuevos(hechos: Vec<Hecho>, vistos: &mut Vec<String>) -> Vec<Hecho> {
    let fuera: Vec<Hecho> = hechos.into_iter().filter(|h| !vistos.contains(&h.id)).collect();
    vistos.extend(fuera.iter().map(|h| h.id.clone()));
    fuera
}

fn fuente_hecho(h: &Hecho, total: usize) -> String {
    match (&h.documento, &h.fecha_doc) {
        (Some(_), Some(f)) => match &h.servicio {
            Some(s) if !s.is_empty() => format!("{} · {}", s, fecha_es(f)),
            _ => fecha_es(f),
        },
        _ => format!("comprobado: no aparece en ninguno de los {} documentos", total),
    }
}

/// Las letras que se escriben en dos pulsaciones (´ + a = «á», ¨ + u = «ü») llegan
/// del método de entrada (IBus en Linux) como `Ime::Commit`, y egui 0.29 las
/// tira si no le avisó antes de que empezaba a componer. Aquí se convierten en
/// texto normal, que el cuadro siempre acepta. La composición a medias («´») no
/// se enseña: para el castellano no hace falta.
fn letras_compuestas(ctx: &egui::Context) {
    ctx.input_mut(|i| {
        i.events.retain_mut(|e| match e {
            egui::Event::Ime(egui::ImeEvent::Commit(t)) if t != "\n" && t != "\r" && !t.is_empty() => {
                *e = egui::Event::Text(std::mem::take(t));
                true
            }
            egui::Event::Ime(_) => false,
            _ => true,
        })
    });
}

/// En Linux, egui-winit tira todo lo que llega del método de entrada (egui#5008)
/// y, si un cuadro de texto lo pide, Xlib manda por ahí las letras con tilde:
/// «´» + «e» se perdía. Sin método de entrada, winit compone él mismo las
/// teclas muertas y la «é» llega como texto. Va al final de cada fotograma.
fn sin_metodo_de_entrada(ctx: &egui::Context) {
    if cfg!(target_os = "linux") {
        ctx.output_mut(|o| o.ime = None);
    }
}

/// Pone una ventana delante de las demás, también de HCIS o SELENE. El gestor
/// de ventanas no deja que una ventana se dé el foco a sí misma si lo último
/// que se usó es otro programa, así que sube un momento a «siempre encima» y,
/// al poco, vuelve a su nivel: queda delante y otra ventana puede taparla
/// después. `fase`: 0 por subir, 1 subida, 2 ya en su nivel.
fn sube_un_momento(ctx: &egui::Context, fase: &mut u8, desde: &mut f64, otra_vez: bool) {
    let ahora = ctx.input(|i| i.time);
    if otra_vez || *fase == 0 {
        ctx.send_viewport_cmd(egui::ViewportCommand::WindowLevel(egui::WindowLevel::AlwaysOnTop));
        // en Linux, pedir el teclado sin que GNOME lo conceda deja la ventana
        // «pidiendo atención» y GNOME avisa con «“Medicci · Chat” is ready»;
        // allí el teclado llega al pulsar en la ventana
        if !cfg!(target_os = "linux") {
            ctx.send_viewport_cmd(egui::ViewportCommand::Focus);
        }
        *fase = 1;
        *desde = ahora;
    }
    if *fase == 1 {
        if ahora - *desde > 0.4 {
            ctx.send_viewport_cmd(egui::ViewportCommand::WindowLevel(egui::WindowLevel::Normal));
            *fase = 2;
        } else {
            ctx.request_repaint();
        }
    }
}

/// egui aplica «dejar pasar los clics» un fotograma después de pedirlo: al
/// plegar o abrir una ventana se pintan un par de fotogramas más para que no se
/// quede tapando HCIS sin verse.
fn otro_par_de_fotogramas(ctx: &egui::Context, visto: &mut (bool, u8), visible: bool) {
    if visto.0 != visible {
        *visto = (visible, 3);
    }
    if visto.1 > 0 {
        visto.1 -= 1;
        ctx.request_repaint();
    }
}

/// La ventana de la burbuja: siempre del mismo tamaño. El chat va en otra
/// ventana encima; si la burbuja creciera para hacerle sitio, la parte nueva
/// saldría negra un fotograma y la burbuja daría un salto.
fn ventana() -> Vec2 {
    Vec2::splat(BOLA + 2.0 * AIRE)
}

/// La ventana del chat, con su borde para la sombra.
fn ventana_del_chat() -> Vec2 {
    PANEL + Vec2::splat(2.0 * AIRE)
}

impl Medicci {
    fn nuevo(datos: Datos, inicial: Option<String>, red: Option<Servidor>) -> Self {
        let mut app = Self {
            usuario: std::env::var("USER").or_else(|_| std::env::var("USERNAME"))
                .unwrap_or_else(|_| "desconocido".into()),
            datos,
            abierto: false,
            t: 0.0,
            marcos: 0,
            chat_encima: 0,
            chat_visto: (false, 0),
            resumen_visto: (false, 0),
            chat_subida: 0.0,
            resumen_al_frente: false,
            resumen_encima: 0,
            resumen_subida: 0.0,
            pedido: None,
            intentos: 0,
            espera: 0,
            paciente: None,
            lector: None,
            hilo: Vec::new(),
            entrada: String::new(),
            ident: String::new(),
            bajar: false,
            foco_ident: false,
            foco_chat: false,
            resumen: None,
            pestana: Pestana::Resumen,
            ampliado: false,
            copiado: f64::NEG_INFINITY,
            resumen_t: 0.0,
            resumen_pos: None,
            pendiente: None,
            red,
            sesion: None,
            filas: 0,
            pedidos: Vec::new(),
            esperando: false,
            tarjeta_vista: None,
            refresco: 0.0,
            filas_red: Vec::new(),
            alertas_red: Vec::new(),
            resumen_auto: false,
            subir: false,
        };
        app.saludo();
        if let Some(id) = inicial {
            app.abre_paciente(&id);
        }
        app
    }

    fn saludo(&mut self) {
        self.hilo.clear();
        self.hilo.push(Mensaje::suyo(vec![Parte::Texto(if self.red.is_some() {
            "Escriba arriba el NHC del paciente y le digo lo que hay que saber antes de que entre."
        } else {
            "Escriba arriba el número o el ID del paciente y leo su historia antes de que entre."
        }.into())]));
    }

    fn abre_paciente(&mut self, ident: &str) {
        let ident = ident.trim();
        if ident.is_empty() {
            return;
        }
        if self.red.is_some() {
            let nhc = servidor::nhc(ident);
            self.hilo.clear();
            self.hilo.push(Mensaje::suyo(vec![Parte::Rotulo(format!("Abriendo {}…", nhc))]));
            self.pedidos.push(Pedido::Abrir(nhc));
            self.ident.clear();
            self.bajar = true;
            return;
        }
        match self.datos.paciente(ident) {
            Some(p) => {
                let lector = Lector::nuevo(&self.datos, &p.id);
                self.datos.registra(&self.usuario, &p.id, "abrir_chat", None);
                let n = lector.docs.len();
                let desde = lector.docs.last().map(|d| año(&d.fecha)).unwrap_or_default();
                let hasta = lector.docs.first().map(|d| año(&d.fecha)).unwrap_or_default();
                self.hilo.clear();
                self.hilo.push(Mensaje::suyo(vec![
                    Parte::Texto(format!("He leído los {} documentos de {}, de {} a {}.",
                                         n, p.id, desde, hasta)),
                    Parte::Texto("Pulse Resumen para ver lo que hay que saber antes de que \
                                  entre, o pregunte por un dato: «GOT de la última analítica», \
                                  «¿alergias?», «¿qué falta por hacer?».".into()),
                ]));
                self.lector = Some(lector);
                self.paciente = Some(p);
                self.ident.clear();
                self.resumen = None;
                self.ampliado = false;
                self.foco_chat = true;
            }
            None => {
                self.hilo.push(Mensaje::suyo(vec![
                    Parte::Aviso(format!("No hay ningún paciente con «{}».", ident)),
                    Parte::Texto("Pruebe con el número (417) o con el ID completo (PAC-00417).".into()),
                ]));
                self.foco_ident = true;
            }
        }
        self.bajar = true;
    }

    fn cambia_paciente(&mut self) {
        self.paciente = None;
        self.lector = None;
        self.resumen = None;
        self.ampliado = false;
        self.sesion = None;
        self.tarjeta_vista = None;
        self.filas_red.clear();
        self.alertas_red.clear();
        self.esperando = false;
        self.entrada.clear();
        self.saludo();
        self.foco_ident = true;
    }

    /// Resumen: directo a la ventana flotante, sin pasar por el chat. Conectada
    /// y sin los datos aún, enseña la tarjeta «antes de que entre».
    fn resumen_flotante(&mut self) {
        if self.prepara_resumen() {
            self.amplia();
        }
    }

    /// Pone la tarjeta del resumen en el chat (los atajos Pruebas y Medicación).
    fn abre_resumen(&mut self) {
        if !self.prepara_resumen() {
            return;
        }
        // una sola tarjeta en el hilo: la nueva sustituye a la anterior
        self.hilo.retain(|m| !m.tarjeta);
        self.hilo.push(Mensaje::mio("Resumen del paciente"));
        self.hilo.push(Mensaje::tarjeta());
        self.pestana = Pestana::Resumen;
        self.bajar = true;
    }

    /// Deja listo `self.resumen`; dice si lo hay.
    fn prepara_resumen(&mut self) -> bool {
        let Some(p) = self.paciente.clone() else { return false };
        if self.red.is_some() {
            // conectada, el mismo resumen con los datos del servidor
            self.resumen = resumen_de_red(&p, &self.filas_red, &self.alertas_red);
            if self.resumen.is_none() {
                // aún no han llegado los datos: la tarjeta «antes de que entre»
                if self.sesion.is_some() {
                    self.hilo.push(Mensaje::mio("RESUMEN"));
                    self.tarjeta_vista = None;
                    self.pedidos.push(Pedido::Tarjeta);
                    self.bajar = true;
                }
                return false;
            }
        } else {
            if self.lector.is_none() {
                return false;
            }
            self.datos.registra(&self.usuario, &p.id, "generar_resumen", None);
            if self.resumen.as_ref().map(|r| &r.paciente.id) != Some(&p.id) {
                self.resumen = self.lee_resumen(&p);
            }
        }
        self.resumen.is_some()
    }

    /// Lleva la tarjeta del chat a una pestaña; si no hay tarjeta, la saca antes.
    fn muestra(&mut self, pestana: Pestana) {
        if !self.hilo.last().is_some_and(|m| m.tarjeta) {
            self.abre_resumen();
        }
        self.pestana = pestana;
        self.bajar = true;
    }

    fn amplia(&mut self) {
        if self.resumen.is_some() && !self.ampliado {
            if let Some(p) = &self.paciente {
                self.datos.registra(&self.usuario, &p.id, "ampliar_resumen", None);
            }
            if self.red.is_some() {
                self.pedidos.push(Pedido::Registro("ampliar_resumen"));
            }
            self.ampliado = true;
            self.resumen_t = 0.0;
            self.resumen_pos = None;
            self.resumen_encima = 0;
        } else if self.ampliado {
            // ya abierta, quizá detrás de HCIS: delante
            self.resumen_al_frente = true;
            self.resumen_encima = 0;
        }
    }

    /// Lee de la base todo lo que enseñan la tarjeta y la ventana ampliada.
    fn lee_resumen(&self, p: &Paciente) -> Option<Resumen> {
        let l = self.lector.as_ref()?;
        let desde = l.docs.last().map(|d| año(&d.fecha)).unwrap_or_default();
        let hasta = l.docs.first().map(|d| año(&d.fecha)).unwrap_or_default();
        let campos = self.datos.ficha(&p.id);
        let mut vistos = Vec::new();
        let contradicciones = nuevos(self.datos.hechos(&p.id, &["contradiccion"]), &mut vistos);
        let alarmas = nuevos(self.datos.hechos(&p.id, &["alarma"]), &mut vistos);
        let faltas = nuevos(self.datos.hechos(&p.id, &["falta"]), &mut vistos);
        let pruebas = nuevos(self.datos.hechos(&p.id, &["pruebas"]), &mut vistos);
        let historia = nuevos(self.datos.hechos(&p.id, &["resumen"]), &mut vistos);
        let todos = self.datos.hechos(&p.id, &[]);

        // alertas: lo de seguridad que sí consta y destaca, la medicación que hay que
        // revisar y los datos de alarma
        let mut alertas: Vec<Punto> = campos.iter()
            .filter(|c| c.bloque == "antes_de_empezar" && c.destacado && !sin_dato(&c.valor))
            .map(|c| Punto { titulo: format!("{}: {}", c.etiqueta, c.valor),
                             detalle: c.detalle.clone(), documento: c.documento.clone() })
            .collect();
        alertas.extend(campos.iter()
            .filter(|c| c.bloque == "por_que_viene" && c.seccion.as_deref() == Some("medicacion") && c.destacado)
            .map(|c| Punto { titulo: format!("{} {}", c.etiqueta, c.valor),
                             detalle: c.detalle.clone(), documento: c.documento.clone() }));
        alertas.extend(alarmas.iter().map(|h| {
            let (titulo, detalle) = cabeza_y_resto(&h.afirmacion);
            Punto { titulo, detalle, documento: h.documento.clone() }
        }));

        let mut no_consta: Vec<String> = campos.iter()
            .filter(|c| c.bloque == "antes_de_empezar" && sin_dato(&c.valor))
            .map(|c| format!("{}: {}", c.etiqueta, c.detalle.as_deref().unwrap_or(&c.valor)))
            .collect();
        no_consta.extend(todos.iter()
            .filter(|h| h.documento.is_none() && !h.categoria.contains("falta"))
            .map(|h| h.afirmacion.clone()));

        let mut marcados: Vec<String> = todos.iter()
            .filter(|h| ["contradiccion", "alarma", "alergias"].iter().any(|c| h.categoria.contains(c)))
            .filter_map(|h| h.documento.clone())
            .collect();
        marcados.dedup();

        let analiticas = l.analiticas();
        let (ultima, penultima) = (analiticas.first().copied(), analiticas.get(1).copied());
        let labs = TABLA_LAB.iter()
            .filter_map(|n| {
                let ahora = ultima.and_then(|d| l.valor_en(d, n));
                let antes = penultima.and_then(|d| l.valor_en(d, n));
                let unidad = ahora.as_ref().or(antes.as_ref())?.unidad.clone();
                Some(FilaLab { nombre: n.to_string(), unidad,
                               antes: antes.map(|v| v.valor), ahora: ahora.map(|v| v.valor) })
            })
            .collect();
        let doc_fecha = |d: Option<&Documento>| d.map(|d| (d.id.clone(), d.fecha.clone()));

        Some(Resumen {
            paciente: p.clone(),
            documentos: l.docs.clone(),
            desde, hasta,
            motivo: campos.iter()
                .find(|c| c.bloque == "por_que_viene" && c.seccion.as_deref() == Some("motivo"))
                .cloned(),
            campos,
            alertas, contradicciones, faltas, pruebas, historia, no_consta, marcados, labs,
            labs_docs: (doc_fecha(penultima), doc_fecha(ultima)),
            hb: l.serie("Hemoglobina"),
            hechos: todos,
        })
    }

    /// Al registro de accesos y, con su texto, a las preguntas del paciente.
    fn apunta_pregunta(&self, p: &Paciente, pregunta: &str, tipo: &str) {
        self.datos.registra(&self.usuario, &p.id, "preguntar", Some(tipo));
        self.datos.apunta_pregunta(&self.usuario, &p.id, p.id_cita.as_deref(), pregunta, tipo);
    }

    fn preguntar(&mut self) {
        let pregunta = self.entrada.trim().to_string();
        if pregunta.is_empty() {
            return;
        }
        self.entrada.clear();
        let baja = lector::sin_tildes(&pregunta);
        if baja.contains("resumen") || baja.contains("resume") || baja.contains("ficha") {
            return self.resumen_flotante();
        }
        self.hilo.push(Mensaje::mio(pregunta.clone()));
        self.bajar = true;
        if self.red.is_some() {
            if self.sesion.is_some() && !self.esperando {
                self.hilo.push(Mensaje::suyo(vec![Parte::Rotulo("Buscando en sus datos…".into())]));
                self.esperando = true;
                self.pedidos.push(Pedido::Preguntar(pregunta));
            }
            return;
        }
        let (Some(p), Some(l)) = (self.paciente.clone(), self.lector.as_ref()) else { return };
        let total = l.docs.len();
        let mut partes = Vec::new();
        match l.responde(&self.datos, &pregunta) {
            Respuesta::NoConsta => {
                self.apunta_pregunta(&p, &pregunta, "no_consta");
                partes.push(Parte::Aviso("No consta en la historia.".into()));
                partes.push(Parte::Texto(format!(
                    "Ninguna frase de los {} documentos responde a eso, y no lo voy a deducir.",
                    total)));
            }
            Respuesta::Valores(valores) => {
                self.apunta_pregunta(&p, &pregunta, "valor");
                for v in valores {
                    partes.push(Parte::Valor(format!("{} {} {}", v.nombre, v.valor, v.unidad)
                        .trim().to_string()));
                    partes.push(Parte::Texto(format!("{} del {}", v.tipo, v.fecha)));
                    partes.push(Parte::Cita(v.linea));
                    partes.push(Parte::Fuente(v.servicio.clone()));
                }
            }
            Respuesta::Hechos(hechos) => {
                self.apunta_pregunta(&p, &pregunta, "hechos");
                for h in hechos {
                    let t = h.afirmacion.clone();
                    partes.push(if h.categoria.contains("contradiccion") {
                        Parte::Aviso(t)
                    } else {
                        Parte::Texto(t)
                    });
                    if let Some(c) = &h.cita {
                        partes.push(Parte::Cita(c.clone()));
                    }
                    partes.push(Parte::Fuente(fuente_hecho(&h, total)));
                }
            }
            Respuesta::Literal(frases) => {
                self.apunta_pregunta(&p, &pregunta, "literal");
                partes.push(Parte::Rotulo("No lo tengo ordenado. Esto es lo que pone, tal cual:".into()));
                for (doc, frase) in frases {
                    partes.push(Parte::Cita(frase));
                    partes.push(Parte::Fuente(format!("{} del {}", doc.tipo, fecha_es(&doc.fecha))));
                }
            }
        }
        self.hilo.push(Mensaje::suyo(partes));
    }

    /// Manda lo pedido al servidor y recoge lo que ha llegado. Cada 5 s
    /// vuelve a pedir la tarjeta: si llega algo por HL7, la burbuja lo dice.
    fn red(&mut self, ctx: &egui::Context) {
        let pedidos = std::mem::take(&mut self.pedidos);
        let llegadas: Vec<Llega> = {
            let Some(red) = &self.red else { return };
            for p in pedidos {
                match (p, &self.sesion) {
                    (Pedido::Abrir(nhc), _) => red.abrir(ctx, nhc, self.usuario.clone()),
                    (Pedido::Preguntar(t), Some(s)) => red.preguntar(ctx, s.clone(), t),
                    (Pedido::Tarjeta, Some(s)) => red.tarjeta(ctx, s.clone()),
                    (Pedido::Filas, Some(s)) => red.filas(ctx, s.clone()),
                    (Pedido::Registro(a), Some(s)) => red.registra(ctx, s.clone(), a),
                    _ => {}
                }
            }
            if let Some(s) = &self.sesion {
                let ahora = ctx.input(|i| i.time);
                if ahora - self.refresco > 5.0 {
                    self.refresco = ahora;
                    red.tarjeta(ctx, s.clone());
                }
                ctx.request_repaint_after(std::time::Duration::from_secs(5));
            }
            red.rx.try_iter().collect()
        };
        for llegada in llegadas {
            self.recibe(llegada);
        }
    }

    fn recibe(&mut self, llegada: Llega) {
        match llegada {
            Llega::Sesion { sesion, nhc, edad, filas } => {
                self.sesion = Some(sesion);
                self.filas = filas;
                self.paciente = Some(Paciente { id: nhc, edad, ..Default::default() });
                self.hilo.clear();
                self.tarjeta_vista = None;
                self.foco_chat = true;
                self.filas_red.clear();
                self.alertas_red.clear();
                self.resumen = None;
                self.ampliado = false;
                self.resumen_auto = true;
            }
            Llega::Tarjeta { sesion, alertas, mas } => {
                if self.sesion.as_deref() != Some(sesion.as_str()) {
                    return;                                 // es de otro paciente
                }
                let textos: Vec<String> = alertas.iter().map(|a| a.texto.clone()).collect();
                if self.tarjeta_vista.as_ref() == Some(&textos) {
                    return;                                 // no ha cambiado nada
                }
                let novedad = self.tarjeta_vista.is_some();
                self.tarjeta_vista = Some(textos);
                self.alertas_red = alertas.iter()
                    .map(|a| (a.texto.clone(), a.fuentes.iter().map(|f| f.referencia.clone()).collect()))
                    .collect();
                if novedad {
                    self.pedidos.push(Pedido::Filas);   // el resumen también cambia
                } else if self.resumen_auto {
                    return;                             // va dentro del resumen, que llega enseguida
                }
                let mut partes = vec![Parte::Rotulo(if novedad {
                    "Ha llegado algo nuevo de HCIS:".into()
                } else {
                    "ANTES DE QUE ENTRE".into()
                })];
                if alertas.is_empty() {
                    partes.push(Parte::Texto("Nada que destacar en sus datos.".into()));
                }
                for a in alertas {
                    partes.push(if a.alta || a.texto.starts_with("Nuevo:") {
                        Parte::Aviso(a.texto)
                    } else {
                        Parte::Texto(a.texto)
                    });
                    for f in a.fuentes {
                        partes.push(Parte::Fuente(recorta(&f.texto, 110)));
                    }
                }
                if mas > 0 {
                    partes.push(Parte::Rotulo(format!("y {} más: pregunte.", mas)));
                }
                if !novedad {
                    partes.push(Parte::Texto("Pregunte lo que necesite: contesta solo con sus datos \
                                              y cita la fila de la que sale. Resumen lo ordena todo.".into()));
                }
                self.hilo.push(Mensaje::suyo(partes));
                self.bajar = true;
            }
            Llega::Respuesta { texto, consta, fuentes, segundos } => {
                if std::mem::take(&mut self.esperando) {
                    self.hilo.pop();
                }
                let limpio = sin_referencias(&texto);
                let mut partes = vec![if consta { Parte::Texto(limpio) } else { Parte::Aviso(limpio) }];
                let n = fuentes.len();
                for f in fuentes {
                    partes.push(Parte::Fuente(recorta(&f.texto, 110)));
                }
                partes.push(Parte::Rotulo(format!("{} · {:.1} s · IA local",
                    match n { 0 => "sin fuentes".into(), 1 => "1 fuente".into(), _ => format!("{} fuentes", n) },
                    segundos)));
                self.hilo.push(Mensaje::suyo(partes));
                self.bajar = true;
            }
            Llega::Filas { sesion, filas } => {
                if self.sesion.as_deref() != Some(sesion.as_str()) {
                    return;
                }
                self.filas_red = filas;
                if std::mem::take(&mut self.resumen_auto) {
                    // recién abierto: el resumen, con sus pestañas, es lo primero que se ve
                    if let Some(p) = &self.paciente {
                        self.resumen = resumen_de_red(p, &self.filas_red, &self.alertas_red);
                    }
                    if self.resumen.is_some() {
                        self.hilo.push(Mensaje::tarjeta());
                        self.pestana = Pestana::Resumen;
                        self.hilo.push(Mensaje::suyo(vec![Parte::Texto(
                            "Pregunte lo que necesite: contesta solo con sus datos y cita la fila                              de la que sale.".into())]));
                        self.subir = true;              // que se vea desde «Por qué viene hoy»
                    } else {
                        self.tarjeta_vista = None;      // sin datos: la tarjeta de siempre
                        self.pedidos.push(Pedido::Tarjeta);
                    }
                } else if let (Some(_), Some(p)) = (&self.resumen, &self.paciente) {
                    self.resumen = resumen_de_red(p, &self.filas_red, &self.alertas_red);
                }
            }
            Llega::Fallo(e) => {
                self.resumen_auto = false;
                if std::mem::take(&mut self.esperando) {
                    self.hilo.pop();
                }
                self.hilo.push(Mensaje::suyo(vec![Parte::Aviso(e)]));
                self.bajar = true;
            }
        }
    }
}

// =====================================================================
//  Aspecto
// =====================================================================

fn fuentes(ctx: &egui::Context) {
    let mut f = egui::FontDefinitions::default();
    f.font_data.insert("noto".into(),
        egui::FontData::from_static(include_bytes!("../fuentes/NotoSans-Regular.ttf")));
    f.font_data.insert("noto_negrita".into(),
        egui::FontData::from_static(include_bytes!("../fuentes/NotoSans-Bold.ttf")));
    f.font_data.insert("noto_mono".into(),
        egui::FontData::from_static(include_bytes!("../fuentes/NotoSansMono-Regular.ttf")));
    // detrás, la Mono (tiene las flechas que a Noto Sans le faltan: «↑», «→»)
    // y las de egui, para los demás símbolos
    let resto = f.families[&FontFamily::Proportional].clone();
    f.families.get_mut(&FontFamily::Proportional).unwrap().splice(0..0, ["noto".into(), "noto_mono".into()]);
    f.families.get_mut(&FontFamily::Monospace).unwrap().insert(0, "noto_mono".into());
    let mut gruesa = vec!["noto_negrita".to_string(), "noto_mono".to_string()];
    gruesa.extend(resto);
    f.families.insert(negrita(), gruesa);
    ctx.set_fonts(f);
}

fn estilo(ctx: &egui::Context) {
    ctx.set_theme(egui::Theme::Light);
    let mut v = egui::Visuals::light();
    v.panel_fill = Color32::TRANSPARENT;
    v.window_fill = BLANCO;
    v.extreme_bg_color = BLANCO;
    v.faint_bg_color = FONDO;
    v.selection.bg_fill = Color32::from_rgb(0xcd, 0xea, 0xdb);
    v.selection.stroke = trazo(1.0, VERDE);
    v.text_cursor.stroke = trazo(2.0, VERDE);
    v.hyperlink_color = VERDE_2;
    v.window_shadow = egui::epaint::Shadow::NONE;
    v.popup_shadow = egui::epaint::Shadow::NONE;
    v.widgets.noninteractive.fg_stroke.color = TINTA;
    v.widgets.noninteractive.bg_stroke = trazo(1.0, LINEA);
    for w in [&mut v.widgets.inactive, &mut v.widgets.hovered,
              &mut v.widgets.active, &mut v.widgets.open] {
        w.rounding = Rounding::same(8.0);
        w.fg_stroke.color = TINTA;
        w.expansion = 0.0;
    }
    v.widgets.inactive.bg_fill = MIO;
    v.widgets.inactive.weak_bg_fill = MIO;
    v.widgets.inactive.bg_stroke = trazo(1.0, LINEA);
    v.widgets.hovered.bg_fill = MIO;
    v.widgets.hovered.weak_bg_fill = MIO;
    v.widgets.hovered.bg_stroke = trazo(1.0, VERDE_2);
    v.widgets.active.bg_stroke = trazo(1.0, VERDE);
    ctx.set_visuals(v);
    ctx.style_mut(|s| {
        use egui::TextStyle::*;
        s.text_styles = [
            (Small, FontId::proportional(10.5)),
            (Body, FontId::proportional(13.0)),
            (Button, FontId::proportional(13.0)),
            (Heading, FontId::new(18.0, negrita())),
            (Monospace, FontId::monospace(11.5)),
        ].into();
        s.spacing.item_spacing = vec2(8.0, 4.0);
        s.spacing.button_padding = vec2(12.0, 6.0);
        s.spacing.scroll = egui::style::ScrollStyle::floating();
        s.spacing.scroll.bar_width = 6.0;
        s.interaction.selectable_labels = false;
    });
}

/// El logo: un médico con bata y fonendo, dibujado dentro de un círculo de radio `r`.
fn medico(p: &egui::Painter, c: Pos2, r: f32, fondo: Color32, alfa: f32) {
    if alfa <= 0.01 {
        return;
    }
    let k = r / 28.0;
    let blanco = BLANCO.gamma_multiply(alfa);
    let oro = ORO.gamma_multiply(alfa);
    // cabeza
    p.circle_filled(c + vec2(0.0, -7.5 * k), 6.6 * k, blanco);
    // hombros: media elipse
    let base = c + vec2(0.0, 14.5 * k);
    let hombros: Vec<Pos2> = (0..=28)
        .map(|i| {
            let a = std::f32::consts::PI * (1.0 + i as f32 / 28.0);
            base + vec2(13.5 * k * a.cos(), 11.5 * k * a.sin())
        })
        .collect();
    p.add(Shape::convex_polygon(hombros, blanco, Stroke::NONE));
    // cuello de la bata
    p.add(Shape::convex_polygon(
        vec![c + vec2(-4.6 * k, 3.2 * k), c + vec2(4.6 * k, 3.2 * k), c + vec2(0.0, 10.0 * k)],
        fondo.gamma_multiply(alfa), Stroke::NONE));
    // fonendo: el tubo baja por la izquierda y acaba en la campana
    p.add(egui::epaint::CubicBezierShape::from_points_stroke(
        [c + vec2(-6.2 * k, 4.4 * k), c + vec2(-7.6 * k, 13.8 * k),
         c + vec2(2.6 * k, 14.6 * k), c + vec2(6.4 * k, 9.4 * k)],
        false, Color32::TRANSPARENT, trazo(1.8 * k, oro)));
    p.circle_filled(c + vec2(6.6 * k, 9.0 * k), 2.5 * k, oro);
    p.circle_filled(c + vec2(6.6 * k, 9.0 * k), 1.0 * k, blanco);
}

/// Sombra suave hecha con anillos: no necesita nada fuera de la ventana.
fn sombra_redonda(p: &egui::Painter, c: Pos2, r: f32, alfa: f32) {
    for (extra, a) in [(5.0, 10u8), (3.5, 16), (2.0, 24), (0.8, 34)] {
        p.circle_filled(c + vec2(0.0, 2.5), r + extra,
                        Color32::from_black_alpha((a as f32 * alfa) as u8));
    }
}

fn boton(ui: &mut egui::Ui, t: &str, fondo: Color32, tinta: Color32, activo: bool) -> bool {
    let b = egui::Button::new(texto(t, 12.5, tinta, true))
        .fill(fondo)
        .stroke(Stroke::NONE)
        .rounding(16.0)
        .min_size(vec2(0.0, 32.0));
    ui.add_enabled(activo, b).on_hover_cursor(egui::CursorIcon::PointingHand).clicked()
}

fn enlace(ui: &mut egui::Ui, t: &str) -> bool {
    ui.add(egui::Label::new(texto(t, 12.0, VERDE_2, true)).sense(Sense::click()))
        .on_hover_cursor(egui::CursorIcon::PointingHand)
        .clicked()
}

fn cita(ui: &mut egui::Ui, t: &str) {
    let r = egui::Frame::none()
        .fill(CITA_FONDO)
        .rounding(4.0)
        .inner_margin(egui::Margin { left: 10.0, right: 8.0, top: 5.0, bottom: 5.0 })
        .show(ui, |ui| {
            ui.label(RichText::new(format!("«{}»", t)).monospace().size(11.0)
                .color(Color32::from_rgb(0x3a, 0x3f, 0x42)));
        })
        .response;
    ui.painter().vline(r.rect.left() + 1.0, r.rect.y_range(), trazo(2.0, ORO));
}

/// De dónde sale un dato, en el chat: discreto y en palabras, sin códigos.
fn fuente(ui: &mut egui::Ui, t: &str) {
    ui.label(texto(format!("Fuente: {}", t), 11.0, GRIS, false));
}

// =====================================================================
//  La burbuja y el chat
// =====================================================================

impl Medicci {
    /// Mantiene la ventana anclada abajo a la derecha con el tamaño que toca.
    /// El gestor de ventanas puede tardar en hacer caso: se reintenta unas veces.
    fn coloca(&mut self, ctx: &egui::Context) {
        let tam = ventana();
        let (monitor, real) = ctx.input(|i| (i.viewport().monitor_size, i.viewport().outer_rect));
        let Some(monitor) = monitor else {
            ctx.request_repaint();
            return;
        };
        let pos = pos2((monitor.x - MARGEN + AIRE - tam.x).round(),
                       (monitor.y - BAJO + AIRE - tam.y).round());
        if self.pedido != Some((tam, pos)) {
            self.pedido = Some((tam, pos));
            self.intentos = 0;
            self.espera = 0;
        }
        let bien = real.is_some_and(|r| (r.min - pos).length() < 2.0 && (r.size() - tam).length() < 2.0);
        if bien || self.intentos >= 6 {
            return;
        }
        ctx.request_repaint();
        if self.espera > 0 {
            self.espera -= 1;
            return;
        }
        ctx.send_viewport_cmd(egui::ViewportCommand::InnerSize(tam));
        ctx.send_viewport_cmd(egui::ViewportCommand::OuterPosition(pos));
        self.intentos += 1;
        self.espera = 10;
    }

    /// El chat, en su propia ventana justo encima de la burbuja. La ventana
    /// existe desde el arranque: plegado no pinta nada y deja pasar los clics.
    /// Si se creara al abrir, GNOME la tomaría por una ventana que roba el
    /// foco (justo se ha pulsado la burbuja) y avisaría con «… is ready».
    fn ventana_chat(&mut self, ctx: &egui::Context) {
        let visible = self.abierto || self.t > 0.0;
        otro_par_de_fotogramas(ctx, &mut self.chat_visto, visible);
        if !visible {
            self.chat_encima = 0;
        }
        // donde está la burbuja de verdad: el gestor de ventanas puede no
        // dejarla del todo en la esquina (barras del escritorio)
        let Some(bola) = ctx.input(|i| i.viewport().outer_rect) else { return };
        let tam = ventana_del_chat();
        // el borde derecho del chat, con el de la burbuja; abajo, el hueco
        let pos = pos2((bola.right() - tam.x).round(),
                       (bola.top() + 2.0 * AIRE - HUECO - tam.y).round());
        let constructor = egui::ViewportBuilder::default()
            .with_title("Medicci · Chat")
            .with_mouse_passthrough(!visible)
            .with_inner_size(tam)
            .with_position(pos)
            .with_decorations(false)
            .with_transparent(true)
            .with_resizable(false)
            .with_taskbar(false);           // no siempre encima: HCIS o SELENE pueden taparlo
        ctx.show_viewport_immediate(egui::ViewportId::from_hash_of("medicci_chat"), constructor,
                                    |ctx, _clase| {
            if !visible {
                return;                     // plegado: transparente y sin clics
            }
            if ctx.input(|i| i.viewport().close_requested() || i.key_pressed(egui::Key::Escape)) {
                self.abierto = false;
            }
            // al abrirse, delante de HCIS o SELENE
            sube_un_momento(ctx, &mut self.chat_encima, &mut self.chat_subida, false);
            if self.t < 1.0 {
                ctx.request_repaint_after(FOTOGRAMA);
            }
            self.panel(ctx);
        });
    }

    fn burbuja(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default().frame(egui::Frame::none()).show(ctx, |ui| {
            let pantalla = ctx.screen_rect();
            let c = pos2(pantalla.right() - AIRE - BOLA / 2.0, pantalla.bottom() - AIRE - BOLA / 2.0);
            let caja = Rect::from_center_size(c, Vec2::splat(BOLA));
            let r = ui.interact(caja, egui::Id::new("burbuja"), Sense::click())
                .on_hover_cursor(egui::CursorIcon::PointingHand);
            let encima = ctx.animate_bool_with_time(egui::Id::new("burbuja_encima"), r.hovered(), 0.12);
            let radio = BOLA / 2.0 - 1.5 + encima * 1.5;
            let p = ui.painter();
            sombra_redonda(p, c, radio, 1.0);
            p.circle_filled(c, radio, VERDE);
            p.circle_stroke(c, radio - 0.75, trazo(1.5, ORO));
            // el médico se va y aparece la flecha de plegar
            let e = easing::cubic_out(self.t);
            medico(p, c, radio * 0.94, VERDE, 1.0 - e);
            if e > 0.01 {
                let s = trazo(2.4, BLANCO.gamma_multiply(e));
                let d = 7.0;
                p.line_segment([c + vec2(-d, -3.0), c + vec2(0.0, 4.0)], s);
                p.line_segment([c + vec2(0.0, 4.0), c + vec2(d, -3.0)], s);
            }
            if r.clicked() {
                self.abierto = !self.abierto;   // abierto, se pliega; plegado, se abre
                if self.abierto {
                    if self.paciente.is_some() { self.foco_chat = true } else { self.foco_ident = true }
                }
            }
        });
    }

    fn panel(&mut self, ctx: &egui::Context) {
        letras_compuestas(ctx);
        let caja = Rect::from_min_size(pos2(AIRE, AIRE), PANEL);
        // crece desde la burbuja: escala desde la esquina de abajo a la derecha
        let e = easing::cubic_out(self.t);
        let s = 0.84 + 0.16 * e;
        let pivote = caja.right_bottom().to_vec2();
        let capa = egui::LayerId::new(egui::Order::Middle, egui::Id::new("panel"));
        ctx.set_transform_layer(capa, TSTransform::new(pivote * (1.0 - s) + vec2(0.0, (1.0 - e) * 10.0), s));

        egui::Area::new(egui::Id::new("panel"))
            .order(egui::Order::Middle)
            .fixed_pos(caja.min)
            .constrain(false)
            .show(ctx, |ui| {
                ui.set_opacity(e);
                let p = ui.painter().clone();
                p.add(egui::epaint::Shadow { offset: vec2(0.0, 6.0), blur: 20.0, spread: 0.0,
                                             color: Color32::from_black_alpha(55) }
                      .as_shape(caja, 16.0));
                p.rect(caja, 16.0, FONDO, trazo(1.0, LINEA));

                let cabeza = Rect::from_min_size(caja.min, vec2(caja.width(), 64.0));
                let ficha = Rect::from_min_size(cabeza.left_bottom(), vec2(caja.width(), 52.0));
                let pie = Rect::from_min_max(pos2(caja.left(), caja.bottom() - PIE), caja.max);
                let hilo = Rect::from_min_max(ficha.left_bottom(), pie.right_top());
                self.cabecera(ui, cabeza);
                self.barra_paciente(ui, ficha);
                self.conversacion(ui, hilo);
                self.redactor(ui, pie);
            });
        sin_metodo_de_entrada(ctx);
    }

    fn cabecera(&mut self, ui: &mut egui::Ui, caja: Rect) {
        let p = ui.painter().clone();
        p.rect_filled(caja, Rounding { nw: 16.0, ne: 16.0, sw: 0.0, se: 0.0 }, VERDE_OSC);
        let c = pos2(caja.left() + 34.0, caja.center().y);
        p.circle_filled(c, 17.0, VERDE);
        p.circle_stroke(c, 16.5, trazo(1.0, ORO));
        medico(&p, c, 16.0, VERDE, 1.0);
        p.text(pos2(c.x + 28.0, caja.center().y - 9.0), egui::Align2::LEFT_CENTER, "Medicci",
               FontId::new(16.0, negrita()), BLANCO);
        p.text(pos2(c.x + 28.0, caja.center().y + 10.0), egui::Align2::LEFT_CENTER,
               if self.red.is_some() { "IA local · conectada a Medicci" } else { "Lee la historia y cita la fuente" },
               FontId::proportional(11.5), VERDE_CLARO);

        let b = Rect::from_center_size(pos2(caja.right() - 28.0, caja.center().y), Vec2::splat(30.0));
        let r = ui.interact(b, egui::Id::new("plegar"), Sense::click())
            .on_hover_cursor(egui::CursorIcon::PointingHand);
        if r.hovered() {
            p.circle_filled(b.center(), 15.0, Color32::from_white_alpha(22));
        }
        p.line_segment([b.center() - vec2(6.0, 0.0), b.center() + vec2(6.0, 0.0)],
                       trazo(2.0, BLANCO));
        if r.clicked() {
            self.abierto = false;
        }

        // apagar y reiniciar: la ventana no tiene marco, así que van aquí
        let mut apagar = false;
        let mut reiniciar = false;
        for (i, (id, ayuda)) in [("apagar", "Apagar Medicci"), ("reiniciar", "Reiniciar Medicci")]
            .into_iter().enumerate()
        {
            let b = Rect::from_center_size(pos2(caja.right() - 64.0 - 34.0 * i as f32, caja.center().y),
                                           Vec2::splat(30.0));
            let r = ui.interact(b, egui::Id::new(id), Sense::click())
                .on_hover_cursor(egui::CursorIcon::PointingHand)
                .on_hover_text(ayuda);
            if r.hovered() {
                p.circle_filled(b.center(), 15.0, Color32::from_white_alpha(22));
            }
            if i == 0 { icono_apagar(&p, b.center()) } else { icono_reiniciar(&p, b.center()) }
            if r.clicked() {
                if i == 0 { apagar = true } else { reiniciar = true }
            }
        }
        if apagar {
            ui.ctx().send_viewport_cmd_to(egui::ViewportId::ROOT, egui::ViewportCommand::Close);
        }
        if reiniciar {
            self.reinicia(ui.ctx());
        }
    }

    /// Se vuelve a abrir a sí misma, con el mismo paciente, y se cierra.
    fn reinicia(&self, ctx: &egui::Context) {
        let Ok(exe) = std::env::current_exe() else { return };
        let mut args = Vec::new();
        let mut viejos = std::env::args().skip(1);
        while let Some(a) = viejos.next() {
            if a == "--paciente" {
                viejos.next();
            } else if a != "--reinicio" {
                args.push(a);
            }
        }
        if let Some(p) = &self.paciente {
            args.extend(["--paciente".to_string(), p.id.clone()]);
        }
        args.push("--reinicio".into());         // la nueva espera a que esta se cierre
        if std::process::Command::new(exe).args(&args).spawn().is_ok() {
            ctx.send_viewport_cmd_to(egui::ViewportId::ROOT, egui::ViewportCommand::Close);
        }
    }

    fn barra_paciente(&mut self, ui: &mut egui::Ui, caja: Rect) {
        let p = ui.painter().clone();
        p.rect_filled(caja, 0.0, BLANCO);
        p.hline(caja.x_range(), caja.bottom() - 0.5, trazo(1.0, LINEA));
        let mut abrir = false;
        let mut cambiar = false;
        let docs = self.lector.as_ref().map(|l| l.docs.len()).unwrap_or(0);
        ui.allocate_new_ui(UiBuilder::new().max_rect(caja.shrink2(vec2(16.0, 9.0)))
                               .layout(Layout::left_to_right(Align::Center)), |ui| {
            if let Some(pac) = &self.paciente {
                ui.vertical(|ui| {
                    ui.spacing_mut().item_spacing.y = 0.0;
                    ui.label(texto(&pac.id, 13.5, TINTA, true));
                    let detalle = if self.red.is_some() {
                        format!("{} años · {} registros", pac.edad, self.filas)
                    } else {
                        format!("{} años · {} · {} documentos", pac.edad, pac.sexo, docs)
                    };
                    ui.label(texto(detalle, 11.0, GRIS, false));
                });
                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    cambiar = enlace(ui, "Cambiar");
                });
            } else {
                ui.label(texto("Paciente", 12.0, GRIS, true));
                ui.add_space(2.0);
                egui::Frame::none()
                    .fill(FONDO)
                    .stroke(trazo(1.0, LINEA))
                    .rounding(8.0)
                    .inner_margin(egui::Margin::symmetric(10.0, 6.0))
                    .show(ui, |ui| {
                        let pista = if self.red.is_some() { "NHC, p. ej. P0129798" } else { "Número o ID, p. ej. 417" };
                        let campo = egui::TextEdit::singleline(&mut self.ident)
                            .hint_text(texto(pista, 13.0, GRIS, false))
                            .frame(false)
                            .text_color(TINTA)
                            .font(FontId::proportional(13.0))
                            .margin(vec2(0.0, 2.0))
                            .desired_width(ui.available_width() - 76.0);
                        let r = ui.add(campo);
                        if self.foco_ident {
                            r.request_focus();
                            self.foco_ident = false;
                        }
                        if r.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                            abrir = true;
                        }
                    });
                abrir |= boton(ui, "Abrir", VERDE, BLANCO, !self.ident.trim().is_empty());
            }
        });
        if abrir {
            let id = self.ident.clone();
            self.abre_paciente(&id);
        }
        if cambiar {
            self.cambia_paciente();
        }
    }

    fn conversacion(&mut self, ui: &mut egui::Ui, caja: Rect) {
        let bajar = std::mem::take(&mut self.bajar);
        let subir = std::mem::take(&mut self.subir);
        let mut accion = None;
        ui.allocate_new_ui(UiBuilder::new().max_rect(caja.shrink2(vec2(12.0, 0.0))), |ui| {
            let mut hilo = egui::ScrollArea::vertical()
                .id_salt("hilo")
                .auto_shrink([false, false])
                .stick_to_bottom(!subir);
            if subir {
                hilo = hilo.vertical_scroll_offset(0.0);
            }
            hilo.show(ui, |ui| {
                    ui.add_space(12.0);
                    let ancho = ui.available_width();
                    for m in &self.hilo {
                        if m.tarjeta {
                            if let Some(r) = &self.resumen {
                                accion = tarjeta(ui, r, self.pestana).or(accion.take());
                            }
                        } else {
                            mensaje(ui, m, ancho);
                        }
                        ui.add_space(8.0);
                    }
                    if bajar {
                        ui.scroll_to_cursor(Some(Align::BOTTOM));
                    }
                });
        });
        match accion {
            Some(Accion::Pestana(p)) => self.pestana = p,
            Some(Accion::Ampliar) => self.amplia(),
            Some(Accion::Tema(t)) => self.pendiente = Some(t),
            None => {}
        }
    }

    fn redactor(&mut self, ui: &mut egui::Ui, caja: Rect) {
        let p = ui.painter().clone();
        p.rect_filled(caja, Rounding { nw: 0.0, ne: 0.0, sw: 16.0, se: 16.0 }, BLANCO);
        p.hline(caja.x_range(), caja.top() + 0.5, trazo(1.0, LINEA));
        let hay = self.paciente.is_some();
        let mut resumen = false;
        let mut enviar = false;
        let mut ir: Option<Pestana> = None;
        let mut falta = false;
        let dentro = caja.shrink2(vec2(12.0, 12.0));
        let (arriba, abajo) = dentro.split_top_bottom_at_y(dentro.top() + 34.0);
        // acciones rápidas: Resumen es la principal
        ui.allocate_new_ui(UiBuilder::new().max_rect(arriba)
                               .layout(Layout::left_to_right(Align::Center)), |ui| {
            ui.spacing_mut().item_spacing.x = 6.0;
            resumen = boton(ui, "Resumen", ORO, TINTA, hay);
            if rapida(ui, "Pruebas", hay) { ir = Some(Pestana::Pruebas) }
            if rapida(ui, "Medicación", hay) { ir = Some(Pestana::Medicacion) }
            falta = rapida(ui, "¿Qué falta?", hay);
        });
        ui.allocate_new_ui(UiBuilder::new().max_rect(abajo.with_min_y(abajo.top() + 8.0))
                               .layout(Layout::left_to_right(Align::Center)), |ui| {
            egui::Frame::none()
                .fill(FONDO)
                .stroke(trazo(1.0, LINEA))
                .rounding(17.0)
                .inner_margin(egui::Margin { left: 12.0, right: 3.0, top: 3.0, bottom: 3.0 })
                .show(ui, |ui| {
                    ui.horizontal(|ui| {
                        let pista = if hay { "Pregunte a la historia" } else { "Primero, el paciente" };
                        let campo = egui::TextEdit::singleline(&mut self.entrada)
                            .hint_text(texto(pista, 13.0, GRIS, false))
                            .frame(false)
                            .text_color(TINTA)
                            .font(FontId::proportional(13.0))
                            .margin(vec2(0.0, 4.0))
                            .desired_width(ui.available_width() - 28.0 - ui.spacing().item_spacing.x - 4.0);
                        let r = ui.add_enabled(hay, campo);
                        if self.foco_chat && hay {
                            r.request_focus();
                            self.foco_chat = false;
                        }
                        if r.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                            enviar = true;
                            r.request_focus();
                        }
                        let (b, pintor) = ui.allocate_painter(Vec2::splat(28.0), Sense::click());
                        let lleno = hay && !self.entrada.trim().is_empty();
                        let c = b.rect.center();
                        pintor.circle_filled(c, 14.0, if lleno { VERDE } else { LINEA });
                        pintor.add(Shape::convex_polygon(
                            vec![c + vec2(-4.0, -6.0), c + vec2(6.5, 0.0), c + vec2(-4.0, 6.0),
                                 c + vec2(-1.5, 0.0)],
                            BLANCO, Stroke::NONE));
                        if lleno && b.on_hover_cursor(egui::CursorIcon::PointingHand).clicked() {
                            enviar = true;
                        }
                    });
                });
        });
        if resumen {
            self.resumen_flotante();
        }
        if let Some(p) = ir {
            self.muestra(p);
        }
        if falta {
            self.entrada = "¿Qué falta por hacer?".into();
            self.preguntar();
        }
        if enviar {
            self.preguntar();
        }
    }
}

/// Un botón secundario de acción rápida: blanco, con borde.
fn rapida(ui: &mut egui::Ui, t: &str, activo: bool) -> bool {
    let b = egui::Button::new(texto(t, 12.5, TINTA, false))
        .fill(BLANCO)
        .stroke(trazo(1.0, LINEA))
        .rounding(16.0)
        .min_size(vec2(0.0, 32.0));
    ui.add_enabled(activo, b).on_hover_cursor(egui::CursorIcon::PointingHand).clicked()
}

fn mensaje(ui: &mut egui::Ui, m: &Mensaje, ancho: f32) {
    let (fondo, borde, esquinas, alin) = if m.mio {
        (MIO, Color32::TRANSPARENT, Rounding { nw: 14.0, ne: 4.0, sw: 14.0, se: 14.0 }, Align::Max)
    } else {
        (BLANCO, LINEA, Rounding { nw: 4.0, ne: 14.0, sw: 14.0, se: 14.0 }, Align::Min)
    };
    ui.with_layout(Layout::top_down(alin), |ui| {
        egui::Frame::none()
            .fill(fondo)
            .stroke(trazo(1.0, borde))
            .rounding(esquinas)
            .inner_margin(egui::Margin::symmetric(12.0, 9.0))
            .show(ui, |ui| {
                ui.set_max_width(ancho * 0.84);
                ui.with_layout(Layout::top_down(Align::Min), |ui| {
                    ui.spacing_mut().item_spacing.y = 4.0;
                    for parte in &m.partes {
                        match parte {
                            Parte::Texto(t) => {
                                ui.label(texto(t, 13.0, if m.mio { TINTA } else { TINTA_2 }, m.mio));
                            }
                            Parte::Valor(t) => { ui.label(texto(t, 18.0, TINTA, true)); }
                            Parte::Aviso(t) => { ui.label(texto(t, 13.0, ROJO, true)); }
                            Parte::Rotulo(t) => { ui.label(texto(t, 11.5, GRIS, false)); }
                            Parte::Cita(t) => cita(ui, t),
                            Parte::Fuente(t) => {
                                fuente(ui, t);
                                ui.add_space(4.0);
                            }
                        }
                    }
                });
            });
    });
}

// =====================================================================
//  Conectada: el mismo resumen, con las filas que manda el servidor
// =====================================================================

/// Un campo de una fila del servidor, como texto («» si no está).
fn campo(f: &FilaRed, k: &str) -> String {
    match &f.campos[k] {
        serde_json::Value::String(s) => s.clone(),
        serde_json::Value::Null => String::new(),
        v => v.to_string(),
    }
}

/// La fecha de una fila, «2025-04-13»: cuándo pasó o cuándo está prevista.
fn fecha_fila(f: &FilaRed) -> String {
    ["fecha", "realizada", "programada"].iter()
        .map(|k| campo(f, k))
        .find(|v| !v.is_empty())
        .map(|v| v.chars().take(10).collect())
        .unwrap_or_default()
}

fn filas_de<'a>(filas: &'a [FilaRed], tabla: &'a str) -> impl Iterator<Item = &'a FilaRed> + 'a {
    filas.iter().filter(move |f| f.tabla == tabla)
}

/// «MOVILIDAD_REDUCIDA» → «Movilidad reducida».
fn legible(t: &str) -> String {
    let t = t.replace('_', " ").to_lowercase();
    let mut letras = t.chars();
    letras.next().map(|l| l.to_uppercase().collect::<String>() + letras.as_str()).unwrap_or_default()
}

fn prueba_legible(tipo: &str) -> String {
    match tipo {
        "ANALITICA" => "Analítica".into(),
        "ECOGRAFIA" => "Ecografía".into(),
        "RADIOGRAFIA" => "Radiografía".into(),
        "ESPIROMETRIA" => "Espirometría".into(),
        "TAC" | "ECG" => tipo.into(),
        _ => legible(tipo),
    }
}

/// Cuántos documentos van en la historia: un crónico tiene miles de filas.
const HISTORIA_MAX: usize = 120;

/// El resumen de la burbuja hecho con los datos del servidor: cada fila es un
/// documento con su referencia (C3, P1…), la tarjeta da las alertas y lo que
/// el servidor no tiene (medicación, alergias, analíticas) sale como «no consta».
fn resumen_de_red(p: &Paciente, filas: &[FilaRed], alertas: &[(String, Vec<String>)]) -> Option<Resumen> {
    let ficha = filas.iter().find(|f| f.tabla == "PACIENTES")?;
    let mut paciente = p.clone();
    paciente.sexo = match campo(ficha, "sexo").as_str() {
        "F" => "mujer".into(),
        "M" => "hombre".into(),
        s => s.to_lowercase(),
    };
    let hecho = |f: &FilaRed, afirmacion: String| Hecho {
        id: f.referencia.clone(),
        categoria: f.tabla.to_lowercase(),
        afirmacion,
        cita: None,
        documento: Some(f.referencia.clone()),
        fecha_doc: Some(fecha_fila(f)),
        servicio: Some(campo(f, "servicio")),
    };
    let ficha_campo = |bloque: &str, seccion: Option<&str>, etiqueta: &str, valor: String,
                       detalle: Option<String>, doc: &str, destacado: bool| CampoFicha {
        bloque: bloque.into(),
        seccion: seccion.map(Into::into),
        etiqueta: etiqueta.into(),
        valor,
        detalle,
        documento: Some(doc.into()),
        fecha_doc: None,
        destacado,
    };

    // cada fila es un documento; lo más reciente primero
    let mut documentos: Vec<Documento> = filas.iter()
        .filter(|f| f.tabla != "PACIENTES")
        .map(|f| Documento {
            id: f.referencia.clone(),
            fecha: fecha_fila(f),
            tipo: match f.tabla.as_str() {
                "DERIVACIONES_AP" => "Derivación de Primaria",
                "CITAS" => "Cita",
                "MOVIMIENTOS_CITAS" => "Cambio de cita",
                "PRUEBAS" => "Prueba",
                "COMUNICACIONES" => "Aviso al paciente",
                otra => otra,
            }.to_string(),
            servicio: campo(f, "servicio"),
            texto: f.texto.clone(),
        })
        .collect();
    documentos.sort_by(|a, b| b.fecha.cmp(&a.fecha));
    let desde = documentos.last().map(|d| año(&d.fecha)).unwrap_or_default();
    let hasta = documentos.first().map(|d| año(&d.fecha)).unwrap_or_default();
    documentos.truncate(HISTORIA_MAX);
    let hechos: Vec<Hecho> = documentos.iter()
        .map(|d| Hecho {
            id: d.id.clone(),
            categoria: d.tipo.to_lowercase(),
            afirmacion: sin_referencias(&d.texto),
            cita: None,
            documento: Some(d.id.clone()),
            fecha_doc: Some(d.fecha.clone()),
            servicio: Some(d.servicio.clone()),
        })
        .collect();

    // por qué viene: la última derivación de Primaria
    let mut campos = Vec::new();
    match filas_de(filas, "DERIVACIONES_AP").max_by_key(|f| fecha_fila(f)) {
        Some(d) => {
            let obs = campo(d, "observaciones");
            campos.push(ficha_campo("por_que_viene", Some("motivo"), "Motivo",
                format!("{} · {}", campo(d, "motivo"), campo(d, "servicio")),
                Some(format!("Derivación de Primaria del {}, prioridad {}.{}", fecha_es(&fecha_fila(d)),
                             campo(d, "prioridad").to_lowercase(),
                             if obs.is_empty() { String::new() } else { format!(" «{}»", recorta(&obs, 220)) })),
                &d.referencia, false));
        }
        None => paciente.motivo = "Sin derivación de Primaria en sus datos.".into(),
    }
    let (accesibilidad, contacto) = (campo(ficha, "accesibilidad"), campo(ficha, "contacto"));
    let f1 = ficha.referencia.as_str();
    if !paciente.sexo.is_empty() {
        campos.push(ficha_campo("cabecera", None, "Sexo", legible(&paciente.sexo), None, f1, false));
    }
    if !accesibilidad.is_empty() && accesibilidad != "NINGUNA" {
        campos.push(ficha_campo("cabecera", None, "Accesibilidad", legible(&accesibilidad), None, f1, true));
        campos.push(ficha_campo("vida", None, "Accesibilidad", legible(&accesibilidad), None, f1, true));
    }
    if !contacto.is_empty() {
        campos.push(ficha_campo("cabecera", None, "Contacto",
                                format!("Contacto: {}", contacto.to_lowercase()), None, f1, false));
        campos.push(ficha_campo("vida", None, "Prefiere que le contacten por",
                                legible(&contacto), None, f1, true));
    }

    // pendiente: pruebas programadas y citas por venir, la más próxima primero
    let mut pendientes: Vec<&FilaRed> = filas_de(filas, "PRUEBAS")
        .filter(|f| campo(f, "estado") == "PROGRAMADA").collect();
    pendientes.sort_by_key(|f| fecha_fila(f));
    let mut citadas: Vec<&FilaRed> = filas_de(filas, "CITAS").filter(|f| campo(f, "estado") == "CITADA").collect();
    citadas.sort_by_key(|f| fecha_fila(f));
    let mut faltas: Vec<Hecho> = pendientes.iter()
        .map(|f| hecho(f, format!("{} programada para el {}", prueba_legible(&campo(f, "tipo")),
                                  fecha_es(&fecha_fila(f)))))
        .chain(citadas.iter().map(|f| hecho(f, format!("Cita en {} el {}", campo(f, "servicio"),
                                                         fecha_es(&fecha_fila(f))))))
        .collect();
    faltas.truncate(8);

    let mut pruebas: Vec<&FilaRed> = filas_de(filas, "PRUEBAS").collect();
    pruebas.sort_by(|a, b| fecha_fila(b).cmp(&fecha_fila(a)));
    let pruebas: Vec<Hecho> = pruebas.into_iter().take(30)
        .map(|f| {
            let (tipo, fecha) = (prueba_legible(&campo(f, "tipo")), fecha_es(&fecha_fila(f)));
            hecho(f, match campo(f, "estado").as_str() {
                "REALIZADA" => format!("{} · realizada: el {}, {}", tipo, fecha,
                    if f.campos["resultado"].as_bool().unwrap_or(false) { "con el resultado disponible" }
                    else { "sin resultado disponible todavía" }),
                "PROGRAMADA" => format!("{} · pendiente: programada para el {}", tipo, fecha),
                "NO_REALIZADA" => format!("{} · no realizada: estaba programada el {}", tipo, fecha),
                e => format!("{} · {}: {}", tipo, legible(e).to_lowercase(), fecha),
            })
        })
        .collect();

    // qué le ha pasado: las cuentas de toda la historia y los últimos cambios de cita
    let cuenta = |tabla: &str, k: &str, v: &str| filas_de(filas, tabla).filter(|f| campo(f, k) == v).count();
    let mut historia = vec![Hecho {
        id: "cuentas".into(),
        categoria: "resumen".into(),
        afirmacion: format!("{} citas desde {}: {} realizadas y {} cerradas sin atenderle. {} pruebas y {} avisos.",
                            filas_de(filas, "CITAS").count(), desde, cuenta("CITAS", "estado", "REALIZADA"),
                            cuenta("MOVIMIENTOS_CITAS", "tipo", "NO_REALIZADA"),
                            filas_de(filas, "PRUEBAS").count(), filas_de(filas, "COMUNICACIONES").count()),
        cita: None,
        documento: None,
        fecha_doc: None,
        servicio: None,
    }];
    let mut movs: Vec<&FilaRed> = filas_de(filas, "MOVIMIENTOS_CITAS").collect();
    movs.sort_by(|a, b| fecha_fila(b).cmp(&fecha_fila(a)));
    historia.extend(movs.into_iter().take(5).map(|f| hecho(f, sin_referencias(&f.texto))));

    let mut marcados: Vec<String> = alertas.iter().flat_map(|(_, r)| r.iter().cloned()).collect();
    marcados.sort();
    marcados.dedup();

    Some(Resumen {
        paciente,
        documentos,
        desde,
        hasta,
        motivo: campos.iter()
            .find(|c| c.bloque == "por_que_viene" && c.seccion.as_deref() == Some("motivo"))
            .cloned(),
        campos,
        alertas: alertas.iter()
            .map(|(t, r)| Punto { titulo: t.clone(), detalle: None, documento: r.first().cloned() })
            .collect(),
        contradicciones: Vec::new(),
        faltas,
        pruebas,
        historia,
        no_consta: vec![
            "Alergias, diagnósticos y medicación: no vienen en los datos que tiene Medicci.".into(),
            "El valor de los resultados: solo consta si la prueba tiene el resultado disponible.".into(),
        ],
        marcados,
        labs: Vec::new(),
        labs_docs: (None, None),
        hb: Vec::new(),
        hechos,
    })
}

// =====================================================================
//  El resumen: la tarjeta del chat y la ventana ampliada
// =====================================================================

fn columna(ui: &mut egui::Ui, ancho: f32, alin: Align, dentro: impl FnOnce(&mut egui::Ui)) {
    ui.allocate_ui_with_layout(vec2(ancho, 0.0), Layout::top_down(alin), |ui| {
        ui.set_width(ancho);
        ui.spacing_mut().item_spacing.y = 2.0;
        dentro(ui);
    });
}

fn pastilla(ui: &mut egui::Ui, t: &str, destacada: bool) {
    egui::Frame::none()
        .fill(if destacada { ORO_SUAVE } else { MIO })
        .rounding(12.0)
        .inner_margin(egui::Margin::symmetric(10.0, 3.0))
        .show(ui, |ui| {
            ui.label(texto(t, 11.5, TINTA, destacada));
        });
}

/// Una etiqueta pequeña de color: «Revisar», «Evitar…».
fn marca(ui: &mut egui::Ui, t: &str, fondo: Color32, tinta: Color32) {
    egui::Frame::none()
        .fill(fondo)
        .rounding(9.0)
        .inner_margin(egui::Margin::symmetric(8.0, 1.0))
        .show(ui, |ui| {
            ui.label(texto(t, 11.0, tinta, true));
        });
}

/// El título de un bloque, en pequeño y con el color de lo que contiene.
fn rotulo(ui: &mut egui::Ui, t: &str, color: Color32) {
    ui.label(texto(t.to_uppercase(), 10.5, color, true));
}

fn fecha_es(f: &str) -> String {
    let t: Vec<&str> = f.split('-').collect();
    if t.len() == 3 {
        format!("{}/{}/{}", t[2].chars().take(2).collect::<String>(), t[1], t[0])
    } else {
        f.to_string()
    }
}

/// «2019-11-27» → «nov 19».
fn mes_año(fecha: &str) -> String {
    const MESES: [&str; 12] = ["ene", "feb", "mar", "abr", "may", "jun",
                               "jul", "ago", "sep", "oct", "nov", "dic"];
    let mes: usize = fecha.get(5..7).and_then(|m| m.parse().ok()).unwrap_or(0);
    match (MESES.get(mes.wrapping_sub(1)), fecha.get(2..4)) {
        (Some(m), Some(a)) => format!("{} {}", m, a),
        _ => año(fecha),
    }
}

/// El documento dicho como lo diría un médico: «Endoscopia», «Cardiología».
fn nombre_doc(d: &Documento) -> String {
    let tipo = d.tipo.to_lowercase();
    let servicio = d.servicio.split(" · ").next().unwrap_or_default();
    if tipo.contains("endoscopia") {
        "Endoscopia".into()
    } else if tipo.contains("urgencias") {
        "Urgencias".into()
    } else if tipo.contains("quirófano") {
        "Quirófano".into()
    } else if tipo.contains("radiología") {
        "Radiología".into()
    } else if tipo.starts_with("derivación") {
        "Derivación".into()
    } else if tipo.contains("consulta") && !servicio.is_empty() {
        servicio.replace("Atención Primaria", "Primaria")
    } else {
        d.tipo.clone()
    }
}

/// «2016-09-12» → «sep 2016».
fn mes_y_año(fecha: &str) -> String {
    const MESES: [&str; 12] = ["ene", "feb", "mar", "abr", "may", "jun",
                               "jul", "ago", "sep", "oct", "nov", "dic"];
    let mes: usize = fecha.get(5..7).and_then(|m| m.parse().ok()).unwrap_or(0);
    match MESES.get(mes.wrapping_sub(1)) {
        Some(m) => format!("{} {}", m, año(fecha)),
        None => año(fecha),
    }
}

/// El documento entero, para cuando se pasa el ratón por su fuente.
fn doc_entero(r: &Resumen, id: &str) -> Option<String> {
    r.documento(id).map(|d| format!("{} del {}\n{}", d.tipo, fecha_es(&d.fecha), d.servicio))
}

/// La fuente de un dato: en gris y en palabras («Endoscopia · sep 2016»);
/// encima del ratón, el documento entero.
fn chip(ui: &mut egui::Ui, r: &Resumen, id: &str) {
    let Some(d) = r.documento(id) else { return };
    let resp = ui.add(egui::Label::new(texto(format!("{} · {}", nombre_doc(d), mes_y_año(&d.fecha)),
                                             10.5, GRIS, false))
                      .sense(Sense::hover()));
    if let Some(t) = doc_entero(r, id) {
        resp.on_hover_text(t);
    }
}

fn icono_apagar(p: &egui::Painter, c: Pos2) {
    let s = trazo(2.0, BLANCO);
    let arco: Vec<Pos2> = (0..=20)
        .map(|i| {
            let a = -std::f32::consts::FRAC_PI_2 + 0.7 + i as f32 / 20.0 * (std::f32::consts::TAU - 1.4);
            c + vec2(6.5 * a.cos(), 6.5 * a.sin())
        })
        .collect();
    p.add(Shape::line(arco, s));
    p.line_segment([c + vec2(0.0, -8.0), c + vec2(0.0, -1.5)], s);
}

fn icono_reiniciar(p: &egui::Painter, c: Pos2) {
    let s = trazo(2.0, BLANCO);
    let (desde, hasta) = (-std::f32::consts::FRAC_PI_2 + 0.5,
                          -std::f32::consts::FRAC_PI_2 + std::f32::consts::TAU - 0.6);
    let arco: Vec<Pos2> = (0..=20)
        .map(|i| {
            let a = desde + (hasta - desde) * i as f32 / 20.0;
            c + vec2(6.5 * a.cos(), 6.5 * a.sin())
        })
        .collect();
    let fin = *arco.last().unwrap_or(&c);
    p.add(Shape::line(arco, s));
    // la punta de la flecha, al final del arco
    let (t, n) = (vec2(-hasta.sin(), hasta.cos()), vec2(hasta.cos(), hasta.sin()));
    p.add(Shape::convex_polygon(vec![fin + t * 4.0, fin - t * 1.5 + n * 3.5, fin - t * 1.5 - n * 3.5],
                                BLANCO, Stroke::NONE));
}

/// Una fila: el contenido y, debajo, en pequeño, de qué documento sale.
fn con_fuente(ui: &mut egui::Ui, r: &Resumen, doc: Option<&str>, dentro: impl FnOnce(&mut egui::Ui)) {
    let ancho = ui.available_width();
    columna(ui, ancho, Align::Min, |ui| {
        dentro(ui);
        if let Some(d) = doc {
            chip(ui, r, d);
        }
    });
}

fn caja(ui: &mut egui::Ui, fondo: Color32, borde: Color32, dentro: impl FnOnce(&mut egui::Ui)) {
    egui::Frame::none()
        .fill(fondo)
        .stroke(trazo(1.0, borde))
        .rounding(8.0)
        .inner_margin(egui::Margin::same(10.0))
        .show(ui, |ui| {
            ui.set_width(ui.available_width());
            ui.spacing_mut().item_spacing.y = 4.0;
            dentro(ui);
        });
}

/// Borde discontinuo: lo que no consta, para que no se confunda con un dato.
fn caja_discontinua(ui: &mut egui::Ui, dentro: impl FnOnce(&mut egui::Ui)) {
    let r = egui::Frame::none()
        .fill(NO_CONSTA_FONDO)
        .rounding(2.0)
        .inner_margin(egui::Margin::same(10.0))
        .show(ui, |ui| {
            ui.set_width(ui.available_width());
            ui.spacing_mut().item_spacing.y = 3.0;
            dentro(ui);
        })
        .response;
    let q = r.rect.shrink(0.5);
    let borde = [q.left_top(), q.right_top(), q.right_bottom(), q.left_bottom(), q.left_top()];
    ui.painter().extend(Shape::dashed_line(&borde, trazo(1.0, NO_CONSTA_BORDE), 4.0, 3.0));
}

fn separador(ui: &mut egui::Ui, color: Color32) {
    ui.add_space(3.0);
    let y = ui.cursor().top();
    ui.painter().hline(ui.max_rect().x_range(), y, trazo(1.0, color));
    ui.add_space(5.0);
}

fn icono_alerta(p: &egui::Painter, c: Pos2) {
    let s = trazo(1.6, ALERTA);
    p.add(Shape::closed_line(vec![c + vec2(0.0, -7.0), c + vec2(7.5, 6.0), c + vec2(-7.5, 6.0)], s));
    p.line_segment([c + vec2(0.0, -2.5), c + vec2(0.0, 1.5)], s);
    p.circle_filled(c + vec2(0.0, 3.8), 1.0, ALERTA);
}

fn icono_reloj(p: &egui::Painter, c: Pos2) {
    let s = trazo(1.5, PENDIENTE);
    p.circle_stroke(c, 6.5, s);
    p.line_segment([c, c + vec2(0.0, -4.0)], s);
    p.line_segment([c, c + vec2(3.0, 1.5)], s);
}

/// Un icono pintado de 18 px, alineado con la primera línea del texto.
fn icono(ui: &mut egui::Ui, pinta: fn(&egui::Painter, Pos2)) {
    let (sitio, _) = ui.allocate_exact_size(vec2(18.0, 18.0), Sense::hover());
    pinta(ui.painter(), sitio.center());
}

fn motivo(ui: &mut egui::Ui, r: &Resumen, tam: f32) {
    match &r.motivo {
        Some(c) => con_fuente(ui, r, c.documento.as_deref(), |ui| {
            ui.label(texto(&c.valor, tam, TINTA, true));
            if let Some(d) = &c.detalle {
                ui.label(texto(d, 12.0, TINTA_2, false));
            }
        }),
        None => {
            ui.label(texto(&r.paciente.motivo, tam, TINTA, true));
        }
    }
}

fn bloque_alertas(ui: &mut egui::Ui, r: &Resumen) {
    if r.alertas.is_empty() {
        return;
    }
    rotulo(ui, &format!("Alertas · {}", r.alertas.len()), ALERTA);
    for a in &r.alertas {
        caja(ui, ALERTA_FONDO, ALERTA_BORDE, |ui| {
            ui.horizontal_top(|ui| {
                icono(ui, icono_alerta);
                con_fuente(ui, r, a.documento.as_deref(), |ui| {
                    ui.label(texto(&a.titulo, 13.0, ALERTA_TITULO, true));
                    if let Some(d) = &a.detalle {
                        ui.label(texto(d, 12.0, TINTA_2, false));
                    }
                });
            });
        });
    }
    ui.add_space(10.0);
}

fn bloque_choques(ui: &mut egui::Ui, r: &Resumen) {
    if r.contradicciones.is_empty() {
        return;
    }
    rotulo(ui, &format!("No coincide con la derivación · {}", r.contradicciones.len()), CHOQUE);
    caja(ui, CHOQUE_FONDO, CHOQUE_BORDE, |ui| {
        for (i, h) in r.contradicciones.iter().enumerate() {
            if i > 0 {
                separador(ui, CHOQUE_BORDE);
            }
            let (cabeza, resto) = cabeza_y_resto(&h.afirmacion);
            con_fuente(ui, r, h.documento.as_deref(), |ui| {
                ui.label(texto(cabeza, 12.5, TINTA, true));
                if let Some(t) = resto {
                    ui.label(texto(t, 12.0, CHOQUE_TACHADO, false));
                }
            });
        }
    });
    ui.add_space(10.0);
}

/// Los valores que han cambiado, en casillas: cifra grande, detalle y fuente.
fn bloque_claves(ui: &mut egui::Ui, r: &Resumen, por_fila: usize) {
    let claves = r.de_seccion("cambios");
    if claves.is_empty() {
        return;
    }
    rotulo(ui, "Valores clave", GRIS);
    let hueco = 8.0;
    let ancho = ((ui.available_width() - hueco * (por_fila as f32 - 1.0)) / por_fila as f32).floor();
    for fila in claves.chunks(por_fila) {
        ui.horizontal_top(|ui| {
            ui.spacing_mut().item_spacing.x = hueco;
            for c in fila {
                egui::Frame::none()
                    .fill(BLANCO)
                    .stroke(trazo(1.0, LINEA))
                    .rounding(8.0)
                    .inner_margin(egui::Margin::symmetric(10.0, 8.0))
                    .show(ui, |ui| {
                        ui.set_width(ancho - 22.0);
                        ui.set_min_height(58.0);
                        ui.vertical(|ui| {
                            ui.spacing_mut().item_spacing.y = 1.0;
                            ui.label(texto(&c.etiqueta, 11.5, GRIS, false));
                            ui.label(texto(&c.valor, 17.0, TINTA, true));
                            if let Some(d) = &c.detalle {
                                ui.label(texto(d, 11.0, TINTA_2, false));
                            }
                            if let Some(d) = &c.documento {
                                chip(ui, r, d);
                            }
                        });
                    });
            }
        });
        ui.add_space(4.0);
    }
    ui.add_space(6.0);
}

fn bloque_pendiente(ui: &mut egui::Ui, r: &Resumen) {
    if r.faltas.is_empty() {
        return;
    }
    rotulo(ui, "Pendiente", PENDIENTE);
    caja(ui, PENDIENTE_FONDO, PENDIENTE_BORDE, |ui| {
        for h in &r.faltas {
            ui.horizontal_top(|ui| {
                icono(ui, icono_reloj);
                con_fuente(ui, r, h.documento.as_deref(), |ui| {
                    ui.label(texto(&h.afirmacion, 12.5, TINTA, false));
                });
            });
        }
    });
    ui.add_space(10.0);
}

fn bloque_no_consta(ui: &mut egui::Ui, r: &Resumen) {
    if r.no_consta.is_empty() {
        return;
    }
    caja_discontinua(ui, |ui| {
        rotulo(ui, "No consta · preguntar", GRIS);
        for t in &r.no_consta {
            ui.label(texto(t, 12.0, TINTA_2, false));
        }
    });
    ui.add_space(10.0);
}

fn bloque_consulta(ui: &mut egui::Ui, r: &Resumen) {
    let campos = r.para_la_consulta();
    if campos.is_empty() {
        return;
    }
    rotulo(ui, "Para la consulta", GRIS);
    caja(ui, BLANCO, LINEA, |ui| {
        for (i, c) in campos.iter().enumerate() {
            if i > 0 {
                separador(ui, LINEA_SUAVE);
            }
            con_fuente(ui, r, c.documento.as_deref(), |ui| {
                ui.label(texto(&c.etiqueta, 11.5, GRIS, true));
                ui.label(texto(&c.valor, 12.5, TINTA, false));
            });
        }
    });
    ui.add_space(10.0);
}

fn bloque_historia(ui: &mut egui::Ui, r: &Resumen) {
    if r.historia.is_empty() {
        return;
    }
    rotulo(ui, "Qué le ha pasado", GRIS);
    caja(ui, BLANCO, LINEA, |ui| {
        for (i, h) in r.historia.iter().enumerate() {
            if i > 0 {
                separador(ui, LINEA_SUAVE);
            }
            con_fuente(ui, r, h.documento.as_deref(), |ui| {
                ui.label(texto(&h.afirmacion, 12.5, TINTA, false));
            });
        }
    });
    ui.add_space(10.0);
}

/// La historia en una línea de tiempo: del documento más reciente al más antiguo.
/// Punto rojo: el documento tiene algo de riesgo.
fn linea_tiempo(ui: &mut egui::Ui, r: &Resumen) {
    let n = r.documentos.len();
    for (i, d) in r.documentos.iter().enumerate() {
        let fila = ui.horizontal_top(|ui| {
            ui.spacing_mut().item_spacing.x = 6.0;
            columna(ui, 44.0, Align::Min, |ui| {
                ui.label(texto(mes_año(&d.fecha), 11.0, GRIS, true));
            });
            let (punto, _) = ui.allocate_exact_size(vec2(12.0, 16.0), Sense::hover());
            let ancho = ui.available_width();
            columna(ui, ancho, Align::Min, |ui| {
                con_fuente(ui, r, Some(&d.id), |ui| {
                    ui.label(texto(d.servicio.to_uppercase(), 10.0, GRIS, true));
                    ui.label(texto(r.linea_de(d), 12.5, TINTA, false));
                });
                ui.add_space(10.0);
            });
            punto.center()
        });
        let c = fila.inner;
        let p = ui.painter();
        if i + 1 < n {
            p.vline(c.x, egui::Rangef::new(c.y + 5.0, fila.response.rect.bottom() + 6.0), trazo(2.0, LINEA));
        }
        if r.marcados.contains(&d.id) {
            p.circle_filled(c, 5.0, ALERTA);
        } else if i == 0 {
            p.circle_filled(c, 5.0, VERDE);
        } else {
            p.circle_filled(c, 5.0, BLANCO);
            p.circle_stroke(c, 4.0, trazo(2.0, NO_CONSTA_BORDE));
        }
    }
    ui.add_space(10.0);
}

/// La penúltima y la última analítica, lado a lado.
fn tabla_labs(ui: &mut egui::Ui, r: &Resumen, id: &str) {
    if r.labs.is_empty() {
        return;
    }
    rotulo(ui, "Analíticas", GRIS);
    caja(ui, BLANCO, LINEA, |ui| {
        egui::Grid::new(id).num_columns(3).spacing(vec2(14.0, 6.0)).show(ui, |ui| {
            ui.label(texto("Parámetro", 11.0, GRIS, false));
            for doc in [&r.labs_docs.0, &r.labs_docs.1] {
                match doc {
                    Some((d, f)) => {
                        let fecha = ui.add(egui::Label::new(texto(fecha_es(f), 11.0, GRIS, false))
                                           .sense(Sense::hover()));
                        match doc_entero(r, d) {
                            Some(t) => fecha.on_hover_text(t),
                            None => fecha,
                        }
                    }
                    None => ui.label(texto("—", 11.0, GRIS, false)),
                };
            }
            ui.end_row();
            for f in &r.labs {
                let nombre = if f.unidad.is_empty() { f.nombre.clone() } else { format!("{} ({})", f.nombre, f.unidad) };
                ui.label(texto(nombre, 12.5, TINTA, false));
                ui.label(texto(f.antes.clone().unwrap_or_else(|| "—".into()), 12.5, GRIS, false));
                let numero = |v: &Option<String>| v.as_ref().and_then(|v| v.replace(',', ".").parse::<f32>().ok());
                let flecha = match (numero(&f.antes), numero(&f.ahora)) {
                    (Some(a), Some(b)) if b < a => " ↓",
                    (Some(a), Some(b)) if b > a => " ↑",
                    _ => "",
                };
                ui.label(texto(format!("{}{}", f.ahora.clone().unwrap_or_else(|| "—".into()), flecha), 12.5, TINTA, true));
                ui.end_row();
            }
        });
    });
    ui.add_space(10.0);
}

fn lista_pruebas(ui: &mut egui::Ui, r: &Resumen) {
    if r.pruebas.is_empty() {
        return;
    }
    rotulo(ui, "Pruebas e imagen", GRIS);
    for h in &r.pruebas {
        caja(ui, BLANCO, LINEA, |ui| {
            ui.horizontal_top(|ui| {
                columna(ui, 34.0, Align::Min, |ui| {
                    ui.label(texto(h.fecha_doc.as_deref().map(año).unwrap_or_default(), 11.0, GRIS, true));
                });
                let (cabeza, resto) = cabeza_y_resto(&h.afirmacion);
                con_fuente(ui, r, h.documento.as_deref(), |ui| {
                    ui.label(texto(cabeza, 12.5, TINTA, true));
                    if let Some(t) = resto {
                        ui.label(texto(t, 12.0, TINTA_2, false));
                    }
                });
            });
        });
    }
    ui.add_space(10.0);
}

fn lista_medicacion(ui: &mut egui::Ui, r: &Resumen) {
    let meds = r.de_seccion("medicacion");
    if meds.is_empty() {
        caja_discontinua(ui, |ui| {
            rotulo(ui, "Medicación · no consta", GRIS);
            ui.label(texto("No viene en los datos que tiene Medicci de este paciente: pregúntela en la consulta.",
                           12.0, TINTA_2, false));
        });
        ui.add_space(10.0);
        return;
    }
    rotulo(ui, "Medicación", GRIS);
    caja(ui, BLANCO, LINEA, |ui| {
        for (i, c) in meds.iter().enumerate() {
            if i > 0 {
                separador(ui, LINEA_SUAVE);
            }
            con_fuente(ui, r, c.documento.as_deref(), |ui| {
                ui.horizontal(|ui| {
                    ui.label(texto(&c.etiqueta, 13.5, TINTA, true));
                    if c.destacado {
                        marca(ui, "Revisar", ALERTA_FONDO, ALERTA);
                    }
                });
                ui.label(texto(&c.valor, 12.0, if sin_dato(&c.valor) { GRIS } else { TINTA_2 }, false));
                if c.destacado {
                    if let Some(d) = &c.detalle {
                        ui.label(texto(d, 12.0, ALERTA_TITULO, false));
                    }
                }
            });
        }
    });
    let alergias: Vec<&CampoFicha> = r.campos.iter()
        .filter(|c| c.bloque == "antes_de_empezar" && c.etiqueta == "Alergias" && !sin_dato(&c.valor))
        .collect();
    if !alergias.is_empty() {
        ui.add_space(4.0);
        ui.horizontal_wrapped(|ui| {
            ui.spacing_mut().item_spacing = vec2(6.0, 6.0);
            for c in alergias {
                marca(ui, &format!("Evitar: {}", c.valor), ALERTA_FONDO, ALERTA_TITULO);
            }
        });
    }
    ui.add_space(10.0);
}

/// La hemoglobina de toda la historia: bajadas en rojo, el resto en gris.
fn grafica_hb(ui: &mut egui::Ui, r: &Resumen) {
    let puntos: Vec<(f32, &ValorHallado)> = r.hb.iter()
        .filter_map(|v| v.valor.replace(',', ".").parse::<f32>().ok().map(|n| (n, v)))
        .collect();
    if puntos.len() < 2 {
        return;
    }
    rotulo(ui, "Hemoglobina · evolución (g/dL)", GRIS);
    let (caja, _) = ui.allocate_exact_size(vec2(ui.available_width(), 150.0), Sense::hover());
    let p = ui.painter();
    p.rect(caja, 8.0, BLANCO, trazo(1.0, LINEA));
    let zona = caja.shrink2(vec2(36.0, 32.0));
    let (min, max) = puntos.iter().fold((f32::MAX, f32::MIN), |(a, b), (n, _)| (a.min(*n), b.max(*n)));
    let (min, max) = (min - 0.5, max + 0.5);
    let n = puntos.len() as f32 - 1.0;
    let sitios: Vec<Pos2> = puntos.iter().enumerate()
        .map(|(i, (v, _))| pos2(zona.left() + zona.width() * i as f32 / n,
                                zona.bottom() - (v - min) / (max - min) * zona.height()))
        .collect();
    for (i, w) in sitios.windows(2).enumerate() {
        let baja = puntos[i + 1].0 < puntos[i].0;
        p.line_segment([w[0], w[1]], trazo(if baja { 2.5 } else { 2.0 }, if baja { ALERTA } else { GRIS }));
    }
    for (i, (s, (v, h))) in sitios.iter().zip(&puntos).enumerate() {
        let ultimo = i + 1 == sitios.len();
        let color = if ultimo { ALERTA } else { TINTA_2 };
        p.circle_filled(*s, if ultimo { 5.5 } else { 4.0 }, color);
        p.text(*s - vec2(0.0, 9.0), egui::Align2::CENTER_BOTTOM, format!("{:.1}", v).replace('.', ","),
               FontId::new(11.5, negrita()), if ultimo { ALERTA } else { TINTA });
        p.text(pos2(s.x, caja.bottom() - 7.0), egui::Align2::CENTER_BOTTOM, mes_año(&h.fecha),
               FontId::proportional(10.5), GRIS);
    }
    ui.add_space(2.0);
    ui.horizontal_wrapped(|ui| {
        ui.spacing_mut().item_spacing = vec2(0.0, 4.0);
        ui.label(texto("Fuentes: ", 10.5, GRIS, false));
        for (i, (_, h)) in puntos.iter().enumerate() {
            if i > 0 {
                ui.label(texto(", ", 10.5, GRIS, false));
            }
            chip(ui, r, &h.documento);
        }
    });
    ui.add_space(10.0);
}

/// Una pestaña de la tarjeta: nombre, número y, si está abierta, subrayada.
fn pestana_boton(ui: &mut egui::Ui, nombre: &str, n: usize, roja: bool, abierta: bool) -> bool {
    let tinta = if abierta { TINTA } else { GRIS };
    let familia = if abierta { negrita() } else { FontFamily::Proportional };
    let g_nombre = ui.painter().layout_no_wrap(nombre.to_string(), FontId::new(12.5, familia), tinta);
    let (fondo_n, tinta_n) = if roja && n > 0 { (ALERTA_FONDO, ALERTA) } else { (LINEA_SUAVE, GRIS) };
    let g_num = ui.painter().layout_no_wrap(n.to_string(), FontId::new(10.5, negrita()), tinta_n);
    let (tam_nombre, tam_num) = (g_nombre.size(), g_num.size());
    let ancho = 8.0 + tam_nombre.x + 5.0 + tam_num.x + 10.0 + 8.0;
    let (rect, resp) = ui.allocate_exact_size(vec2(ancho, 36.0), Sense::click());
    let p = ui.painter();
    let x = rect.left() + 8.0;
    p.galley(pos2(x, rect.center().y - tam_nombre.y / 2.0), g_nombre, tinta);
    let pildora = Rect::from_min_size(pos2(x + tam_nombre.x + 5.0, rect.center().y - 8.0),
                                      vec2(tam_num.x + 10.0, 16.0));
    p.rect_filled(pildora, 8.0, fondo_n);
    p.galley(pildora.center() - tam_num / 2.0, g_num, tinta_n);
    if abierta {
        p.hline(rect.x_range(), rect.bottom() - 1.0, trazo(2.5, VERDE));
    } else if resp.hovered() {
        p.hline(rect.x_range(), rect.bottom() - 1.0, trazo(2.0, LINEA));
    }
    resp.on_hover_cursor(egui::CursorIcon::PointingHand).clicked()
}

/// La tarjeta de resumen dentro del chat: motivo, pestañas y el botón de ampliar.
fn tarjeta(ui: &mut egui::Ui, r: &Resumen, pestana: Pestana) -> Option<Accion> {
    let mut accion = None;
    egui::Frame::none()
        .fill(BLANCO)
        .stroke(trazo(1.0, LINEA))
        .rounding(12.0)
        .show(ui, |ui| {
            ui.set_width(ui.available_width());
            ui.spacing_mut().item_spacing.y = 4.0;
            egui::Frame::none()
                .inner_margin(egui::Margin { left: 12.0, right: 12.0, top: 10.0, bottom: 4.0 })
                .show(ui, |ui| {
                    ui.horizontal(|ui| {
                        rotulo(ui, "Por qué viene hoy", GRIS);
                        ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                            let b = egui::Button::new(texto("Ampliar", 11.5, TINTA, true))
                                .fill(BLANCO)
                                .stroke(trazo(1.0, LINEA))
                                .rounding(8.0)
                                .min_size(vec2(0.0, 26.0));
                            if ui.add(b).on_hover_text("Ver todo el resumen en una ventana grande")
                                .on_hover_cursor(egui::CursorIcon::PointingHand).clicked() {
                                accion = Some(Accion::Ampliar);
                            }
                        });
                    });
                    motivo(ui, r, 13.5);
                });

            let pestanas = [
                (Pestana::Resumen, "Resumen", r.alertas.len() + r.contradicciones.len(), true),
                (Pestana::Historial, "Historial", r.documentos.len(), false),
                (Pestana::Pruebas, "Pruebas", r.pruebas.len(), false),
                (Pestana::Medicacion, "Medicación", r.de_seccion("medicacion").len(), false),
            ];
            let fila = ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing.x = 0.0;
                ui.add_space(4.0);
                for (p, nombre, n, roja) in pestanas {
                    if pestana_boton(ui, nombre, n, roja, p == pestana) {
                        accion = Some(Accion::Pestana(p));
                    }
                }
            }).response;
            ui.painter().hline(ui.max_rect().x_range(), fila.rect.bottom() + 0.5, trazo(1.0, LINEA));

            egui::Frame::none()
                .inner_margin(egui::Margin { left: 12.0, right: 12.0, top: 10.0, bottom: 2.0 })
                .show(ui, |ui| {
                    ui.set_width(ui.available_width());
                    ui.spacing_mut().item_spacing.y = 4.0;
                    match pestana {
                        Pestana::Resumen => {
                            bloque_alertas(ui, r);
                            bloque_choques(ui, r);
                            bloque_claves(ui, r, 2);
                            bloque_pendiente(ui, r);
                            bloque_no_consta(ui, r);
                        }
                        Pestana::Historial => linea_tiempo(ui, r),
                        Pestana::Pruebas => {
                            tabla_labs(ui, r, "labs_chat");
                            lista_pruebas(ui, r);
                            bloque_pendiente(ui, r);
                        }
                        Pestana::Medicacion => lista_medicacion(ui, r),
                    }
                });
            let y = ui.cursor().top();
            ui.painter().hline(ui.max_rect().x_range(), y, trazo(1.0, LINEA));
            egui::Frame::none()
                .inner_margin(egui::Margin::symmetric(12.0, 8.0))
                .show(ui, |ui| {
                    ui.label(texto("Cada dato lleva su documento. Medicci propone; usted decide.", 11.0, GRIS, false));
                });
        });
    accion
}

/// El resumen ampliado: el motivo arriba y tres columnas que bajan por separado.
fn hoja_ampliada(ui: &mut egui::Ui, r: &Resumen, cuerpo: Rect) -> Option<Accion> {
    let mut accion = None;
    let franja = egui::Frame::none()
        .fill(FICHA_FONDO)
        .inner_margin(egui::Margin::symmetric(22.0, 12.0))
        .show(ui, |ui| {
            ui.set_width(ui.available_width());
            ui.horizontal_top(|ui| {
                columna(ui, 140.0, Align::Min, |ui| {
                    ui.add_space(3.0);
                    rotulo(ui, "Por qué viene hoy", GRIS);
                });
                let ancho = ui.available_width();
                columna(ui, ancho, Align::Min, |ui| {
                    motivo(ui, r, 15.5);
                    let chips: Vec<&CampoFicha> = r.de("cabecera").into_iter()
                        .filter(|c| !c.valor.contains(" años")).collect();
                    if !chips.is_empty() {
                        ui.add_space(4.0);
                        ui.horizontal_wrapped(|ui| {
                            ui.spacing_mut().item_spacing = vec2(6.0, 6.0);
                            for c in chips {
                                pastilla(ui, &c.valor, c.destacado);
                            }
                        });
                    }
                });
            });
        })
        .response;
    ui.painter().hline(cuerpo.x_range(), franja.rect.bottom(), trazo(1.0, LINEA));

    let arriba = franja.rect.bottom() + 1.0;
    let alto = (cuerpo.bottom() - arriba).max(120.0);
    let total = cuerpo.width();
    let a1 = (total * 0.31).floor();
    let a3 = (total * 0.29).floor();
    let a2 = total - a1 - a3;
    let columnas = [
        Rect::from_min_size(pos2(cuerpo.left(), arriba), vec2(a1, alto)),
        Rect::from_min_size(pos2(cuerpo.left() + a1, arriba), vec2(a2, alto)),
        Rect::from_min_size(pos2(cuerpo.left() + a1 + a2, arriba), vec2(a3, alto)),
    ];
    for (i, sitio) in columnas.iter().enumerate() {
        if i > 0 {
            ui.painter().vline(sitio.left(), sitio.y_range(), trazo(1.0, LINEA));
        }
        ui.allocate_new_ui(UiBuilder::new().max_rect(*sitio), |ui| {
            egui::ScrollArea::vertical()
                .id_salt(("columna", i))
                .auto_shrink([false, false])
                .show(ui, |ui| {
                    egui::Frame::none()
                        .inner_margin(egui::Margin::symmetric(18.0, 14.0))
                        .show(ui, |ui| {
                            ui.set_width(ui.available_width());
                            ui.spacing_mut().item_spacing.y = 4.0;
                            match i {
                                0 => {
                                    bloque_alertas(ui, r);
                                    bloque_choques(ui, r);
                                    bloque_consulta(ui, r);
                                    bloque_no_consta(ui, r);
                                }
                                1 => {
                                    bloque_historia(ui, r);
                                    grafica_hb(ui, r);
                                    bloque_claves(ui, r, 4);
                                    tabla_labs(ui, r, "labs_hoja");
                                    lista_pruebas(ui, r);
                                    bloque_pendiente(ui, r);
                                }
                                _ => {
                                    lista_medicacion(ui, r);
                                    rotulo(ui, &format!("Historia · {} a {}", r.desde, r.hasta), GRIS);
                                    ui.add_space(4.0);
                                    linea_tiempo(ui, r);
                                    let temas = r.de("temas");
                                    if !temas.is_empty() {
                                        rotulo(ui, "Memoria profunda · se pregunta en el chat", GRIS);
                                        ui.horizontal_wrapped(|ui| {
                                            ui.spacing_mut().item_spacing = vec2(6.0, 6.0);
                                            for c in temas {
                                                let b = egui::Button::new(texto(&c.valor, 12.0, VERDE_2, false))
                                                    .fill(BLANCO)
                                                    .stroke(trazo(1.0, LINEA))
                                                    .rounding(14.0);
                                                if ui.add(b).on_hover_cursor(egui::CursorIcon::PointingHand).clicked() {
                                                    accion = Some(Accion::Tema(c.valor.clone()));
                                                }
                                            }
                                        });
                                    }
                                }
                            }
                        });
                });
        });
    }
    accion
}

/// El resumen en texto, para pegarlo en la nota clínica del HCE.
fn texto_nota(r: &Resumen) -> String {
    let fuente = |d: &Option<String>| d.as_ref().map(|d| format!(" [{}]", d)).unwrap_or_default();
    let mut t = format!("Resumen Medicci · {} · {} años · {}\n", r.paciente.id, r.paciente.edad, r.paciente.sexo);
    if let Some(m) = &r.motivo {
        t += &format!("Motivo: {}{}\n", m.valor, fuente(&m.documento));
    }
    if !r.alertas.is_empty() {
        t += "\nAlertas:\n";
        for a in &r.alertas {
            t += &format!("- {}{}\n", a.titulo, fuente(&a.documento));
        }
    }
    if !r.contradicciones.is_empty() {
        t += "\nNo coincide con la derivación:\n";
        for h in &r.contradicciones {
            t += &format!("- {}{}\n", h.afirmacion, fuente(&h.documento));
        }
    }
    let claves = r.de_seccion("cambios");
    if !claves.is_empty() {
        t += "\nValores clave:\n";
        for c in claves {
            let detalle = c.detalle.as_ref().map(|d| format!(" ({})", d)).unwrap_or_default();
            let fecha = c.fecha_doc.as_deref().map(|f| format!(", {}", fecha_es(f))).unwrap_or_default();
            t += &format!("- {}: {}{}{}{}\n", c.etiqueta, c.valor, detalle, fecha, fuente(&c.documento));
        }
    }
    if !r.faltas.is_empty() {
        t += "\nPendiente:\n";
        for h in &r.faltas {
            t += &format!("- {}{}\n", h.afirmacion, fuente(&h.documento));
        }
    }
    if !r.no_consta.is_empty() {
        t += "\nNo consta:\n";
        for n in &r.no_consta {
            t += &format!("- {}\n", n);
        }
    }
    t += &format!("\nLeídos {} documentos, de {} a {}. Generado por Medicci con IA local: verificar antes de decidir.\n",
                  r.documentos.len(), r.desde, r.hasta);
    t
}

impl Medicci {
    /// La ventana del resumen ampliado. Como la del chat, existe desde el
    /// arranque y, cerrada, es transparente y deja pasar los clics.
    fn ventana_resumen(&mut self, ctx: &egui::Context) {
        if self.resumen.is_none() {
            self.ampliado = false;
        }
        let abierta = self.ampliado;
        otro_par_de_fotogramas(ctx, &mut self.resumen_visto, abierta);
        let monitor = ctx.input(|i| i.viewport().monitor_size).unwrap_or(vec2(1440.0, 900.0));
        let hoja = vec2(HOJA.x.min(monitor.x - 80.0), HOJA.y.min(monitor.y - 80.0));
        let tam = hoja + Vec2::splat(2.0 * AIRE);
        let pos = *self.resumen_pos.get_or_insert_with(|| {
            pos2(((monitor.x - tam.x) / 2.0).round(), ((monitor.y - tam.y) / 2.0).round())
        });
        let constructor = egui::ViewportBuilder::default()
            .with_title("Medicci · Resumen")
            .with_mouse_passthrough(!abierta)
            .with_inner_size(tam)
            .with_position(pos)
            .with_decorations(false)
            .with_transparent(true)
            .with_resizable(false)
            .with_taskbar(false);
        let id = egui::ViewportId::from_hash_of("medicci_resumen");
        let Some(resumen) = (if abierta { self.resumen.take() } else { None }) else {
            ctx.show_viewport_immediate(id, constructor, |_, _| {});
            return;
        };

        let dt = ctx.input(|i| i.stable_dt).min(0.05);
        self.resumen_t = (self.resumen_t + dt / 0.22).min(1.0);
        let e = easing::cubic_out(self.resumen_t);
        let ahora = ctx.input(|i| i.time);
        let recien_copiado = ahora - self.copiado < 2.0;
        let mut cerrar = false;
        let mut copiar = false;
        let mut accion = None;

        ctx.show_viewport_immediate(id, constructor, |ctx, _clase| {
            if ctx.input(|i| i.viewport().close_requested() || i.key_pressed(egui::Key::Escape)) {
                cerrar = true;
            }
            // al abrirse, y al pedirla otra vez, delante de todo (también del chat)
            sube_un_momento(ctx, &mut self.resumen_encima, &mut self.resumen_subida,
                            std::mem::take(&mut self.resumen_al_frente));
            if e < 1.0 || recien_copiado {
                ctx.request_repaint_after(FOTOGRAMA);
            }
            let caja = Rect::from_min_size(pos2(AIRE, AIRE), hoja);
            let s = 0.96 + 0.04 * e;
            let capa = egui::LayerId::new(egui::Order::Middle, egui::Id::new("resumen_hoja"));
            ctx.set_transform_layer(capa, TSTransform::new(
                caja.center().to_vec2() * (1.0 - s) + vec2(0.0, (1.0 - e) * 8.0), s));
            egui::CentralPanel::default().frame(egui::Frame::none()).show(ctx, |_| {});
            egui::Area::new(egui::Id::new("resumen_hoja"))
                .order(egui::Order::Middle)
                .fixed_pos(caja.min)
                .constrain(false)
                .show(ctx, |ui| {
                    ui.set_opacity(e);
                    let p = ui.painter().clone();
                    p.add(egui::epaint::Shadow { offset: vec2(0.0, 6.0), blur: 22.0, spread: 0.0,
                                                 color: Color32::from_black_alpha(60) }
                          .as_shape(caja, 14.0));
                    p.rect(caja, 14.0, FONDO, trazo(1.0, LINEA));

                    // cabecera: se arrastra desde aquí
                    let cabeza = Rect::from_min_size(caja.min, vec2(caja.width(), 74.0));
                    p.rect_filled(cabeza, Rounding { nw: 14.0, ne: 14.0, sw: 0.0, se: 0.0 }, VERDE_OSC);
                    let arrastre = ui.interact(cabeza, egui::Id::new("arrastre"), Sense::click_and_drag());
                    if arrastre.drag_started() {
                        ctx.send_viewport_cmd(egui::ViewportCommand::StartDrag);
                    }
                    let c = pos2(cabeza.left() + 38.0, cabeza.center().y);
                    p.circle_filled(c, 19.0, VERDE);
                    p.circle_stroke(c, 18.5, trazo(1.0, ORO));
                    medico(&p, c, 18.0, VERDE, 1.0);
                    let pac = &resumen.paciente;
                    p.text(pos2(c.x + 31.0, cabeza.center().y - 10.0), egui::Align2::LEFT_CENTER,
                           format!("Resumen ampliado · {}", pac.id),
                           FontId::new(17.0, negrita()), BLANCO);
                    p.text(pos2(c.x + 31.0, cabeza.center().y + 12.0), egui::Align2::LEFT_CENTER,
                           format!("{} años · {} · {} documentos leídos, de {} a {}",
                                   pac.edad, pac.sexo, resumen.documentos.len(), resumen.desde, resumen.hasta),
                           FontId::proportional(12.0), VERDE_CLARO);
                    let x = Rect::from_center_size(pos2(cabeza.right() - 30.0, cabeza.center().y),
                                                   Vec2::splat(30.0));
                    let rx = ui.interact(x, egui::Id::new("cerrar_resumen"), Sense::click())
                        .on_hover_cursor(egui::CursorIcon::PointingHand);
                    if rx.hovered() {
                        p.circle_filled(x.center(), 15.0, Color32::from_white_alpha(22));
                    }
                    let d = 5.5;
                    let sx = trazo(2.0, BLANCO);
                    p.line_segment([x.center() + vec2(-d, -d), x.center() + vec2(d, d)], sx);
                    p.line_segment([x.center() + vec2(-d, d), x.center() + vec2(d, -d)], sx);
                    if rx.clicked() {
                        cerrar = true;
                    }
                    // copiar a la nota clínica, a la izquierda de la X
                    let sitio = Rect::from_min_max(pos2(cabeza.center().x, cabeza.center().y - 17.0),
                                                   pos2(x.left() - 12.0, cabeza.center().y + 17.0));
                    ui.allocate_new_ui(UiBuilder::new().max_rect(sitio)
                                           .layout(Layout::right_to_left(Align::Center)), |ui| {
                        let t = if recien_copiado { "Copiado en el portapapeles" } else { "Copiar a la nota clínica" };
                        let b = egui::Button::new(texto(t, 12.5, BLANCO, true))
                            .fill(VERDE)
                            .stroke(trazo(1.0, VERDE_CLARO))
                            .rounding(10.0)
                            .min_size(vec2(0.0, 34.0));
                        if ui.add(b).on_hover_cursor(egui::CursorIcon::PointingHand).clicked() {
                            copiar = true;
                        }
                    });

                    let pie = Rect::from_min_max(pos2(caja.left(), caja.bottom() - 52.0), caja.max);
                    let cuerpo = Rect::from_min_max(cabeza.left_bottom(), pie.right_top());
                    ui.allocate_new_ui(UiBuilder::new().max_rect(cuerpo), |ui| {
                        accion = hoja_ampliada(ui, &resumen, cuerpo);
                    });

                    p.rect_filled(pie, Rounding { nw: 0.0, ne: 0.0, sw: 14.0, se: 14.0 }, BLANCO);
                    p.hline(pie.x_range(), pie.top() + 0.5, trazo(1.0, LINEA));
                    ui.allocate_new_ui(UiBuilder::new().max_rect(pie.shrink2(vec2(20.0, 9.0)))
                                           .layout(Layout::left_to_right(Align::Center)), |ui| {
                        let (punto, _) = ui.allocate_exact_size(vec2(8.0, 8.0), Sense::hover());
                        ui.painter().circle_filled(punto.center(), 3.5, VERDE);
                        ui.label(texto("Cada dato lleva su fuente. Medicci propone; usted decide.", 11.5, GRIS, false));
                        ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                            if boton(ui, "Volver al chat", VERDE, BLANCO, true) {
                                cerrar = true;
                            }
                        });
                    });
                });
        });

        if copiar {
            ctx.copy_text(texto_nota(&resumen));
            self.datos.registra(&self.usuario, &resumen.paciente.id, "copiar_resumen", None);
            if self.red.is_some() {
                self.pedidos.push(Pedido::Registro("copiar_nota"));
            }
            self.copiado = ahora;
        }
        if let Some(Accion::Tema(t)) = accion {
            self.pendiente = Some(t);
            cerrar = true;
        }
        if cerrar {
            self.ampliado = false;
        }
        self.resumen = Some(resumen);
    }
}


impl eframe::App for Medicci {
    fn clear_color(&self, _visuals: &egui::Visuals) -> [f32; 4] {
        [0.0, 0.0, 0.0, 0.0]            // fondo transparente: solo se ve lo que se pinta
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if ctx.input(|i| i.key_pressed(egui::Key::Escape)) {
            self.abierto = false;
        }
        // el despliegue: la ventana del chat ya existe, así que empieza al pulsar
        let dt = ctx.input(|i| i.stable_dt).min(0.05);
        if self.abierto {
            self.t = (self.t + dt / 0.26).min(1.0);
        } else {
            self.t = (self.t - dt / 0.18).max(0.0);
        }
        if (self.abierto && self.t < 1.0) || (!self.abierto && self.t > 0.0) {
            ctx.request_repaint_after(FOTOGRAMA);
        }

        // «siempre encima» se pide al crear la ventana, pero algunos gestores de
        // ventanas (openbox) no hacen caso hasta que ya se ve: se vuelve a pedir
        if self.marcos < 3 {
            self.marcos += 1;
            ctx.request_repaint();
            if self.marcos == 3 {
                ctx.send_viewport_cmd(egui::ViewportCommand::WindowLevel(egui::WindowLevel::AlwaysOnTop));
            }
        }
        self.red(ctx);
        self.coloca(ctx);
        if std::env::var_os("MEDICCI_DEPURA").is_some() {
            let (monitor, real) = ctx.input(|i| (i.viewport().monitor_size, i.viewport().outer_rect));
            eprintln!("abierto={} t={:.2} lista={} intentos={} monitor={:?} real={:?} pedido={:?}",
                      self.abierto, self.t, self.lista, self.intentos, monitor, real, self.pedido);
        }
        self.burbuja(ctx);
        self.ventana_chat(ctx);
        self.ventana_resumen(ctx);

        if let Some(tema) = self.pendiente.take() {
            self.abierto = true;
            self.entrada = tema;
            self.preguntar();
        }
    }
}

/// Una sola burbuja por usuario: la primera se queda un puerto local y las
/// demás, al no poder, se cierran. El puerto sale del usuario de Windows, para
/// que en un mismo servidor (Citrix) cada médico tenga la suya. Al reiniciar,
/// la nueva espera a que la vieja lo suelte.
///
/// Windows reserva rangos de puertos (Hyper-V, WSL) que cambian al reiniciar:
/// ahí el puerto da «acceso denegado», no «en uso», y se prueba el siguiente.
/// Solo «en uso» quiere decir que ya hay otra burbuja. Si no se puede coger
/// ninguno, la burbuja abre igual, sin cerrojo.
fn una_sola(usuario: &str, esperar: bool) -> Result<Option<std::net::TcpListener>, ()> {
    let base = 40000 + usuario.bytes().fold(7u32, |h, b| h.wrapping_mul(31).wrapping_add(b as u32)) % 20000;
    for intento in 0..if esperar { 50 } else { 1 } {
        if intento > 0 {
            std::thread::sleep(std::time::Duration::from_millis(100));
        }
        let mut en_uso = false;
        for puerto in base..base + 300 {
            match std::net::TcpListener::bind(("127.0.0.1", puerto as u16)) {
                Ok(l) => return Ok(Some(l)),
                Err(e) if e.kind() == std::io::ErrorKind::AddrInUse => {
                    en_uso = true;
                    break;
                }
                Err(_) => {}
            }
        }
        if !en_uso {
            return Ok(None);
        }
    }
    Err(())
}

fn main() -> eframe::Result<()> {
    // En Wayland una ventana no puede colocarse sola en la esquina; por
    // XWayland sí, así que en Linux se prefiere X11 cuando lo hay.
    #[cfg(target_os = "linux")]
    if std::env::var_os("DISPLAY").is_some() {
        std::env::remove_var("WAYLAND_DISPLAY");
    }

    let mut ruta: Option<PathBuf> = None;
    let mut paciente: Option<String> = None;
    // adónde mandar las peticiones: --servidor, la variable o medicci.conf (el que diga el hospital)
    let mut url = std::env::var("MEDICCI_SERVIDOR").ok().or_else(|| servidor::de_la_configuracion("MEDICCI_SERVIDOR"));
    let mut reinicio = false;
    let mut abierta = false;
    let mut args = std::env::args().skip(1);
    while let Some(a) = args.next() {
        match a.as_str() {
            "--reinicio" => reinicio = true,
            "--abierta" => abierta = true,        // con el chat ya desplegado: para la demo
            "--db" => ruta = args.next().map(PathBuf::from),
            "--paciente" => paciente = args.next(),
            "--servidor" => url = args.next(),
            "--ayuda" | "-h" | "--help" => {
                println!("medicci [--db ruta.db] [--paciente 417] [--servidor http://localhost:8000] [--abierta]");
                return Ok(());
            }
            _ => {}
        }
    }
    let usuario = std::env::var("USER").or_else(|_| std::env::var("USERNAME")).unwrap_or_default();
    let Ok(_cerrojo) = una_sola(&usuario, reinicio) else {
        eprintln!("Medicci ya está abierta.");
        return Ok(());
    };
    // Conectada si el servidor responde; si no, sola con su base, como siempre.
    let red = url.map(|u| Servidor::nuevo(&u)).filter(|s| {
        let si = s.responde();
        if !si {
            eprintln!("El servidor de Medicci no responde: la burbuja usa su base local.");
        }
        si
    });
    let ruta = ruta.unwrap_or_else(|| {
        let exe = std::env::current_exe().unwrap_or_default();
        for base in [exe.parent().map(|p| p.to_path_buf()), std::env::current_dir().ok()]
            .into_iter().flatten()
        {
            for candidata in [base.join("medicci.db"), base.join("bd/medicci.db"),
                              base.join("../bd/medicci.db"), base.join("../../bd/medicci.db")] {
                if candidata.exists() {
                    return candidata;
                }
            }
        }
        PathBuf::from("medicci.db")
    });

    let datos = match Datos::abrir(&ruta) {
        Ok(d) => d,
        Err(e) => {
            eprintln!("No puedo abrir la base de datos {}: {}", ruta.display(), e);
            std::process::exit(1);
        }
    };

    let opciones = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title("Medicci")
            .with_inner_size(ventana())
            .with_decorations(false)
            .with_transparent(true)
            .with_resizable(false)
            .with_always_on_top()
            .with_taskbar(false),
        // sin esperar al refresco de pantalla: el chat y el resumen plegados son
        // ventanas que no se ven, y XWayland solo les da un refresco por segundo;
        // con vsync, cada fotograma de la animación esperaba ese segundo
        vsync: false,
        ..Default::default()
    };

    eframe::run_native(
        "Medicci",
        opciones,
        Box::new(move |cc| {
            fuentes(&cc.egui_ctx);
            estilo(&cc.egui_ctx);
            let mut app = Medicci::nuevo(datos, paciente, red);
            app.abierto = abierta;
            Ok(Box::new(app))
        }),
    )
}

#[cfg(test)]
mod pruebas {
    use super::*;

    fn con_paciente(ident: &str) -> Medicci {
        let datos = datos::base_de_prueba(&format!("resumen-{}", ident));
        let mut app = Medicci::nuevo(datos, Some(ident.into()), None);
        app.abre_resumen();
        app
    }

    /// Contra un servidor de verdad: `MEDICCI_SERVIDOR=http://localhost:8000 cargo test -- --ignored`
    #[test]
    #[ignore]
    fn conectada_resumen_contra_el_servidor() {
        let url = std::env::var("MEDICCI_SERVIDOR").unwrap_or("http://localhost:8000".into());
        let datos = datos::base_de_prueba("conectada");
        let mut app = Medicci::nuevo(datos, None, Some(Servidor::nuevo(&url)));
        let ctx = egui::Context::default();
        app.abre_paciente("129798");
        for _ in 0..200 {
            app.red(&ctx);
            if !app.filas_red.is_empty() && !app.alertas_red.is_empty() {
                break;
            }
            std::thread::sleep(std::time::Duration::from_millis(50));
        }
        assert!(!app.filas_red.is_empty(), "no han llegado los datos del servidor");
        assert!(app.hilo.iter().any(|m| m.tarjeta), "al abrir, el resumen sale solo");
        app.abre_resumen();
        assert!(app.hilo.last().is_some_and(|m| m.tarjeta), "Resumen no pone la tarjeta en el chat");
        let r = app.resumen.as_ref().expect("hay resumen");
        println!("motivo: {:?} · alertas {} · documentos {} · pruebas {} · pendiente {}",
                 r.motivo.as_ref().map(|m| &m.valor), r.alertas.len(), r.documentos.len(),
                 r.pruebas.len(), r.faltas.len());
        assert!(!r.alertas.is_empty());
        app.muestra(Pestana::Pruebas);
        assert!(app.pestana == Pestana::Pruebas);
    }

    #[test]
    fn conectada_el_resumen_sale_de_las_filas_del_servidor() {
        use serde_json::json;
        let f = |r: &str, t: &str, campos: serde_json::Value| FilaRed {
            referencia: r.into(), tabla: t.into(), texto: format!("Fila {}", r), campos,
        };
        let filas = vec![
            f("F1", "PACIENTES", json!({"edad": 60, "sexo": "F", "contacto": "LLAMADA",
                                        "accesibilidad": "MOVILIDAD_REDUCIDA"})),
            f("D1", "DERIVACIONES_AP", json!({"fecha": "2025-04-13 10:31:00", "servicio": "Dermatología",
                                              "prioridad": "PREFERENTE", "motivo": "Reanudar seguimiento",
                                              "observaciones": "Perdió la cita de revisión"})),
            f("C1", "CITAS", json!({"fecha": "2025-06-05", "servicio": "Dermatología", "estado": "NO_REALIZADA"})),
            f("P1", "PRUEBAS", json!({"tipo": "RADIOGRAFIA", "programada": "2025-05-23 09:45:00",
                                      "realizada": null, "estado": "PROGRAMADA", "resultado": false})),
        ];
        let p = Paciente { id: "P0129798".into(), edad: 60, ..Default::default() };
        let alertas = [("1 cita se cerró sin atenderle.".to_string(), vec!["C1".to_string()])];
        let r = resumen_de_red(&p, &filas, &alertas).expect("hay resumen");
        assert_eq!(r.paciente.sexo, "mujer");
        assert!(r.motivo.as_ref().unwrap().valor.starts_with("Reanudar seguimiento"));
        assert_eq!(r.documentos.iter().map(|d| d.id.as_str()).collect::<Vec<_>>(), ["C1", "P1", "D1"]);
        assert_eq!(r.alertas[0].documento.as_deref(), Some("C1"));
        assert!(r.marcados.contains(&"C1".to_string()));
        assert!(r.faltas[0].afirmacion.starts_with("Radiografía programada para el 23/05/2025"));
        assert!(r.pruebas[0].afirmacion.starts_with("Radiografía · pendiente"));
        assert!(r.para_la_consulta().iter().any(|c| c.valor == "Movilidad reducida"));
        assert!(resumen_de_red(&p, &[], &alertas).is_none(), "sin datos no hay resumen");
    }

    /// Sin glifo, egui pinta un cuadro: todo símbolo que escriba la burbuja,
    /// el servidor o la base tiene que estar en las fuentes.
    #[test]
    fn los_simbolos_tienen_glifo() {
        let ctx = egui::Context::default();
        fuentes(&ctx);
        let _ = ctx.run(Default::default(), |_| {});
        let simbolos = "←↑→↓−·—…•«»¿¡áéíóúñÁÉÍÓÚÑüº";
        for familia in [FontFamily::Proportional, negrita()] {
            for c in simbolos.chars() {
                assert!(ctx.fonts(|f| f.has_glyph(&FontId::new(12.0, familia.clone()), c)),
                        "falta «{}» en {:?}", c, familia);
            }
        }
    }

    #[test]
    fn las_letras_compuestas_se_escriben() {
        let ctx = egui::Context::default();
        let mut texto = String::from("cami");
        let mut entrada = egui::RawInput::default();
        entrada.events = vec![
            egui::Event::Ime(egui::ImeEvent::Commit("ó".into())),
            egui::Event::Text("n ".into()),
            egui::Event::Ime(egui::ImeEvent::Preedit("¨".into())),
            egui::Event::Ime(egui::ImeEvent::Commit("ü".into())),
            egui::Event::Text("ñÁ".into()),
        ];
        for eventos in [Vec::new(), std::mem::take(&mut entrada.events)] {
            let _ = ctx.run(egui::RawInput { events: eventos, ..Default::default() }, |ctx| {
                letras_compuestas(ctx);
                egui::CentralPanel::default().show(ctx, |ui| {
                    let r = ui.add(egui::TextEdit::singleline(&mut texto).id(egui::Id::new("t")));
                    r.request_focus();
                });
            });
        }
        assert_eq!(texto, "camión üñÁ");
    }

    #[test]
    fn la_pregunta_se_guarda_tal_cual() {
        let mut app = Medicci::nuevo(datos::base_de_prueba("preguntas"), Some("2856".into()), None);
        app.entrada = "¿Tiene alergias? Pingüino, ÑANDÚ".into();
        app.preguntar();
        let p = app.datos.preguntas("PAC-02856");
        assert_eq!(p, vec![("¿Tiene alergias? Pingüino, ÑANDÚ".to_string(), "hechos".to_string())]);
    }

    #[test]
    fn resumen_abre_la_ventana_sin_tocar_el_chat() {
        let datos = datos::base_de_prueba("flotante");
        let mut app = Medicci::nuevo(datos, Some("417".into()), None);
        let antes = app.hilo.len();
        app.resumen_flotante();
        assert!(app.ampliado);
        assert_eq!(app.hilo.len(), antes, "el chat no cambia");
    }

    #[test]
    fn el_resumen_sale_como_tarjeta_en_el_chat() {
        let mut app = con_paciente("2856");
        assert!(app.hilo.last().is_some_and(|m| m.tarjeta));
        assert!(!app.ampliado);
        // pedirlo otra vez no duplica la tarjeta
        app.abre_resumen();
        assert_eq!(app.hilo.iter().filter(|m| m.tarjeta).count(), 1);
        app.muestra(Pestana::Pruebas);
        assert!(app.pestana == Pestana::Pruebas);
        app.amplia();
        assert!(app.ampliado);
    }

    #[test]
    fn el_resumen_trae_alertas_contradicciones_y_analiticas_con_fuente() {
        for ident in ["417", "2856"] {
            let app = con_paciente(ident);
            let r = app.resumen.as_ref().expect("hay resumen");
            assert!(!r.alertas.is_empty(), "{}", ident);
            assert!(!r.contradicciones.is_empty(), "{}", ident);
            assert!(r.motivo.is_some(), "{}", ident);
            assert!(r.alertas.iter().any(|a| a.documento.is_some()), "{}", ident);
            for id in &r.marcados {
                assert!(r.documento(id).is_some(), "{}: {} no es suyo", ident, id);
            }
        }
        let app = con_paciente("2856");
        let r = app.resumen.as_ref().unwrap();
        assert!(r.labs.iter().any(|f| f.nombre == "Hemoglobina" && f.ahora.as_deref() == Some("9,6")));
        assert!(r.alertas.iter().any(|a| a.titulo.contains("Contrastes yodados")));
        let nota = texto_nota(r);
        assert!(nota.contains("[E03]") && nota.contains("No coincide con la derivación"));
    }

    #[test]
    fn la_afirmacion_se_parte_por_donde_toca() {
        let (a, b) = cabeza_y_resto("Sí tiene antecedentes digestivos de peso: angiodisplasia gástrica en 2019.");
        assert_eq!(a, "Sí tiene antecedentes digestivos de peso");
        assert_eq!(b.as_deref(), Some("Angiodisplasia gástrica en 2019."));
        // «H. pylori» no es el final de una frase
        let (a, b) = cabeza_y_resto("H. pylori negativo en biopsias antrales de 2012");
        assert_eq!(a, "H. pylori negativo en biopsias antrales de 2012");
        assert!(b.is_none());
    }
}
