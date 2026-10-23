//! Medicci conectada: la burbuja habla con el servidor de Medici, el mismo que
//! usa la pestaña web (mvp/servidor.py). Un cerebro y dos caras: la pestaña,
//! cuando HCIS va en el navegador, y la burbuja, cuando no.
//!
//! Las llamadas van en otro hilo para que la burbuja no se congele mientras
//! contesta el modelo; lo que llega se recoge en cada fotograma.

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

pub enum Llega {
    Sesion { sesion: String, nhc: String, edad: i64, filas: i64 },
    Tarjeta { sesion: String, alertas: Vec<Alerta>, mas: i64 },
    Respuesta { texto: String, consta: bool, fuentes: Vec<Fuente>, segundos: f64 },
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
            Err(v["error"].as_str().unwrap_or("El servidor de Medici ha dado un error.").to_string())
        }
        Err(e) => Err(format!("No llego al servidor de Medici ({}).", e)),
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

/// Los NHC de los CSV son «P» y siete cifras: «129798» vale por «P0129798».
pub fn nhc(ident: &str) -> String {
    let limpio: String = ident.trim().to_uppercase().chars().filter(|c| !c.is_whitespace()).collect();
    let cifras = limpio.trim_start_matches('P');
    match cifras.parse::<u32>() {
        Ok(n) if !cifras.is_empty() && cifras.chars().all(|c| c.is_ascii_digit()) => format!("P{:07}", n),
        _ => limpio,
    }
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

    /// Abre una sesión atada al paciente y trae su tarjeta.
    pub fn abrir(&self, ctx: &egui::Context, nhc: String) {
        self.en_hilo(ctx, move |base| {
            let s = match leer(agente(15).post(&format!("{base}/api/sesion")).send_json(json!({ "nhc": nhc }))) {
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
            ]
        });
    }

    pub fn tarjeta(&self, ctx: &egui::Context, sesion: String) {
        self.en_hilo(ctx, move |base| vec![tarjeta(base, &sesion)]);
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

    /// Contra un servidor de verdad: `MEDICI_SERVIDOR=http://localhost:8000 cargo test -- --ignored`
    #[test]
    #[ignore]
    fn contra_el_servidor() {
        let url = std::env::var("MEDICI_SERVIDOR").unwrap_or("http://localhost:8000".into());
        let s = Servidor::nuevo(&url);
        assert!(s.responde(), "no hay servidor en {}", url);
        let ctx = egui::Context::default();
        s.abrir(&ctx, nhc("129798"));
        let Llega::Sesion { sesion, nhc, edad, .. } = espera(&s) else { panic!("esperaba la sesión") };
        println!("sesión {} · {} · {} años", sesion, nhc, edad);
        let Llega::Tarjeta { alertas, .. } = espera(&s) else { panic!("esperaba la tarjeta") };
        for a in &alertas {
            println!("  • {}  [{}]", a.texto, a.fuentes.iter().map(|f| f.referencia.as_str()).collect::<Vec<_>>().join(", "));
        }
        assert!(!alertas.is_empty());
        s.preguntar(&ctx, sesion, "¿Tiene alergias?".into());
        let Llega::Respuesta { texto, consta, .. } = espera(&s) else { panic!("esperaba la respuesta") };
        println!("  ¿Tiene alergias? → {}", texto);
        assert!(!consta);
    }

    #[test]
    fn el_nhc_se_escribe_como_se_quiera() {
        for ident in ["P0129798", "p0129798", "129798", "0129798", " 129798 ", "P129798"] {
            assert_eq!(nhc(ident), "P0129798", "{}", ident);
        }
        assert_eq!(nhc("PAC-00417"), "PAC-00417");
    }
}
