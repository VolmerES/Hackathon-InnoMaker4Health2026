//! Acceso a la base de datos de la demo.
//!
//! Si el fichero no existe, se crea con el esquema y los datos que van
//! incrustados en el binario, así que el ejecutable funciona solo.

use rusqlite::{Connection, Row};
use std::path::{Path, PathBuf};

const SQL_CITA: &str = include_str!("../../bd/sql/01_cita.sql");
const SQL_MEDICCI: &str = include_str!("../../bd/sql/02_medicci.sql");
const SQL_VISTAS: &str = include_str!("../../bd/sql/03_vistas.sql");
const SQL_DATOS: &str = include_str!("../../bd/sql/04_datos.sql");

#[derive(Clone, Default)]
#[allow(dead_code)]
pub struct Paciente {
    pub id: String,
    pub edad: i64,
    pub sexo: String,
    pub centro: String,
    pub medico_ap: String,
    pub id_cita: Option<String>,
    pub motivo: String,
}

#[derive(Clone)]
pub struct Documento {
    pub id: String,
    pub fecha: String,
    pub tipo: String,
    pub servicio: String,
    pub texto: String,
}

#[derive(Clone)]
pub struct Hecho {
    pub id: String,
    pub categoria: String,
    pub afirmacion: String,
    pub cita: Option<String>,
    pub documento: Option<String>,
    pub fecha_doc: Option<String>,
    pub servicio: Option<String>,
}

#[derive(Clone)]
pub struct CampoFicha {
    pub bloque: String,
    pub seccion: Option<String>,
    pub etiqueta: String,
    pub valor: String,
    pub detalle: Option<String>,
    pub documento: Option<String>,
    pub fecha_doc: Option<String>,
    pub destacado: bool,
}

pub struct Datos {
    cx: Connection,
    #[allow(dead_code)]
    pub ruta: PathBuf,
}

impl Datos {
    /// Abre la base; la crea con los datos de la demo si no estaba.
    pub fn abrir(ruta: &Path) -> Result<Self, String> {
        let nueva = !ruta.exists();
        if let Some(dir) = ruta.parent() {
            let _ = std::fs::create_dir_all(dir);
        }
        let cx = Connection::open(ruta).map_err(|e| e.to_string())?;
        if nueva {
            for guion in [SQL_CITA, SQL_MEDICCI, SQL_VISTAS, SQL_DATOS] {
                cx.execute_batch(guion).map_err(|e| e.to_string())?;
            }
        }
        Ok(Self { cx, ruta: ruta.to_path_buf() })
    }

    #[allow(dead_code)]
    pub fn pacientes(&self) -> Vec<Paciente> {
        self.consulta(
            "SELECT p.id_paciente, p.edad, p.sexo, p.centro_salud, p.medico_ap,
                    c.id_cita, c.motivo_cita
               FROM pacientes p
          LEFT JOIN citas c ON c.id_paciente = p.id_paciente
           ORDER BY p.id_paciente",
            [],
            fila_paciente,
        )
    }

    /// Acepta el ID completo («PAC-00417»), sin guion o solo el número («417»).
    pub fn paciente(&self, ident: &str) -> Option<Paciente> {
        let limpio: String = ident.trim().to_uppercase().chars().filter(|c| !c.is_whitespace()).collect();
        let cifras = limpio.trim_start_matches("PAC").trim_start_matches('-');
        let ident = match cifras.parse::<u32>() {
            Ok(n) if cifras.chars().all(|c| c.is_ascii_digit()) => format!("PAC-{:05}", n),
            _ => limpio,
        };
        self.consulta(
            "SELECT p.id_paciente, p.edad, p.sexo, p.centro_salud, p.medico_ap,
                    c.id_cita, c.motivo_cita
               FROM pacientes p
          LEFT JOIN citas c ON c.id_paciente = p.id_paciente
              WHERE p.id_paciente = ?1 OR p.nhc = ?1
              LIMIT 1",
            [ident.as_str()],
            fila_paciente,
        )
        .into_iter()
        .next()
    }

    pub fn documentos(&self, id_paciente: &str) -> Vec<Documento> {
        self.consulta(
            "SELECT id_documento, fecha_documento, tipo_documento, servicio, texto
               FROM documentos_hce
              WHERE id_paciente = ?1
           ORDER BY fecha_documento DESC",
            [id_paciente],
            |f| {
                Ok(Documento {
                    id: f.get(0)?,
                    fecha: f.get(1)?,
                    tipo: f.get(2)?,
                    servicio: f.get::<_, Option<String>>(3)?.unwrap_or_default(),
                    texto: f.get(4)?,
                })
            },
        )
    }

    /// Hechos del paciente; si se pasan categorías, los que contengan alguna.
    pub fn hechos(&self, id_paciente: &str, categorias: &[&str]) -> Vec<Hecho> {
        let mut sql = String::from(
            "SELECT h.id_hecho, h.categoria, h.afirmacion, h.cita_literal, h.id_documento,
                    d.fecha_documento, d.servicio
               FROM hechos h
          LEFT JOIN documentos_hce d ON d.id_documento = h.id_documento
              WHERE h.id_paciente = ?1",
        );
        if !categorias.is_empty() {
            let trozos: Vec<String> = categorias
                .iter()
                .map(|c| format!("h.categoria LIKE '%{}%'", c.replace('\'', "")))
                .collect();
            sql.push_str(&format!(" AND ({})", trozos.join(" OR ")));
        }
        sql.push_str(" ORDER BY h.id_hecho");
        self.consulta(&sql, [id_paciente], |f| {
            Ok(Hecho {
                id: f.get(0)?,
                categoria: f.get(1)?,
                afirmacion: f.get(2)?,
                cita: f.get(3)?,
                documento: f.get(4)?,
                fecha_doc: f.get(5)?,
                servicio: f.get(6)?,
            })
        })
    }

    pub fn ficha(&self, id_paciente: &str) -> Vec<CampoFicha> {
        self.consulta(
            "SELECT fc.bloque, fc.etiqueta, fc.valor, fc.detalle, fc.id_documento,
                    d.fecha_documento, fc.seccion, fc.destacado
               FROM ficha_campos fc
               JOIN fichas_consulta f ON f.id_ficha = fc.id_ficha
               JOIN citas c           ON c.id_cita  = f.id_cita
          LEFT JOIN documentos_hce d  ON d.id_documento = fc.id_documento
              WHERE c.id_paciente = ?1
           ORDER BY fc.orden",
            [id_paciente],
            |f| {
                Ok(CampoFicha {
                    bloque: f.get(0)?,
                    etiqueta: f.get(1)?,
                    valor: f.get(2)?,
                    detalle: f.get(3)?,
                    documento: f.get(4)?,
                    fecha_doc: f.get(5)?,
                    seccion: f.get(6)?,
                    destacado: f.get::<_, String>(7)? == "S",
                })
            },
        )
    }

    /// Deja constancia de quién abre qué: lo exige el uso clínico.
    pub fn registra(&self, usuario: &str, id_paciente: &str, accion: &str, objeto: Option<&str>) {
        let id: String = (0..16)
            .map(|_| {
                let n = std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .map(|d| d.subsec_nanos())
                    .unwrap_or(0);
                char::from_digit(n % 16, 16).unwrap_or('0')
            })
            .collect();
        let _ = self.cx.execute(
            "INSERT INTO accesos (id_acceso, usuario, id_paciente, accion, objeto, fecha_hora, sistema)
             VALUES (?1, ?2, ?3, ?4, ?5, datetime('now'), 'medicci')",
            rusqlite::params![id, usuario, id_paciente, accion, objeto],
        );
    }

    fn consulta<T, P, F>(&self, sql: &str, parametros: P, convierte: F) -> Vec<T>
    where
        P: rusqlite::Params,
        F: Fn(&Row) -> rusqlite::Result<T>,
    {
        let mut sentencia = match self.cx.prepare(sql) {
            Ok(s) => s,
            Err(_) => return Vec::new(),
        };
        let filas = match sentencia.query_map(parametros, |f| convierte(f)) {
            Ok(f) => f,
            Err(_) => return Vec::new(),
        };
        filas.filter_map(|f| f.ok()).collect()
    }
}

fn fila_paciente(f: &Row) -> rusqlite::Result<Paciente> {
    Ok(Paciente {
        id: f.get(0)?,
        edad: f.get(1)?,
        sexo: f.get(2)?,
        centro: f.get::<_, Option<String>>(3)?.unwrap_or_default(),
        medico_ap: f.get::<_, Option<String>>(4)?.unwrap_or_default(),
        id_cita: f.get(5)?,
        motivo: f.get::<_, Option<String>>(6)?.unwrap_or_default(),
    })
}

#[cfg(test)]
mod pruebas {
    use super::*;

    /// Una base nueva en el directorio temporal, creada con el SQL incrustado.
    pub fn base_de_prueba(nombre: &str) -> Datos {
        let ruta = std::env::temp_dir().join(format!("medicci-prueba-{}-{}.db", nombre, std::process::id()));
        let _ = std::fs::remove_file(&ruta);
        Datos::abrir(&ruta).expect("la base se crea con el SQL incrustado")
    }

    #[test]
    fn se_crea_con_los_dos_pacientes() {
        let d = base_de_prueba("pacientes");
        let ids: Vec<String> = d.pacientes().into_iter().map(|p| p.id).collect();
        assert_eq!(ids, vec!["PAC-00417", "PAC-02856"]);
    }

    #[test]
    fn el_paciente_se_encuentra_por_numero_o_por_id() {
        let d = base_de_prueba("ident");
        for ident in ["PAC-00417", "pac-00417", "417", "00417", " 417 ", "PAC417"] {
            assert_eq!(d.paciente(ident).map(|p| p.id), Some("PAC-00417".into()), "{}", ident);
        }
        assert!(d.paciente("999").is_none());
        assert!(d.paciente("hola").is_none());
    }

    #[test]
    fn cada_cita_literal_esta_en_su_documento() {
        let d = base_de_prueba("citas");
        for p in d.pacientes() {
            let docs = d.documentos(&p.id);
            assert!(!docs.is_empty());
            for h in d.hechos(&p.id, &[]) {
                if let (Some(c), Some(id)) = (&h.cita, &h.documento) {
                    let doc = docs.iter().find(|x| &x.id == id).expect("el documento existe");
                    assert!(doc.texto.contains(c.as_str()), "{}: «{}» no está en {}", h.id, c, id);
                }
            }
        }
    }

    #[test]
    fn hay_ficha_para_cada_paciente() {
        let d = base_de_prueba("ficha");
        for p in d.pacientes() {
            let f = d.ficha(&p.id);
            assert!(f.iter().any(|c| c.bloque == "antes_de_empezar"), "{}", p.id);
            assert!(f.iter().any(|c| c.bloque == "constantes"), "{}", p.id);
        }
    }
}

#[cfg(test)]
pub use pruebas::base_de_prueba;
