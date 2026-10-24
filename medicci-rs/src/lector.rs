//! El lector: responde con lo que está escrito en la historia.
//!
//! Tres caminos, en este orden:
//!   1. un valor concreto («GOT de la última analítica»),
//!   2. los hechos guardados de esa categoría («¿alergias?»),
//!   3. frases literales que contengan lo preguntado.
//! Si no hay nada, lo dice. Nunca deduce.

use crate::datos::{Datos, Documento, Hecho};
use regex::RegexBuilder;

/// Parámetros que sabe buscar, con sus alias y su unidad.
pub const ANALITOS: &[(&str, &[&str], &str)] = &[
    ("GOT", &["got", "ast"], "U/L"),
    ("GPT", &["gpt", "alt"], "U/L"),
    ("GGT", &["ggt"], "U/L"),
    ("Bilirrubina", &["bilirrubina"], "mg/dL"),
    ("Amilasa", &["amilasa"], "U/L"),
    ("Lipasa", &["lipasa"], "U/L"),
    ("Hemoglobina", &["hb", "hemoglobina"], "g/dL"),
    ("Hematocrito", &["hto"], "%"),
    ("VCM", &["vcm"], "fL"),
    ("Plaquetas", &["plaquetas"], ""),
    ("Leucocitos", &["leucos", "leucocitos"], ""),
    ("Ferritina", &["ferritina"], "ng/mL"),
    ("Hierro", &["hierro"], "µg/dL"),
    ("IST", &["ist"], "%"),
    ("Creatinina", &["cr", "creatinina"], "mg/dL"),
    ("Filtrado glomerular", &["fg", "filtrado"], "mL/min"),
    ("Urea", &["urea"], "mg/dL"),
    ("Albúmina", &["albumina"], "g/dL"),
    ("Glucosa", &["glucosa"], "mg/dL"),
    ("HbA1c", &["hba1c"], "%"),
    ("Colesterol LDL", &["ldl"], "mg/dL"),
    ("PCR", &["pcr"], "mg/L"),
    ("TSH", &["tsh"], "µU/mL"),
    ("Vitamina D", &["vitamina d"], "ng/mL"),
    ("INR", &["inr"], ""),
    ("Calprotectina", &["calprotectina"], "µg/g"),
    ("Peso", &["peso"], "kg"),
    ("Talla", &["talla"], "cm"),
    ("IMC", &["imc"], ""),
    ("Tensión arterial", &["ta", "tension arterial"], "mmHg"),
    ("Frecuencia cardíaca", &["fc"], "lpm"),
];

/// De la palabra de la pregunta a la categoría de los hechos.
const LEXICO: &[(&str, &[&str])] = &[
    ("resumen", &["que le ha pasado", "resumen", "resume", "historia", "caso", "situacion"]),
    ("alarma", &["alarma", "bandera", "grave", "riesgo", "perdida de peso", "sangrado", "disfagia"]),
    ("contradiccion", &["contradic", "discrepan", "cuadra", "error", "no coincide", "fiable"]),
    ("alergias", &["alergia", "alergica", "namc", "metamizol", "nolotil", "yodo", "contraste"]),
    ("medicacion", &["medicacion", "medicamento", "tratamiento", "toma", "farmaco", "tto"]),
    ("pruebas", &["prueba", "pruebas", "analitica", "eco", "ecografia", "endoscopia",
                  "gastroscopia", "colonoscopia", "resultado"]),
    ("antecedentes", &["antecedente", "previo", "cronico", "ingreso", "urgencias"]),
    ("anticoagulacion", &["anticoagul", "acenocumarol", "sintrom", "inr", "adiro", "aas"]),
    ("renal", &["renal", "rinon", "creatinina", "filtrado", "erc"]),
    ("falta", &["falta", "pendiente", "no se ha hecho", "queda por"]),
    ("motivo", &["motivo", "por que viene", "derivacion", "cita de hoy"]),
    ("peso", &["peso", "kilos", "imc", "adelgaz"]),
    ("habitos", &["fuma", "tabaco", "alcohol", "bebe"]),
    ("familia", &["familiar", "familia", "herencia"]),
    ("cirugias", &["cirugia", "operad", "intervencion", "quirurg"]),
    ("salud_mental", &["salud mental", "ansiedad", "depresion", "psiq", "animo"]),
    ("urgencias", &["urgencias", "ingreso"]),
    ("endoscopia", &["endoscop", "gastroscop", "colonoscop"]),
    ("hierro", &["hierro", "ferrop", "anemia", "ferritina"]),
    ("imagen", &["imagen", "tc ", "rmn", "resonancia", "radiolog"]),
    ("social", &["social", "trabajo", "vive con", "cuidador", "cuida"]),
    ("hpylori", &["pylori", "helicobacter"]),
];

const NUMERO: &str = r"[<>]?\d+(?:[.,]\d+)?(?:\s*/\s*\d+)?\s*%?";

/// Minúsculas y sin tildes, para comparar como se escribe de verdad.
pub fn sin_tildes(texto: &str) -> String {
    texto
        .to_lowercase()
        .chars()
        .map(|c| match c {
            'á' | 'à' | 'ä' | 'â' => 'a',
            'é' | 'è' | 'ë' | 'ê' => 'e',
            'í' | 'ì' | 'ï' | 'î' => 'i',
            'ó' | 'ò' | 'ö' | 'ô' => 'o',
            'ú' | 'ù' | 'ü' | 'û' => 'u',
            'ñ' => 'n',
            'ç' => 'c',
            otro => otro,
        })
        .collect()
}

pub struct ValorHallado {
    pub nombre: String,
    pub valor: String,
    pub unidad: String,
    pub linea: String,
    pub documento: String,
    pub fecha: String,
    pub tipo: String,
    pub servicio: String,
}

pub enum Respuesta {
    Valores(Vec<ValorHallado>),
    Hechos(Vec<Hecho>),
    Literal(Vec<(Documento, String)>),
    NoConsta,
}

pub struct Lector {
    pub docs: Vec<Documento>,
    id_paciente: String,
}

impl Lector {
    pub fn nuevo(datos: &Datos, id_paciente: &str) -> Self {
        Self { docs: datos.documentos(id_paciente), id_paciente: id_paciente.to_string() }
    }

    pub fn responde(&self, datos: &Datos, pregunta: &str) -> Respuesta {
        let valores = self.valores(pregunta);
        if !valores.is_empty() {
            return Respuesta::Valores(valores);
        }
        let claves = self.claves(pregunta);
        if !claves.is_empty() {
            let refs: Vec<&str> = claves.iter().map(|c| c.as_str()).collect();
            let mut hechos = datos.hechos(&self.id_paciente, &refs);
            if !hechos.is_empty() {
                // lo que contradice la derivación, primero
                hechos.sort_by_key(|h| !h.categoria.contains("contradiccion"));
                return Respuesta::Hechos(hechos);
            }
        }
        let sueltas = self.frases(pregunta);
        if !sueltas.is_empty() {
            return Respuesta::Literal(sueltas);
        }
        Respuesta::NoConsta
    }

    /// Valores concretos: se busca el parámetro en las analíticas primero.
    fn valores(&self, pregunta: &str) -> Vec<ValorHallado> {
        let p = format!(" {} ", sin_tildes(pregunta).replace(['¿', '?', '¡', '!', ',', ';', ':'], " "));
        let mut salida = Vec::new();
        for (nombre, alias, unidad) in ANALITOS {
            if !alias.iter().any(|a| p.contains(&format!(" {} ", a))) {
                continue;
            }
            let patron = format!(r"\b({})\s*:?\s*({})", alias.join("|"), NUMERO);
            let re = match RegexBuilder::new(&patron).case_insensitive(true).build() {
                Ok(r) => r,
                Err(_) => continue,
            };
            let es_lab = |d: &&Documento| d.tipo == "Analítica" || d.tipo == "Resultado de pruebas";
            let labs: Vec<&Documento> = self.docs.iter().filter(es_lab).collect();
            let resto: Vec<&Documento> = self.docs.iter().filter(|d| !es_lab(d)).collect();
            'grupos: for grupo in [labs, resto] {
                for doc in grupo {
                    for linea in doc.texto.lines() {
                        if let Some(c) = re.captures(&sin_tildes(linea)) {
                            salida.push(ValorHallado {
                                nombre: nombre.to_string(),
                                valor: c[2].trim().replace('.', ","),
                                unidad: unidad.to_string(),
                                linea: linea.trim().to_string(),
                                documento: doc.id.clone(),
                                fecha: doc.fecha.clone(),
                                tipo: doc.tipo.clone(),
                                servicio: doc.servicio.clone(),
                            });
                            break 'grupos;
                        }
                    }
                }
            }
        }
        salida
    }

    /// El valor de un parámetro en un documento, con su línea literal.
    pub fn valor_en(&self, doc: &Documento, nombre: &str) -> Option<ValorHallado> {
        let (nombre, alias, unidad) = ANALITOS.iter().find(|(n, _, _)| *n == nombre)?;
        let patron = format!(r"\b({})\s*:?\s*({})", alias.join("|"), NUMERO);
        let re = RegexBuilder::new(&patron).case_insensitive(true).build().ok()?;
        doc.texto.lines().find_map(|linea| {
            re.captures(&sin_tildes(linea)).map(|c| ValorHallado {
                nombre: nombre.to_string(),
                valor: c[2].trim().replace('.', ","),
                unidad: unidad.to_string(),
                linea: linea.trim().to_string(),
                documento: doc.id.clone(),
                fecha: doc.fecha.clone(),
                tipo: doc.tipo.clone(),
                servicio: doc.servicio.clone(),
            })
        })
    }

    /// Todos los valores de un parámetro en la historia, del más antiguo al más reciente.
    pub fn serie(&self, nombre: &str) -> Vec<ValorHallado> {
        self.docs.iter().rev().filter_map(|d| self.valor_en(d, nombre)).collect()
    }

    /// Las analíticas, de la más reciente a la más antigua.
    pub fn analiticas(&self) -> Vec<&Documento> {
        self.docs.iter().filter(|d| d.tipo == "Analítica" || d.tipo == "Resultado de pruebas").collect()
    }

    fn claves(&self, pregunta: &str) -> Vec<String> {
        let t = sin_tildes(pregunta);
        LEXICO
            .iter()
            .filter(|(_, palabras)| palabras.iter().any(|p| t.contains(p)))
            .map(|(clave, _)| clave.to_string())
            .collect()
    }

    /// Última opción: frases del texto que contengan alguna palabra larga.
    fn frases(&self, pregunta: &str) -> Vec<(Documento, String)> {
        const VACIAS: &[&str] = &["que", "de", "la", "el", "los", "las", "un", "una", "se", "ha",
            "han", "tiene", "tienen", "por", "para", "con", "del", "al", "en", "su", "sus", "le",
            "hay", "como", "cuando", "cual", "donde", "esta", "este", "paciente", "dice", "consta"];
        let palabras: Vec<String> = sin_tildes(pregunta)
            .split(|c: char| !c.is_alphanumeric())
            .filter(|p| p.len() > 3 && !VACIAS.contains(p))
            .map(|p| p.to_string())
            .collect();
        if palabras.is_empty() {
            return Vec::new();
        }
        let mut salida = Vec::new();
        for doc in &self.docs {
            for frase in doc.texto.split(['\n', '.']) {
                let limpia = frase.trim();
                if limpia.len() > 14 && palabras.iter().any(|p| sin_tildes(limpia).contains(p)) {
                    salida.push((doc.clone(), limpia.to_string()));
                    if salida.len() >= 4 {
                        return salida;
                    }
                }
            }
        }
        salida
    }
}

#[cfg(test)]
mod pruebas {
    use super::*;
    use crate::datos::base_de_prueba;

    #[test]
    fn un_valor_sale_con_su_linea_literal() {
        let d = base_de_prueba("valor");
        let l = Lector::nuevo(&d, "PAC-00417");
        match l.responde(&d, "Hb de la última analítica") {
            Respuesta::Valores(v) => {
                assert_eq!(v[0].nombre, "Hemoglobina");
                let doc = l.docs.iter().find(|x| x.id == v[0].documento).unwrap();
                assert!(doc.texto.contains(&v[0].linea));
            }
            _ => panic!("esperaba un valor"),
        }
    }

    #[test]
    fn las_alergias_salen_y_la_contradiccion_va_primero() {
        let d = base_de_prueba("alergias");
        let l = Lector::nuevo(&d, "PAC-00417");
        match l.responde(&d, "¿alergias?") {
            Respuesta::Hechos(h) => assert!(h[0].categoria.contains("contradiccion")),
            _ => panic!("esperaba hechos"),
        }
    }

    #[test]
    fn la_serie_de_hemoglobina_va_en_orden_y_sale_de_sus_documentos() {
        let d = base_de_prueba("serie");
        let l = Lector::nuevo(&d, "PAC-02856");
        let hb = l.serie("Hemoglobina");
        assert!(hb.len() >= 2);
        assert!(hb.windows(2).all(|w| w[0].fecha <= w[1].fecha));
        assert_eq!(hb.last().map(|v| v.valor.as_str()), Some("9,6"));
        for v in &hb {
            let doc = l.docs.iter().find(|x| x.id == v.documento).unwrap();
            assert!(doc.texto.contains(&v.linea));
        }
    }

    #[test]
    fn lo_que_no_esta_no_se_inventa() {
        let d = base_de_prueba("noconsta");
        let l = Lector::nuevo(&d, "PAC-00417");
        assert!(matches!(l.responde(&d, "zxqw"), Respuesta::NoConsta));
    }
}
