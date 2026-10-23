//! Medicci · lector de historia clínica.
//!
//! Una burbuja con un médico, siempre abajo a la derecha y encima del HCE.
//! Al pulsarla se despliega el chat: arriba se escribe el número del
//! paciente y abajo se pregunta. RESUMEN abre una ventana flotante, en
//! mitad de la pantalla, con lo que el lector ha sacado de la historia.
//!
//!     medicci                       # abre con la base de al lado
//!     medicci --db ruta.db --paciente 417

#![cfg_attr(windows, windows_subsystem = "windows")]

mod datos;
mod lector;

use datos::{CampoFicha, Datos, Hecho, Paciente};
use eframe::egui;
use egui::emath::{easing, TSTransform};
use egui::{pos2, vec2, Align, Color32, FontFamily, FontId, Layout, Pos2, Rect, RichText,
           Rounding, Sense, Shape, Stroke, UiBuilder, Vec2};
use lector::{Lector, Respuesta};
use std::path::PathBuf;

// medidas, en puntos
const BOLA: f32 = 56.0;                     // diámetro de la burbuja
const PANEL: Vec2 = vec2(360.0, 540.0);     // el chat desplegado
const HUECO: f32 = 12.0;                    // entre el chat y la burbuja
const AIRE: f32 = 16.0;                     // borde transparente donde cabe la sombra
const MARGEN: f32 = 24.0;                   // de la burbuja al borde derecho
#[cfg(windows)]
const BAJO: f32 = 72.0;                     // de la burbuja al borde de abajo (barra de tareas)
#[cfg(not(windows))]
const BAJO: f32 = 24.0;
const HOJA: Vec2 = vec2(660.0, 740.0);      // la ventana del resumen

// la paleta de Medicci
const VERDE: Color32 = Color32::from_rgb(0x1a, 0x5c, 0x44);
const VERDE_OSC: Color32 = Color32::from_rgb(0x0e, 0x2d, 0x20);
const VERDE_2: Color32 = Color32::from_rgb(0x1d, 0x5a, 0x44);
const VERDE_CLARO: Color32 = Color32::from_rgb(0x9d, 0xc3, 0xb1);
const CREMA: Color32 = Color32::from_rgb(0xf6, 0xf3, 0xec);
const BLANCO: Color32 = Color32::from_rgb(0xff, 0xff, 0xff);
const LINEA: Color32 = Color32::from_rgb(0xe3, 0xde, 0xd1);
const LINEA_SUAVE: Color32 = Color32::from_rgb(0xef, 0xeb, 0xe1);
const ORO: Color32 = Color32::from_rgb(0xd9, 0xa3, 0x2b);
const ORO_SUAVE: Color32 = Color32::from_rgb(0xfb, 0xf0, 0xd5);
const ROJO: Color32 = Color32::from_rgb(0x9c, 0x3b, 0x23);
const GRIS: Color32 = Color32::from_rgb(0x6b, 0x7a, 0x72);
const TINTA: Color32 = Color32::from_rgb(0x1d, 0x2a, 0x24);
const TINTA_2: Color32 = Color32::from_rgb(0x3f, 0x4c, 0x45);
const MIO: Color32 = Color32::from_rgb(0xe4, 0xee, 0xe8);
const CITA_FONDO: Color32 = Color32::from_rgb(0xfb, 0xf7, 0xe8);

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
}

impl Mensaje {
    fn mio(t: impl Into<String>) -> Self {
        Self { mio: true, partes: vec![Parte::Texto(t.into())] }
    }
    fn suyo(partes: Vec<Parte>) -> Self {
        Self { mio: false, partes }
    }
}

/// Lo que enseña la ventana de resumen, ya leído de la base.
struct Resumen {
    paciente: Paciente,
    documentos: usize,
    desde: String,
    hasta: String,
    campos: Vec<CampoFicha>,
    historia: Vec<Hecho>,
    contradicciones: Vec<Hecho>,
    alarmas: Vec<Hecho>,
    faltas: Vec<Hecho>,
}

struct Medicci {
    datos: Datos,
    usuario: String,
    abierto: bool,
    t: f32,                         // 0 = burbuja, 1 = chat desplegado
    lista: bool,                    // la ventana ya tiene el tamaño que toca
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
    resumen_t: f32,
    resumen_pos: Option<Pos2>,
    pendiente: Option<String>,      // un tema pulsado en el resumen
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
        (Some(d), Some(f)) => match &h.servicio {
            Some(s) if !s.is_empty() => format!("¹ {} · {} · {}", d, f, s),
            _ => format!("¹ {} · {}", d, f),
        },
        _ => format!("¹ comprobado: no aparece en ninguno de los {} documentos", total),
    }
}

/// Tamaño de la ventana principal: solo la burbuja, o el chat encima de ella.
fn ventana(desplegada: bool) -> Vec2 {
    if desplegada {
        vec2(PANEL.x + 2.0 * AIRE, PANEL.y + HUECO + BOLA + 2.0 * AIRE)
    } else {
        vec2(BOLA + 2.0 * AIRE, BOLA + 2.0 * AIRE)
    }
}

impl Medicci {
    fn nuevo(datos: Datos, inicial: Option<String>) -> Self {
        let mut app = Self {
            usuario: std::env::var("USER").or_else(|_| std::env::var("USERNAME"))
                .unwrap_or_else(|_| "desconocido".into()),
            datos,
            abierto: false,
            t: 0.0,
            lista: false,
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
            resumen_t: 0.0,
            resumen_pos: None,
            pendiente: None,
        };
        app.saludo();
        if let Some(id) = inicial {
            app.abre_paciente(&id);
        }
        app
    }

    fn saludo(&mut self) {
        self.hilo.clear();
        self.hilo.push(Mensaje::suyo(vec![Parte::Texto(
            "Escriba arriba el número o el ID del paciente y leo su historia antes de que entre."
                .into())]));
    }

    fn abre_paciente(&mut self, ident: &str) {
        let ident = ident.trim();
        if ident.is_empty() {
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
                    Parte::Texto("Pulse RESUMEN para ver lo que hay que saber antes de que \
                                  entre, o pregunte por un dato: «GOT de la última analítica», \
                                  «¿alergias?», «¿qué falta por hacer?».".into()),
                ]));
                self.lector = Some(lector);
                self.paciente = Some(p);
                self.ident.clear();
                self.resumen = None;
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
        self.entrada.clear();
        self.saludo();
        self.foco_ident = true;
    }

    fn abre_resumen(&mut self) {
        let (Some(p), Some(l)) = (self.paciente.clone(), self.lector.as_ref()) else { return };
        self.datos.registra(&self.usuario, &p.id, "generar_resumen", None);
        let documentos = l.docs.len();
        let desde = l.docs.last().map(|d| año(&d.fecha)).unwrap_or_default();
        let hasta = l.docs.first().map(|d| año(&d.fecha)).unwrap_or_default();
        let mut vistos = Vec::new();
        let contradicciones = nuevos(self.datos.hechos(&p.id, &["contradiccion"]), &mut vistos);
        let alarmas = nuevos(self.datos.hechos(&p.id, &["alarma"]), &mut vistos);
        let faltas = nuevos(self.datos.hechos(&p.id, &["falta"]), &mut vistos);
        let historia = nuevos(self.datos.hechos(&p.id, &["resumen"]), &mut vistos);
        let campos = self.datos.ficha(&p.id);
        let con_fuente = campos.iter().filter(|c| c.documento.is_some()).count() + vistos.len();

        self.hilo.push(Mensaje::mio("RESUMEN"));
        let mut partes = vec![Parte::Texto(format!(
            "Resumen abierto en el centro de la pantalla: {} datos, cada uno con su fuente.",
            con_fuente))];
        if !contradicciones.is_empty() {
            partes.push(Parte::Aviso(format!(
                "{} no cuadran con la derivación.", contradicciones.len())));
        }
        self.hilo.push(Mensaje::suyo(partes));
        self.bajar = true;

        self.resumen = Some(Resumen {
            paciente: p, documentos, desde, hasta, campos,
            historia, contradicciones, alarmas, faltas,
        });
        self.resumen_t = 0.0;
        self.resumen_pos = None;
    }

    fn preguntar(&mut self) {
        let pregunta = self.entrada.trim().to_string();
        if pregunta.is_empty() {
            return;
        }
        self.entrada.clear();
        let baja = lector::sin_tildes(&pregunta);
        if baja.contains("resumen") || baja.contains("resume") || baja.contains("ficha") {
            return self.abre_resumen();
        }
        self.hilo.push(Mensaje::mio(pregunta.clone()));
        self.bajar = true;
        let (Some(p), Some(l)) = (self.paciente.clone(), self.lector.as_ref()) else { return };
        let total = l.docs.len();
        let mut partes = Vec::new();
        match l.responde(&self.datos, &pregunta) {
            Respuesta::NoConsta => {
                self.datos.registra(&self.usuario, &p.id, "preguntar", Some("no_consta"));
                partes.push(Parte::Aviso("No consta en la historia.".into()));
                partes.push(Parte::Texto(format!(
                    "Ninguna frase de los {} documentos responde a eso, y no lo voy a deducir.",
                    total)));
            }
            Respuesta::Valores(valores) => {
                self.datos.registra(&self.usuario, &p.id, "preguntar", Some("valor"));
                for v in valores {
                    partes.push(Parte::Valor(format!("{} {} {}", v.nombre, v.valor, v.unidad)
                        .trim().to_string()));
                    partes.push(Parte::Texto(format!("{} del {}", v.tipo, v.fecha)));
                    partes.push(Parte::Cita(v.linea));
                    partes.push(Parte::Fuente(format!("¹ {} · {}", v.documento, v.servicio)));
                }
            }
            Respuesta::Hechos(hechos) => {
                self.datos.registra(&self.usuario, &p.id, "preguntar", Some("hechos"));
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
                self.datos.registra(&self.usuario, &p.id, "preguntar", Some("literal"));
                partes.push(Parte::Rotulo("No lo tengo ordenado. Esto es lo que pone, tal cual:".into()));
                for (doc, frase) in frases {
                    partes.push(Parte::Cita(frase));
                    partes.push(Parte::Fuente(format!("¹ {} · {} · {}", doc.id, doc.fecha, doc.tipo)));
                }
            }
        }
        self.hilo.push(Mensaje::suyo(partes));
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
    // las de egui se quedan detrás, para los símbolos que Noto no tenga
    let resto = f.families[&FontFamily::Proportional].clone();
    f.families.get_mut(&FontFamily::Proportional).unwrap().insert(0, "noto".into());
    f.families.get_mut(&FontFamily::Monospace).unwrap().insert(0, "noto_mono".into());
    let mut gruesa = vec!["noto_negrita".to_string()];
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
    v.faint_bg_color = CREMA;
    v.selection.bg_fill = Color32::from_rgb(0xcf, 0xe3, 0xd8);
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
                .color(Color32::from_rgb(0x33, 0x45, 0x4f)));
        })
        .response;
    ui.painter().vline(r.rect.left() + 1.0, r.rect.y_range(), trazo(2.0, ORO));
}

fn fuente(ui: &mut egui::Ui, t: &str) {
    ui.label(RichText::new(t).monospace().size(10.5).color(VERDE_2));
}

// =====================================================================
//  La burbuja y el chat
// =====================================================================

impl Medicci {
    /// Mantiene la ventana anclada abajo a la derecha con el tamaño que toca.
    /// El gestor de ventanas puede tardar en hacer caso: se reintenta unas veces.
    fn coloca(&mut self, ctx: &egui::Context) {
        let tam = ventana(self.abierto || self.t > 0.0);
        let (monitor, real) = ctx.input(|i| (i.viewport().monitor_size, i.viewport().outer_rect));
        let Some(monitor) = monitor else {
            self.lista = false;
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
            self.lista = true;
            return;
        }
        self.lista = false;
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
                self.abierto = !self.abierto;
                if self.abierto {
                    if self.paciente.is_some() { self.foco_chat = true } else { self.foco_ident = true }
                }
            }
        });
    }

    fn panel(&mut self, ctx: &egui::Context) {
        let pantalla = ctx.screen_rect();
        let abajo = pantalla.bottom() - AIRE - BOLA - HUECO;
        let caja = Rect::from_min_max(pos2(pantalla.right() - AIRE - PANEL.x, abajo - PANEL.y),
                                      pos2(pantalla.right() - AIRE, abajo));
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
                p.rect(caja, 16.0, CREMA, trazo(1.0, LINEA));

                let cabeza = Rect::from_min_size(caja.min, vec2(caja.width(), 64.0));
                let ficha = Rect::from_min_size(cabeza.left_bottom(), vec2(caja.width(), 52.0));
                let pie = Rect::from_min_max(pos2(caja.left(), caja.bottom() - 60.0), caja.max);
                let hilo = Rect::from_min_max(ficha.left_bottom(), pie.right_top());
                self.cabecera(ui, cabeza);
                self.barra_paciente(ui, ficha);
                self.conversacion(ui, hilo);
                self.redactor(ui, pie);
            });
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
               "Lee la historia y cita la fuente", FontId::proportional(11.5), VERDE_CLARO);

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
                    ui.label(texto(format!("{} años · {} · {} documentos", pac.edad, pac.sexo, docs),
                                   11.0, GRIS, false));
                });
                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    cambiar = enlace(ui, "Cambiar");
                });
            } else {
                ui.label(texto("Paciente", 12.0, GRIS, true));
                ui.add_space(2.0);
                egui::Frame::none()
                    .fill(CREMA)
                    .stroke(trazo(1.0, LINEA))
                    .rounding(8.0)
                    .inner_margin(egui::Margin::symmetric(10.0, 6.0))
                    .show(ui, |ui| {
                        let campo = egui::TextEdit::singleline(&mut self.ident)
                            .hint_text(texto("Número o ID, p. ej. 417", 13.0, GRIS, false))
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
        ui.allocate_new_ui(UiBuilder::new().max_rect(caja.shrink2(vec2(12.0, 0.0))), |ui| {
            egui::ScrollArea::vertical()
                .id_salt("hilo")
                .auto_shrink([false, false])
                .stick_to_bottom(true)
                .show(ui, |ui| {
                    ui.add_space(12.0);
                    let ancho = ui.available_width();
                    for m in &self.hilo {
                        mensaje(ui, m, ancho);
                        ui.add_space(8.0);
                    }
                    if bajar {
                        ui.scroll_to_cursor(Some(Align::BOTTOM));
                    }
                });
        });
    }

    fn redactor(&mut self, ui: &mut egui::Ui, caja: Rect) {
        let p = ui.painter().clone();
        p.rect_filled(caja, Rounding { nw: 0.0, ne: 0.0, sw: 16.0, se: 16.0 }, BLANCO);
        p.hline(caja.x_range(), caja.top() + 0.5, trazo(1.0, LINEA));
        let hay = self.paciente.is_some();
        let mut resumen = false;
        let mut enviar = false;
        ui.allocate_new_ui(UiBuilder::new().max_rect(caja.shrink2(vec2(12.0, 12.0)))
                               .layout(Layout::left_to_right(Align::Center)), |ui| {
            resumen = boton(ui, "RESUMEN", ORO, TINTA, hay);
            egui::Frame::none()
                .fill(CREMA)
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
            self.abre_resumen();
        }
        if enviar {
            self.preguntar();
        }
    }
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
//  La ventana del resumen
// =====================================================================

/// Un título de sección, con una línea a la derecha.
fn seccion(ui: &mut egui::Ui, titulo: &str, nota: &str) {
    ui.add_space(18.0);
    ui.horizontal(|ui| {
        ui.label(texto(titulo, 14.5, TINTA, true));
        if !nota.is_empty() {
            ui.label(texto(nota, 11.5, GRIS, false));
        }
    });
    ui.add_space(6.0);
}

fn columna(ui: &mut egui::Ui, ancho: f32, alin: Align, dentro: impl FnOnce(&mut egui::Ui)) {
    ui.allocate_ui_with_layout(vec2(ancho, 0.0), Layout::top_down(alin), |ui| {
        ui.set_width(ancho);
        ui.spacing_mut().item_spacing.y = 2.0;
        dentro(ui);
    });
}

fn hoja_blanca(ui: &mut egui::Ui, dentro: impl FnOnce(&mut egui::Ui)) {
    egui::Frame::none()
        .fill(BLANCO)
        .stroke(trazo(1.0, LINEA))
        .rounding(10.0)
        .inner_margin(egui::Margin::symmetric(16.0, 2.0))
        .show(ui, |ui| {
            ui.set_width(ui.available_width());
            dentro(ui);
        });
}

fn separa(ui: &mut egui::Ui, primera: bool) {
    if !primera {
        let y = ui.cursor().top();
        ui.painter().hline(ui.max_rect().x_range(), y, trazo(1.0, LINEA_SUAVE));
    }
}

/// Tabla de tres columnas: qué, cuánto y de dónde sale.
fn tabla(ui: &mut egui::Ui, filas: &[&CampoFicha]) {
    hoja_blanca(ui, |ui| {
        for (i, c) in filas.iter().enumerate() {
            separa(ui, i == 0);
            ui.add_space(8.0);
            ui.horizontal_top(|ui| {
                let total = ui.available_width();
                let hueco = ui.spacing().item_spacing.x;
                columna(ui, 132.0, Align::Min, |ui| {
                    ui.label(texto(&c.etiqueta, 12.0, GRIS, false));
                });
                columna(ui, total - 132.0 - 96.0 - 2.0 * hueco, Align::Min, |ui| {
                    let vacio = sin_dato(&c.valor);
                    let tam = if c.destacado { 14.5 } else { 13.0 };
                    // el dato corto va en negrita; una frase entera, no
                    let corto = c.valor.chars().count() <= 40;
                    ui.label(texto(&c.valor, tam, if vacio { GRIS } else { TINTA }, !vacio && corto));
                    if let Some(d) = &c.detalle {
                        ui.label(texto(d, 12.0, TINTA_2, false));
                    }
                });
                columna(ui, 96.0, Align::Max, |ui| {
                    match (&c.documento, &c.fecha_doc) {
                        (Some(d), Some(f)) => fuente(ui, &format!("¹ {} · {}", d, año(f))),
                        (Some(d), None) => fuente(ui, &format!("¹ {}", d)),
                        _ => { ui.label(texto("—", 11.0, LINEA, false)); }
                    }
                });
            });
            ui.add_space(8.0);
        }
    });
}

/// Casillas en fila: alergias, constantes...
fn casillas(ui: &mut egui::Ui, celdas: &[&CampoFicha]) {
    if celdas.is_empty() {
        return;
    }
    let hueco = 8.0;
    let n = celdas.len() as f32;
    let ancho = ((ui.available_width() - hueco * (n - 1.0)) / n).floor();
    ui.horizontal_top(|ui| {
        ui.spacing_mut().item_spacing.x = hueco;
        for c in celdas {
            egui::Frame::none()
                .fill(BLANCO)
                .stroke(trazo(1.0, if c.destacado { ROJO } else { LINEA }))
                .rounding(10.0)
                .inner_margin(egui::Margin::symmetric(11.0, 9.0))
                .show(ui, |ui| {
                    ui.set_width(ancho - 22.0);
                    ui.set_min_height(54.0);
                    ui.vertical(|ui| {
                        ui.spacing_mut().item_spacing.y = 2.0;
                        ui.label(texto(&c.etiqueta, 11.5, GRIS, false));
                        let vacio = sin_dato(&c.valor);
                        let color = if vacio { GRIS } else if c.destacado { ROJO } else { TINTA };
                        ui.label(texto(&c.valor, if vacio { 13.0 } else { 15.0 }, color, !vacio));
                        if let Some(d) = &c.documento {
                            fuente(ui, &format!("¹ {}", d));
                        }
                    });
                });
        }
    });
}

fn bloque_hechos(ui: &mut egui::Ui, hechos: &[Hecho], rojo: bool, total: usize) {
    hoja_blanca(ui, |ui| {
        for (i, h) in hechos.iter().enumerate() {
            separa(ui, i == 0);
            ui.add_space(9.0);
            let r = ui.vertical(|ui| {
                ui.spacing_mut().item_spacing.y = 4.0;
                ui.label(texto(&h.afirmacion, 13.0, if rojo { ROJO } else { TINTA }, rojo));
                if let Some(c) = &h.cita {
                    cita(ui, c);
                }
                fuente(ui, &fuente_hecho(h, total));
            }).response;
            if rojo {
                ui.painter().vline(r.rect.left() - 9.0, r.rect.y_range(), trazo(2.5, ROJO));
            }
            ui.add_space(9.0);
        }
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

/// El contenido de la hoja de resumen. Devuelve el tema pulsado, si alguno.
fn hoja_resumen(ui: &mut egui::Ui, r: &Resumen) -> Option<String> {
    let mut tema = None;
    let de = |bloque: &str| -> Vec<&CampoFicha> {
        r.campos.iter().filter(|c| c.bloque == bloque).collect()
    };
    let de_seccion = |seccion: &str| -> Vec<&CampoFicha> {
        r.campos.iter()
            .filter(|c| c.bloque == "por_que_viene" && c.seccion.as_deref() == Some(seccion))
            .collect()
    };

    // la edad y el sexo ya van en la cabecera
    let chips: Vec<&CampoFicha> = de("cabecera").into_iter()
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

    let antes = de("antes_de_empezar");
    if !antes.is_empty() {
        seccion(ui, "Antes de empezar", "");
        casillas(ui, &antes);
    }

    seccion(ui, "Por qué viene hoy", "");
    let motivo = de_seccion("motivo");
    if motivo.is_empty() {
        hoja_blanca(ui, |ui| {
            ui.add_space(9.0);
            ui.label(texto(&r.paciente.motivo, 13.0, TINTA, true));
            ui.add_space(9.0);
        });
    } else {
        tabla(ui, &motivo);
    }
    let cambios = de_seccion("cambios");
    if !cambios.is_empty() {
        seccion(ui, "Qué ha cambiado", "desde la última vez");
        tabla(ui, &cambios);
    }

    if !r.contradicciones.is_empty() {
        seccion(ui, "Lo que no cuadra con la derivación", "");
        bloque_hechos(ui, &r.contradicciones, true, r.documentos);
    }
    if !r.alarmas.is_empty() {
        seccion(ui, "Datos de alarma", "");
        bloque_hechos(ui, &r.alarmas, false, r.documentos);
    }

    let problemas = de_seccion("problemas");
    if !problemas.is_empty() {
        seccion(ui, "Problemas activos", "");
        tabla(ui, &problemas);
    }
    let medicacion = de_seccion("medicacion");
    if !medicacion.is_empty() {
        seccion(ui, "Medicación", "");
        tabla(ui, &medicacion);
    }

    let constantes = de("constantes");
    if !constantes.is_empty() {
        seccion(ui, "Constantes", "la última toma");
        casillas(ui, &constantes);
    }

    if !r.historia.is_empty() {
        seccion(ui, "Qué le ha pasado", "");
        bloque_hechos(ui, &r.historia, false, r.documentos);
    }

    let vida = de("vida");
    if !vida.is_empty() {
        seccion(ui, "Su vida, en una línea", "");
        tabla(ui, &vida);
    }

    if !r.faltas.is_empty() {
        seccion(ui, "Lo que falta por hacer", "");
        bloque_hechos(ui, &r.faltas, false, r.documentos);
    }

    let temas = de("temas");
    if !temas.is_empty() {
        seccion(ui, "Memoria profunda", "toque un tema y se pregunta en el chat");
        ui.horizontal_wrapped(|ui| {
            ui.spacing_mut().item_spacing = vec2(6.0, 6.0);
            for c in temas {
                let b = egui::Button::new(texto(&c.valor, 12.0, VERDE_2, false))
                    .fill(BLANCO)
                    .stroke(trazo(1.0, LINEA))
                    .rounding(14.0);
                if ui.add(b).on_hover_cursor(egui::CursorIcon::PointingHand).clicked() {
                    tema = Some(c.valor.clone());
                }
            }
        });
    }
    ui.add_space(20.0);
    tema
}

impl Medicci {
    fn ventana_resumen(&mut self, ctx: &egui::Context) {
        let Some(resumen) = self.resumen.take() else { return };
        let monitor = ctx.input(|i| i.viewport().monitor_size).unwrap_or(vec2(1440.0, 900.0));
        let hoja = vec2(HOJA.x.min(monitor.x - 80.0), HOJA.y.min(monitor.y - 80.0));
        let tam = hoja + Vec2::splat(2.0 * AIRE);
        let pos = *self.resumen_pos.get_or_insert_with(|| {
            pos2(((monitor.x - tam.x) / 2.0).round(), ((monitor.y - tam.y) / 2.0).round())
        });
        let constructor = egui::ViewportBuilder::default()
            .with_title("Medicci · Resumen")
            .with_inner_size(tam)
            .with_position(pos)
            .with_decorations(false)
            .with_transparent(true)
            .with_resizable(false)
            .with_always_on_top()
            .with_taskbar(false);

        let dt = ctx.input(|i| i.stable_dt).min(0.05);
        self.resumen_t = (self.resumen_t + dt / 0.22).min(1.0);
        let e = easing::cubic_out(self.resumen_t);
        let mut cerrar = false;
        let mut tema = None;

        ctx.show_viewport_immediate(egui::ViewportId::from_hash_of("medicci_resumen"), constructor,
                                    |ctx, _clase| {
            if ctx.input(|i| i.viewport().close_requested() || i.key_pressed(egui::Key::Escape)) {
                cerrar = true;
            }
            if e < 1.0 {
                ctx.request_repaint();
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
                    p.rect(caja, 14.0, CREMA, trazo(1.0, LINEA));

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
                           format!("Resumen de entrada · {}", pac.id),
                           FontId::new(17.0, negrita()), BLANCO);
                    p.text(pos2(c.x + 31.0, cabeza.center().y + 12.0), egui::Align2::LEFT_CENTER,
                           format!("{} años · {} · {} documentos leídos, de {} a {}",
                                   pac.edad, pac.sexo, resumen.documentos, resumen.desde, resumen.hasta),
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

                    let cuerpo = Rect::from_min_max(cabeza.left_bottom(), caja.max).shrink2(vec2(6.0, 4.0));
                    ui.allocate_new_ui(UiBuilder::new().max_rect(cuerpo), |ui| {
                        egui::ScrollArea::vertical()
                            .id_salt("resumen")
                            .auto_shrink([false, false])
                            .show(ui, |ui| {
                                egui::Frame::none()
                                    .inner_margin(egui::Margin::symmetric(16.0, 8.0))
                                    .show(ui, |ui| {
                                        ui.set_width(ui.available_width());
                                        tema = hoja_resumen(ui, &resumen);
                                    });
                            });
                    });
                });
        });

        if let Some(t) = tema {
            self.pendiente = Some(t);
        }
        if !cerrar {
            self.resumen = Some(resumen);
        }
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
        // el despliegue avanza cuando la ventana ya ha crecido
        let dt = ctx.input(|i| i.stable_dt).min(0.05);
        if self.abierto {
            if self.lista {
                self.t = (self.t + dt / 0.26).min(1.0);
            }
        } else {
            self.t = (self.t - dt / 0.18).max(0.0);
        }
        if (self.abierto && self.t < 1.0) || (!self.abierto && self.t > 0.0) {
            ctx.request_repaint();
        }

        self.coloca(ctx);
        self.burbuja(ctx);
        if self.t > 0.0 {
            self.panel(ctx);
        }
        self.ventana_resumen(ctx);

        if let Some(tema) = self.pendiente.take() {
            self.abierto = true;
            self.entrada = tema;
            self.preguntar();
        }
    }
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
    let mut args = std::env::args().skip(1);
    while let Some(a) = args.next() {
        match a.as_str() {
            "--db" => ruta = args.next().map(PathBuf::from),
            "--paciente" => paciente = args.next(),
            "--ayuda" | "-h" | "--help" => {
                println!("medicci [--db ruta.db] [--paciente 417]");
                return Ok(());
            }
            _ => {}
        }
    }
    let ruta = ruta.unwrap_or_else(|| {
        let exe = std::env::current_exe().unwrap_or_default();
        for base in [exe.parent().map(|p| p.to_path_buf()), std::env::current_dir().ok()]
            .into_iter().flatten()
        {
            for candidata in [base.join("medicci.db"), base.join("bd/jano.db"),
                              base.join("../bd/jano.db"), base.join("../../bd/jano.db")] {
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
            .with_inner_size(ventana(false))
            .with_decorations(false)
            .with_transparent(true)
            .with_resizable(false)
            .with_always_on_top()
            .with_taskbar(false),
        ..Default::default()
    };

    eframe::run_native(
        "Medicci",
        opciones,
        Box::new(move |cc| {
            fuentes(&cc.egui_ctx);
            estilo(&cc.egui_ctx);
            Ok(Box::new(Medicci::nuevo(datos, paciente)))
        }),
    )
}
