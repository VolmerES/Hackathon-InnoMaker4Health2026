//! Medicci conectada: la burbuja habla con el servidor de Medicci, el mismo que
//! usa la pestaña web (mvp/servidor.py). Un cerebro y dos caras: la pestaña,
//! cuando HCIS va en el navegador, y la burbuja, cuando no.
//!
//! Las llamadas van en otro hilo para que la burbuja no se congele mientras
//! contesta el modelo; lo que llega se recoge en cada fotograma.
//!
//! Cada sesión va a nombre del médico (el usuario de Windows): el servidor
//! apunta en su registro de accesos todo lo que abre, ve y pregunta.

use eframe::egui;
use serde_json::{json, Value};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::time::Duration;

pub struct Fuente {
    pub referencia: String,
    pub texto: String,
}

pub struct Alerta {
    pub texto: String,
    pub alta: bool,
    pub fuentes: Vec<Fuente>,
}

/// Una fila de los datos del paciente, tal como la ve el modelo, con sus campos.
pub struct FilaRed {
    pub referencia: String,
    pub tabla: String,
    pub texto: String,
    pub campos: Value,
}

pub enum Llega {
    Sesion { sesion: String, nhc: String, edad: i64, filas: i64 },
    Filas { sesion: String, filas: Vec<FilaRed> },
    Tarjeta { sesion: String, alertas: Vec<Alerta>, mas: i64 },
    Respuesta { texto: String, consta: bool, fuentes: Vec<Fuente>, segundos: f64 },
    Paso { n: i64, accion: String, texto: String },   // la simulación (mvp/simular.py)
    Dictado(Result<String, String>),                 // el texto de lo dictado, o por qué no
    Fallo(String),
}

pub struct Servidor {
    base: String,
    tx: Sender<Llega>,
    pub rx: Receiver<Llega>,
}

fn agente(segundos: u64) -> ureq::Agent {
    ureq::AgentBuilder::new().timeout(Duration::from_secs(segundos)).build()
}

fn leer(r: Result<ureq::Response, ureq::Error>) -> Result<Value, String> {
    match r {
        Ok(resp) => resp.into_json::<Value>().map_err(|e| e.to_string()),
        Err(ureq::Error::Status(_, resp)) => {
            let v: Value = resp.into_json().unwrap_or(Value::Null);
            Err(v["error"].as_str().unwrap_or("El servidor de Medicci ha dado un error.").to_string())
        }
        Err(e) => Err(format!("No llego al servidor de Medicci ({}).", e)),
    }
}

fn fuentes(v: &Value) -> Vec<Fuente> {
    v.as_array()
        .map(|a| {
            a.iter()
                .map(|f| Fuente {
                    referencia: f["ref"].as_str().unwrap_or_default().to_string(),
                    texto: f["texto"].as_str().unwrap_or_default().to_string(),
                })
                .collect()
        })
        .unwrap_or_default()
}

fn tarjeta(base: &str, sesion: &str) -> Llega {
    match leer(agente(15).get(&format!("{base}/api/sesion/{sesion}/tarjeta")).call()) {
        Ok(v) => Llega::Tarjeta {
            sesion: sesion.to_string(),
            alertas: v["alertas"]
                .as_array()
                .map(|a| {
                    a.iter()
                        .map(|x| Alerta {
                            texto: x["texto"].as_str().unwrap_or_default().to_string(),
                            alta: x["prioridad"].as_i64().unwrap_or(0) >= 75,
                            fuentes: fuentes(&x["fuentes"]),
                        })
                        .collect()
                })
                .unwrap_or_default(),
            mas: v["mas"].as_i64().unwrap_or(0),
        },
        Err(e) => Llega::Fallo(e),
    }
}

fn filas(base: &str, sesion: &str) -> Llega {
    match leer(agente(30).get(&format!("{base}/api/sesion/{sesion}/filas")).call()) {
        Ok(v) => Llega::Filas {
            sesion: sesion.to_string(),
            filas: v["filas"]
                .as_array()
                .map(|a| {
                    a.iter()
                        .map(|f| FilaRed {
                            referencia: f["ref"].as_str().unwrap_or_default().to_string(),
                            tabla: f["tabla"].as_str().unwrap_or_default().to_string(),
                            texto: f["texto"].as_str().unwrap_or_default().to_string(),
                            campos: f["campos"].clone(),
                        })
                        .collect()
                })
                .unwrap_or_default(),
        },
        Err(e) => Llega::Fallo(e),
    }
}

/// Los NHC de los CSV son «P» y siete cifras: «129798» vale por «P0129798».
pub fn nhc(ident: &str) -> String {
    let limpio: String = ident.trim().to_uppercase().chars().filter(|c| !c.is_whitespace()).collect();
    let cifras = limpio.trim_start_matches('P');
    match cifras.parse::<u32>() {
        Ok(n) if !cifras.is_empty() && cifras.chars().all(|c| c.is_ascii_digit()) => format!("P{:07}", n),
        _ => limpio,
    }
}

/// Una clave de medicci.conf (CLAVE=valor; # comenta). El fichero lo pone
/// Informática al lado del ejecutable: así se le dice a la burbuja adónde
/// mandar las peticiones sin tocar nada más.
pub fn leer_clave(texto: &str, clave: &str) -> Option<String> {
    texto.lines().find_map(|linea| {
        let linea = linea.split(" #").next().unwrap_or("").trim();
        let (k, v) = linea.split_once('=')?;
        (k.trim() == clave && !v.trim().is_empty()).then(|| v.trim().to_string())
    })
}

/// Busca medicci.conf al lado del ejecutable, en la carpeta de arranque y en la
/// de arriba (la raíz del repositorio, cuando se arranca con just).
pub fn de_la_configuracion(clave: &str) -> Option<String> {
    let mut sitios = Vec::new();
    if let Some(dir) = std::env::current_exe().ok().as_deref().and_then(|e| e.parent()) {
        sitios.push(dir.join("medicci.conf"));
    }
    if let Ok(dir) = std::env::current_dir() {
        sitios.push(dir.join("medicci.conf"));
        sitios.push(dir.join("..").join("medicci.conf"));
    }
    sitios.iter()
        .filter_map(|ruta| std::fs::read_to_string(ruta).ok())
        .find_map(|texto| leer_clave(&texto, clave))
}

impl Servidor {
    pub fn nuevo(base: &str) -> Self {
        let (tx, rx) = channel();
        Self { base: base.trim_end_matches('/').to_string(), tx, rx }
    }

    /// ¿Responde? Se mira al arrancar: si no, la burbuja usa su base local.
    pub fn responde(&self) -> bool {
        agente(3).get(&format!("{}/api/estado", self.base)).call().is_ok()
    }

    fn en_hilo(&self, ctx: &egui::Context, trabajo: impl FnOnce(&str) -> Vec<Llega> + Send + 'static) {
        let (base, tx, ctx) = (self.base.clone(), self.tx.clone(), ctx.clone());
        std::thread::spawn(move || {
            for llegada in trabajo(&base) {
                let _ = tx.send(llegada);
            }
            ctx.request_repaint();
        });
    }

    /// Abre una sesión atada al paciente y al médico, y trae su tarjeta y sus datos.
    pub fn abrir(&self, ctx: &egui::Context, nhc: String, medico: String) {
        self.en_hilo(ctx, move |base| {
            let cuerpo = json!({ "nhc": nhc, "medico": medico, "origen": "burbuja" });
            let s = match leer(agente(15).post(&format!("{base}/api/sesion")).send_json(cuerpo)) {
                Ok(v) => v,
                Err(e) => return vec![Llega::Fallo(e)],
            };
            let sesion = s["sesion"].as_str().unwrap_or_default().to_string();
            vec![
                Llega::Sesion {
                    sesion: sesion.clone(),
                    nhc: s["nhc"].as_str().unwrap_or_default().to_string(),
                    edad: s["edad"].as_i64().unwrap_or(0),
                    filas: s["filas"].as_i64().unwrap_or(0),
                },
                tarjeta(base, &sesion),
                filas(base, &sesion),
            ]
        });
    }

    /// Vuelve a traer los datos: ha llegado algo nuevo por HL7.
    pub fn filas(&self, ctx: &egui::Context, sesion: String) {
        self.en_hilo(ctx, move |base| vec![filas(base, &sesion)]);
    }

    /// El paso en el que va la simulación del día del médico.
    pub fn paso(&self, ctx: &egui::Context) {
        self.en_hilo(ctx, move |base| match leer(agente(3).get(&format!("{base}/api/simulacion")).call()) {
            Ok(v) => vec![Llega::Paso {
                n: v["n"].as_i64().unwrap_or(0),
                accion: v["accion"].as_str().unwrap_or_default().to_string(),
                texto: v["texto"].as_str().unwrap_or_default().to_string(),
            }],
            Err(_) => Vec::new(),
        });
    }

    /// Apunta en el registro de accesos lo que el médico hace en pantalla.
    pub fn registra(&self, ctx: &egui::Context, sesion: String, accion: &'static str) {
        self.en_hilo(ctx, move |base| {
            let _ = agente(5).post(&format!("{base}/api/sesion/{sesion}/registro"))
                .send_json(json!({ "accion": accion }));
            Vec::new()
        });
    }

    pub fn tarjeta(&self, ctx: &egui::Context, sesion: String) {
        self.en_hilo(ctx, move |base| vec![tarjeta(base, &sesion)]);
    }

    /// Manda lo dictado (WAV) y trae el texto: el médico lo revisa antes de preguntar.
    pub fn dictar(&self, ctx: &egui::Context, sesion: String, audio: Vec<u8>) {
        self.en_hilo(ctx, move |base| {
            let url = format!("{base}/api/sesion/{sesion}/voz");
            let r = agente(90).post(&url).set("Content-Type", "audio/wav").send_bytes(&audio);
            vec![Llega::Dictado(leer(r).map(|v| v["texto"].as_str().unwrap_or_default().trim().to_string()))]
        });
    }

    pub fn preguntar(&self, ctx: &egui::Context, sesion: String, texto: String) {
        self.en_hilo(ctx, move |base| {
            let url = format!("{base}/api/sesion/{sesion}/pregunta");
            vec![match leer(agente(120).post(&url).send_json(json!({ "texto": texto }))) {
                Ok(v) => Llega::Respuesta {
                    texto: v["respuesta"].as_str().unwrap_or_default().to_string(),
                    consta: v["consta"].as_bool().unwrap_or(false),
                    fuentes: fuentes(&v["fuentes"]),
                    segundos: v["segundos"].as_f64().unwrap_or(0.0),
                },
                Err(e) => Llega::Fallo(e),
            }]
        });
    }
}

#[cfg(test)]
mod pruebas {
    use super::*;

    fn espera(s: &Servidor) -> Llega {
        s.rx.recv_timeout(Duration::from_secs(60)).expect("el servidor contesta")
    }

    /// Contra un servidor de verdad: `MEDICCI_SERVIDOR=http://localhost:8000 cargo test -- --ignored`
    #[test]
    #[ignore]
    fn contra_el_servidor() {
        let url = std::env::var("MEDICCI_SERVIDOR").unwrap_or("http://localhost:8000".into());
        let s = Servidor::nuevo(&url);
        assert!(s.responde(), "no hay servidor en {}", url);
        let ctx = egui::Context::default();
        s.abrir(&ctx, nhc("129798"), "prueba-burbuja".into());
        let Llega::Sesion { sesion, nhc, edad, .. } = espera(&s) else { panic!("esperaba la sesión") };
        println!("sesión {} · {} · {} años", sesion, nhc, edad);
        let Llega::Tarjeta { alertas, .. } = espera(&s) else { panic!("esperaba la tarjeta") };
        for a in &alertas {
            println!("  • {}  [{}]", a.texto, a.fuentes.iter().map(|f| f.referencia.as_str()).collect::<Vec<_>>().join(", "));
        }
        assert!(!alertas.is_empty());
        let Llega::Filas { filas, .. } = espera(&s) else { panic!("esperaba los datos") };
        println!("  {} filas", filas.len());
        assert!(filas.iter().any(|f| f.tabla == "PACIENTES"));
        s.preguntar(&ctx, sesion, "¿Tiene alergias?".into());
        let Llega::Respuesta { texto, consta, .. } = espera(&s) else { panic!("esperaba la respuesta") };
        println!("  ¿Tiene alergias? → {}", texto);
        assert!(!consta);
    }

    /// El dictado contra un servidor de verdad, con un WAV grabado:
    /// `MEDICCI_WAV=pregunta.wav cargo test -- --ignored dictado`
    #[test]
    #[ignore]
    fn dictado_contra_el_servidor() {
        let url = std::env::var("MEDICCI_SERVIDOR").unwrap_or("http://localhost:8000".into());
        let audio = std::fs::read(std::env::var("MEDICCI_WAV").expect("falta MEDICCI_WAV")).unwrap();
        let s = Servidor::nuevo(&url);
        let ctx = egui::Context::default();
        s.abrir(&ctx, nhc("129798"), "prueba-burbuja".into());
        let Llega::Sesion { sesion, .. } = espera(&s) else { panic!("esperaba la sesión") };
        while let Ok(_) = s.rx.recv_timeout(Duration::from_secs(5)) {}   // la tarjeta y los datos
        s.dictar(&ctx, sesion, audio);
        let Llega::Dictado(r) = espera(&s) else { panic!("esperaba el dictado") };
        let texto = r.expect("el servidor transcribe");
        println!("  dictado → {}", texto);
        assert!(!texto.is_empty());
    }

    #[test]
    fn el_nhc_se_escribe_como_se_quiera() {
        for ident in ["P0129798", "p0129798", "129798", "0129798", " 129798 ", "P129798"] {
            assert_eq!(nhc(ident), "P0129798", "{}", ident);
        }
        assert_eq!(nhc("PAC-00417"), "PAC-00417");
    }

    #[test]
    fn la_direccion_sale_de_medicci_conf() {
        let texto = "# comentario
# MEDICCI_SERVIDOR=http://no
MEDICCI_URL=http://modelo:11434/v1
                     MEDICCI_SERVIDOR = https://medicci.hospital.local  # del pliego
";
        assert_eq!(leer_clave(texto, "MEDICCI_SERVIDOR").as_deref(), Some("https://medicci.hospital.local"));
        assert_eq!(leer_clave(texto, "MEDICCI_CLAVE"), None);
    }
}
