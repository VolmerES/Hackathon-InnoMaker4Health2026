// Genera bd/sql/04_datos.sql a partir de los ficheros de datos de la demo.
// Uso: node bd/carga/generar_datos.js
const path = require("path");
const fs = require("fs");
const W = path.join(__dirname, "..", "..", "demo", "web");
global.window = {};
require(path.join(W, "historial-datos.js"));
require(path.join(W, "historial-datos-2.js"));
require(path.join(W, "ficha-medici.js"));
const w = global.window;

const q = v => v === null || v === undefined ? "NULL" : "'" + String(v).replace(/'/g, "''") + "'";
const n = v => v === null || v === undefined ? "NULL" : String(v);
const fechaISO = f => { const [d, m, a] = f.split("/"); return `${a}-${m}-${d}`; };
const out = [];
const añade = (tabla, cols, filas) => filas.forEach(f =>
  out.push(`INSERT INTO ${tabla} (${cols.join(", ")}) VALUES (${f.join(", ")});`));

const PACS = [
  {id:"P1", p:w.PACIENTE, docs:w.DOCS, hechos:w.HECHOS, ficha:w.FICHAS.P1,
   episodio:"CEX-2026-118442", cita:"CITA-2026-00417", deriv:"DER-2026-04871", lectura:"LEC-P1-001", fichaId:"FICHA-P1-001"},
  {id:"P2", p:w.P2.paciente, docs:w.P2.docs, hechos:w.P2.hechos, ficha:w.FICHAS.P2,
   episodio:"CEX-2026-118517", cita:"CITA-2026-02856", deriv:"DER-2026-04902", lectura:"LEC-P2-001", fichaId:"FICHA-P2-001"}
];

out.push("-- Generado por bd/carga/generar_datos.js · datos sintéticos de la demo");
out.push("BEGIN TRANSACTION;");

for (const X of PACS){
  const p = X.p;
  añade("pacientes",
    ["id_paciente","nhc","cipa","fecha_nacimiento","edad","sexo","preferencias_contacto",
     "necesidades_accesibilidad","telefono","centro_salud","medico_ap"],
    [[q(p.nhc), q(p.nhc), q(p.cipa), q(fechaISO(p.nacimiento)), n(p.edad), q(p.sexo),
      q(p.contacto), q(p.acc), q(p.telefono), q(p.centro), q(p.medico_ap)]]);

  const deriv = X.docs.find(d => d.tipo === "Derivación a consulta externa");
  añade("derivaciones_ap",
    ["id_derivacion","id_paciente","fecha_solicitud","especialidad_destino","prioridad",
     "motivo_derivacion","observaciones_medico","centro_origen","medico_solicitante","estado_solicitud"],
    [[q(X.deriv), q(p.nhc), q(fechaISO(deriv.fecha) + " 09:00:00"), q("APARATO DIGESTIVO"), q("Normal"),
      q(p.cita.motivo), q(deriv.texto), q(p.centro), q(deriv.autor), q("Validada")]]);

  añade("citas",
    ["id_cita","id_paciente","id_derivacion","fecha_asignacion","tipo_visita","estado_actual",
     "prioridad","servicio","episodio","motivo_cita"],
    [[q(X.cita), q(p.nhc), q(X.deriv), q("2026-10-23"), q("Primera"), q("Planificada"),
      q("Normal"), q("Aparato Digestivo"), q(X.episodio), q(p.cita.motivo)]]);

  añade("documentos_hce",
    ["id_documento","id_paciente","fecha_documento","tipo_documento","servicio","autor","origen","texto","palabras"],
    X.docs.map(d => [q(d.id), q(p.nhc), q(fechaISO(d.fecha)), q(d.tipo), q(d.servicio),
      q(d.autor), q("HCIS"), q(d.texto), n(d.texto.split(/\s+/).length)]));

  añade("lecturas",
    ["id_lectura","id_paciente","id_cita","fecha_lectura","modelo","version_modelo",
     "temperatura","documentos_leidos","milisegundos","ejecutada_en"],
    [[q(X.lectura), q(p.nhc), q(X.cita), q("2026-10-23 03:10:00"),
      q("lector-extractivo"), q("demo-1"), "0", n(X.docs.length), "900", q("servidor del hospital")]]);

  let i = 0;
  añade("hechos",
    ["id_hecho","id_lectura","id_paciente","id_documento","categoria","afirmacion","cita_literal",
     "pos_inicio","pos_fin","confianza","consta"],
    X.hechos.map(h => {
      const doc = h.doc ? X.docs.find(d => d.id === h.doc) : null;
      const pos = doc && h.cita ? doc.texto.indexOf(h.cita) : -1;
      return [q(`${X.id}-H${String(++i).padStart(3,"0")}`), q(X.lectura), q(p.nhc),
        h.doc ? q(h.doc) : "NULL", q(h.k), q(h.t), h.cita ? q(h.cita) : "NULL",
        pos >= 0 ? n(pos) : "NULL", pos >= 0 ? n(pos + h.cita.length) : "NULL",
        "0.95", q(h.doc ? "S" : "N")];
    }));

  // la ficha de primera consulta, campo a campo
  const f = X.ficha, campos = [];
  const add = (bloque, seccion, etiqueta, valor, detalle, doc, destacado) =>
    campos.push({bloque, seccion, etiqueta, valor, detalle, doc, destacado: destacado ? "S" : "N"});

  f.chips.forEach(c => add("cabecera", null, "Chip", c.t, null, null, c.destacado));
  f.antes.forEach(t => add("antes_de_empezar", null, t.t, t.v, t.d, t.doc, t.rojo));
  add("por_que_viene", "motivo", "Motivo de derivación", f.motivo.t, f.motivo.espera, f.motivo.doc, true);
  f.cambios.filas.forEach(c => add("por_que_viene", "cambios", c.t, c.v, c.d, c.doc));
  f.problemas.filas.forEach(x => add("por_que_viene", "problemas", x.t, x.v, null, x.doc));
  f.medicacion.filas.forEach(m => add("por_que_viene", "medicacion", m.f,
    [m.dosis, m.pauta].filter(Boolean).join(" · "), m.aviso, m.doc, !!m.aviso));
  add("por_que_viene", "medicacion", "Adherencia", f.medicacion.adherencia, null, null);
  f.constantes.forEach(c => add("constantes", null, c.t, c.v, c.d, c.doc));
  f.vida.forEach(v => add("vida", null, v.t, v.d, null, v.doc, v.marco));
  f.temas.forEach(t => add("temas", null, "Tema", t, null, null));

  añade("fichas_consulta",
    ["id_ficha","id_cita","id_lectura","generada_en","formato","campos_con_dato","campos_sin_dato","vista_por","vista_en"],
    [[q(X.fichaId), q(X.cita), q(X.lectura), q("2026-10-23 03:10:02"), q("medici-v1"),
      n(campos.filter(c => c.doc).length), n(campos.filter(c => !c.doc).length), "NULL", "NULL"]]);

  let j = 0;
  añade("ficha_campos",
    ["id_campo","id_ficha","bloque","seccion","etiqueta","valor","detalle","id_documento","orden","destacado"],
    campos.map(c => [q(`${X.id}-C${String(++j).padStart(3,"0")}`), q(X.fichaId), q(c.bloque),
      c.seccion ? q(c.seccion) : "NULL", q(c.etiqueta), q(c.valor), c.detalle ? q(c.detalle) : "NULL",
      c.doc ? q(c.doc) : "NULL", n(j), q(c.destacado)]));
}

out.push("COMMIT;");
fs.writeFileSync(path.join(__dirname, "..", "sql", "04_datos.sql"), out.join("\n") + "\n");
console.log("04_datos.sql ·", out.length, "sentencias");
