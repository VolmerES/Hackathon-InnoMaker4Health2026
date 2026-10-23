// Agente flotante de Jano: lector de la historia, resumen y ficha de contexto.
// Lo usan las dos pantallas (SELENE y HCIS). Cada pantalla aporta su contexto:
//   window.HCE = { paciente(), docs(), porId(), iraDoc(id, cita) }
(function(){
const MARKUP = `<!-- agente flotante -->
<button id="lanzador" aria-label="Abrir el lector de historia" title="Lector de historia">
  <span class="punto"></span>
  <svg width="34" height="34" viewBox="0 0 34 34" aria-hidden="true">
    <circle cx="17" cy="10.5" r="5.2" fill="#fff"/>
    <path d="M5.5 29c0-6 5.2-9.5 11.5-9.5S28.5 23 28.5 29z" fill="#fff"/>
    <path d="M12.6 20.2v4.1a4.4 4.4 0 0 0 8.8 0v-4.1" fill="none" stroke="#047a55" stroke-width="1.5"/>
    <circle cx="21.4" cy="25.6" r="2" fill="#047a55"/>
    <path d="M17 6.2v2.6M15.7 7.5h2.6" stroke="#047a55" stroke-width="1.4"/>
  </svg>
</button>
<button id="resumen-btn" title="Resumen de la historia">
  <svg width="20" height="20" viewBox="0 0 34 34" aria-hidden="true">
    <circle cx="17" cy="10.5" r="5.2" fill="#047a55"/>
    <path d="M5.5 29c0-6 5.2-9.5 11.5-9.5S28.5 23 28.5 29z" fill="#047a55"/>
  </svg>
  Generar resumen
</button>

<div id="velo" hidden>
  <section id="modal" role="dialog" aria-modal="true" aria-labelledby="modal-t">
    <div class="cab">
      <span class="av"><svg width="22" height="22" viewBox="0 0 34 34" aria-hidden="true">
        <circle cx="17" cy="10.5" r="5.2" fill="#047a55"/>
        <path d="M5.5 29c0-6 5.2-9.5 11.5-9.5S28.5 23 28.5 29z" fill="#047a55"/></svg></span>
      <span><b id="modal-t"></b><small id="modal-sub"></small></span>
      <button class="x" id="modal-cerrar" aria-label="Cerrar">×</button>
    </div>
    <div class="pest">
      <button data-m="resumen" aria-selected="true">Resumen de entrada</button>
      <button data-m="ficha" aria-selected="false">Ficha de contexto · 3 capas</button>
    </div>
    <div class="cuerpo" id="modal-cuerpo"></div>
    <div class="pie">
      <span class="nota">Pulse cualquier referencia para abrir el documento en la historia.</span>
      <button id="modal-ok">Cerrar</button>
    </div>
  </section>
</div>

<section id="chat" hidden aria-label="Lector de historia">
  <div class="cab">
    <span class="av">
      <svg width="22" height="22" viewBox="0 0 34 34" aria-hidden="true">
        <circle cx="17" cy="10.5" r="5.2" fill="#047a55"/>
        <path d="M5.5 29c0-6 5.2-9.5 11.5-9.5S28.5 23 28.5 29z" fill="#047a55"/>
      </svg>
    </span>
    <span><b>Jano · lector de historia</b><small id="chat-sub"></small></span>
    <button class="x" id="chat-cerrar" aria-label="Cerrar">×</button>
  </div>
  <div class="cuerpo" id="chat-cuerpo" aria-live="polite"></div>
  <form id="chat-form" autocomplete="off">
    <input id="chat-q" placeholder="Pregunte sobre este paciente" aria-label="Pregunta">
    <button type="submit">Enviar</button>
  </form>
</section>`;

const esc = s => String(s).replace(/[&<>"]/g, c => ({"&":"&amp;","<":"&lt;",">":"&gt;",'"':"&quot;"}[c]));
const sinT = s => s.toLowerCase().normalize("NFD").replace(/[\u0300-\u036f]/g,"");
const $ = s => document.querySelector(s);
const P = () => window.HCE.paciente();
const docsDe = () => window.HCE.docs();
const porIdDe = () => window.HCE.porId();
const iraDoc = (id, cita) => window.HCE.iraDoc(id, cita);

/* ================= agente flotante ================= */
const CLAVES_RESUMEN = ["motivo","resumen","contradiccion","falta"];
const chatCuerpo = () => $("#chat-cuerpo");

function clavesDe(preg){
  const t = sinT(preg), ks = new Set();
  window.LEXICO.forEach(l => { if (l.v.some(v => t.includes(sinT(v)))) ks.add(l.k); });
  return [...ks];
}
const hechosDe = claves => P().hechos.filter(h => h.k.split("|").some(k => claves.includes(k)));
function frasesLiterales(preg){
  const stop = new Set(["que","de","la","el","los","las","un","una","se","ha","han","tiene","tienen","por","para","con","del","al","en","y","o","es","su","sus","le","hay","como","cuando","cuanto","cual","donde","algo","sobre","este","esta","paciente","dice","consta"]);
  const pal = sinT(preg).replace(/[¿?¡!.,;:]/g,"").split(/\s+/).filter(w => w.length > 3 && !stop.has(w));
  if (!pal.length) return [];
  const out = [];
  docsDe().forEach(d => d.texto.split(/\n(?=[A-ZÁÉÍÓÚ])|(?<=\.)\s+/).forEach(fr => {
    if (fr.trim().length > 12 && pal.some(w => sinT(fr).includes(w))) out.push({doc:d.id, cita:fr.trim()});
  }));
  return out.slice(0,4);
}
function respuesta(preg, forzadas){
  let claves = forzadas || clavesDe(preg);
  const concretas = ["eco","analitica","hpylori","soh","endoscopia","celiaquia","imagen"];
  if (claves.includes("pruebas"))
    claves = claves.some(k => concretas.includes(k)) ? claves.filter(k => k !== "pruebas") : claves.concat(concretas);
  const hs = hechosDe(claves);
  if (hs.length) return {tipo:"hechos", hs};
  const fr = frasesLiterales(preg);
  return fr.length ? {tipo:"literal", fr} : {tipo:"nada"};
}
function filaHecho(h, literal){
  const d = h.doc ? porIdDe()[h.doc] : null;
  return `<div class="linea">
    <div class="afirm${!literal && h.k && h.k.includes("contradiccion") ? " aviso" : ""}">${esc(literal ? "En la historia está escrito:" : h.t)}</div>
    ${h.cita ? `<blockquote>${esc(h.cita)}</blockquote>` : ""}
    ${d ? `<button class="fuente" data-doc="${d.id}" data-cita="${esc(h.cita || "")}">${d.id} · ${d.fecha} · ${esc(d.servicio)}</button>`
        : `<span class="fuente" style="cursor:default">comprobado: no aparece en los ${docsDe().length} documentos</span>`}</div>`;
}
function enlaza(cont){
  cont.querySelectorAll("button.fuente,button.src").forEach(b => b.onclick = () => iraDoc(b.dataset.doc, b.dataset.cita || ""));
}
function dice(html){
  const c = chatCuerpo();
  const vacio = c.querySelector(":scope > p");
  if (vacio && !c.querySelector(".ag,.yo")) vacio.remove();
  c.insertAdjacentHTML("beforeend", html);
  c.scrollTop = c.scrollHeight;
  enlaza(c);
}
function pregunta(texto, claves){
  dice(`<div class="yo">${esc(texto)}</div>`);
  if (!claves){
    const directa = respuestaDirecta(texto);
    if (directa) return dice(`<div class="ag">${directa}</div>`);
  }
  const r = respuesta(texto, claves);
  if (r.tipo === "nada")
    dice(`<div class="ag"><h4>Respuesta</h4><p class="nada">No consta en la historia. Ninguna frase de los ${docsDe().length} documentos responde a eso y no lo voy a deducir.</p></div>`);
  else
    dice(`<div class="ag">${r.tipo === "literal" ? '<h4>Lo que dice la historia</h4>' : ""}
      ${(r.tipo === "hechos" ? r.hs : r.fr).map(h => filaHecho(h, r.tipo === "literal")).join("")}</div>`);
}


/* ---------- respuesta directa: valores concretos ---------- */
const ANALITOS = [
 {n:"GOT", a:["got","ast"], u:"U/L"},
 {n:"GPT", a:["gpt","alt"], u:"U/L"},
 {n:"GGT", a:["ggt"], u:"U/L"},
 {n:"Fosfatasa alcalina", a:["fa"], u:"U/L"},
 {n:"Bilirrubina total", a:["bilirrubina total","bilirrubina"], u:"mg/dL"},
 {n:"LDH", a:["ldh"], u:"U/L"},
 {n:"Amilasa", a:["amilasa"], u:"U/L"},
 {n:"Lipasa", a:["lipasa"], u:"U/L"},
 {n:"Hemoglobina", a:["hb","hemoglobina"], u:"g/dL"},
 {n:"Hematocrito", a:["hto","hematocrito"], u:"%"},
 {n:"VCM", a:["vcm"], u:"fL"},
 {n:"Leucocitos", a:["leucos","leucocitos"], u:""},
 {n:"Plaquetas", a:["plaquetas"], u:""},
 {n:"Ferritina", a:["ferritina"], u:"ng/mL"},
 {n:"Hierro", a:["hierro"], u:"µg/dL"},
 {n:"IST", a:["ist"], u:"%"},
 {n:"Creatinina", a:["cr","creatinina"], u:"mg/dL"},
 {n:"Filtrado glomerular", a:["fg","filtrado glomerular","filtrado"], u:"ml/min"},
 {n:"Urea", a:["urea"], u:"mg/dL"},
 {n:"Sodio", a:["na"], u:"mmol/L"},
 {n:"Potasio", a:["k"], u:"mmol/L"},
 {n:"Albúmina", a:["albumina"], u:"g/dL"},
 {n:"Proteínas totales", a:["proteinas totales"], u:"g/dL"},
 {n:"Glucosa", a:["glucosa"], u:"mg/dL"},
 {n:"HbA1c", a:["hba1c"], u:"%"},
 {n:"Colesterol total", a:["col total","colesterol"], u:"mg/dL"},
 {n:"LDL", a:["ldl"], u:"mg/dL"},
 {n:"HDL", a:["hdl"], u:"mg/dL"},
 {n:"Triglicéridos", a:["tg","trigliceridos"], u:"mg/dL"},
 {n:"PCR", a:["pcr"], u:"mg/L"},
 {n:"TSH", a:["tsh"], u:"µU/mL"},
 {n:"Vitamina B12", a:["b12","vitamina b12"], u:"pg/mL"},
 {n:"Folato", a:["folato"], u:"ng/mL"},
 {n:"Vitamina D", a:["vitamina d"], u:"ng/mL"},
 {n:"Calcio", a:["calcio"], u:"mg/dL"},
 {n:"INR", a:["inr"], u:""},
 {n:"Calprotectina fecal", a:["calprotectina"], u:"µg/g"},
 {n:"Reticulocitos", a:["reticulocitos"], u:"%"},
 {n:"Peso", a:["peso"], u:"kg"},
 {n:"Talla", a:["talla"], u:"cm"},
 {n:"IMC", a:["imc"], u:""},
 {n:"Tensión arterial", a:["ta","tension arterial","presion arterial"], u:"mmHg"},
 {n:"Frecuencia cardíaca", a:["fc","frecuencia cardiaca"], u:"lpm"}
];
const NUM = "[<>]?\\d+(?:[.,]\\d+)?(?:\\.\\d{3})?(?:\\s*/\\s*\\d+)?\\s*%?";

const esLab = d => d.tipo === "Analítica" || d.tipo === "Resultado de pruebas";
function buscaEn(lista, re){
  for (const d of lista){
    for (const linea of d.texto.split("\n")){
      const m = sinT(linea).match(re);
      if (m) return {doc:d.id, fecha:d.fecha, tipo:d.tipo, valor:m[2].trim().replace(".",","), linea:linea.trim().replace(/\s+/g," ")};
    }
  }
  return null;
}
function valorEnDocs(an, pidioUltima){
  const re = new RegExp("\\b(" + an.a.map(x => x.replace(/[.*+?^${}()|[\]\\]/g,"\\$&")).join("|") + ")\\s*:?\\s*(" + NUM + ")", "i");
  const labs = docsDe().filter(esLab);                 // docsDe() va del más reciente al más antiguo
  const v = buscaEn(labs, re) || buscaEn(docsDe().filter(d => !esLab(d)), re);
  if (!v) return null;
  // si lo pedido es «la última» y el valor no está en la analítica más reciente, se avisa
  const ultimaAnalitica = docsDe().find(d => d.tipo === "Analítica");
  v.avisoUltima = (pidioUltima && ultimaAnalitica && v.doc !== ultimaAnalitica.id)
    ? `La analítica del ${ultimaAnalitica.fecha} no incluye ${an.n}: este es el último valor registrado.` : "";
  return v;
}
function analitosPreguntados(preg){
  const t = " " + sinT(preg).replace(/[¿?¡!.,;:]/g," ") + " ";
  return ANALITOS.filter(an => an.a.some(x => t.includes(" " + sinT(x) + " ")));
}
function respuestaDirecta(preg){
  const ans = analitosPreguntados(preg);
  if (!ans.length) return null;
  const pidioUltima = /ultim|reciente|actual|hoy/.test(sinT(preg));
  const filas = ans.map(an => ({an, v:valorEnDocs(an, pidioUltima)})).filter(x => x.v);
  if (!filas.length) return null;
  return filas.map(({an,v}) => {
    const d = porIdDe()[v.doc];
    return `<div class="linea">
      <div class="directo"><b>${esc(an.n)} ${esc(v.valor)}</b>${an.u ? ` <span class="u">${esc(an.u)}</span>` : ""}
        <span class="cuando">· ${esc(v.tipo.toLowerCase())} del ${v.fecha}</span></div>
      ${v.avisoUltima ? `<div class="aviso-ult">${esc(v.avisoUltima)}</div>` : ""}
      <blockquote>${esc(v.linea)}</blockquote>
      <button class="fuente" data-doc="${v.doc}" data-cita="${esc(v.linea.slice(0,60))}">${v.doc} · ${d.fecha} · ${esc(d.servicio)}</button>
    </div>`;
  }).join("");
}

/* ---------- ventana flotante: resumen y ficha ---------- */
let modoModal = "resumen";

function htmlResumen(){
  const hs = [];
  CLAVES_RESUMEN.forEach(k => hechosDe([k]).forEach(h => { if (!hs.includes(h)) hs.push(h); }));
  return `<p style="margin:0 0 10px;color:#5b6b76">Leídos los ${docsDe().length} documentos de la historia en 0,9 s. Cada afirmación lleva la frase literal de la que sale.</p>
    ${hs.map(h => filaHecho(h,false)).join("")}`;
}
function htmlFicha(){
  const f = P().ficha, pid = porIdDe();
  const fu = (txt, doc) => doc && pid[doc]
    ? `<div class="fuente-m"><sup>1</sup> <button class="src" data-doc="${doc}">${esc(txt || (doc + " · " + pid[doc].fecha))}</button></div>`
    : (txt ? `<div class="fuente-m"><sup>1</sup> ${esc(txt)}</div>` : "");
  const sinDato = v => /^no consta|^sin registro/i.test(v) ? " sin" : "";

  return `<div class="med">
    <div class="med-eyebrow">${esc(f.eyebrow)}</div>
    <div class="med-cab">
      <h2>${esc(f.nombre)}</h2>
      <div class="med-chips">${f.chips.map(c => `<span class="chip${c.destacado ? " dest" : ""}">${esc(c.t)}</span>`).join("")}</div>
    </div>

    <h3 class="med-h">Antes de empezar</h3>
    <div class="med-grid4">${f.antes.map(t => `<div class="tarjeta${t.rojo ? " roja" : ""}">
        <div class="rot${t.rojo ? " rojo" : ""}">${esc(t.t)}</div>
        <div class="val${sinDato(t.v)}">${esc(t.v)}</div>
        <p>${esc(t.d)}</p>${fu(t.f, t.doc)}</div>`).join("")}</div>

    <h3 class="med-h">Por qué viene hoy</h3>
    <div class="med-grid2">
      <div class="tarjeta motivo">
        <div class="rot">Motivo de derivación</div>
        <div class="val">${esc(f.motivo.t)}</div>
        <p><b>Lo que espera:</b> ${esc(f.motivo.espera)}</p>${fu(f.motivo.f, f.motivo.doc)}</div>
      <div class="tarjeta">
        <div class="rot">${esc(f.cambios.titulo)}</div>
        ${f.cambios.filas.map(c => `<div class="cambio">
            <span class="spark ${c.tend}"></span>
            <span><b>${esc(c.t)} ${esc(c.v)}</b> <span class="gris">· ${esc(c.d)}</span></span>
            ${c.doc ? `<button class="src" data-doc="${c.doc}">${c.doc}</button>` : ""}</div>`).join("")}
        ${fu(f.cambios.f)}</div>
    </div>

    <div class="med-grid2">
      <div class="tarjeta"><h4>Problemas activos</h4>
        ${f.problemas.filas.map(x => `<div class="fila-m"><span>${esc(x.t)}</span>
          <span class="gris">${esc(x.v)}</span>
          ${x.doc ? `<button class="src" data-doc="${x.doc}">${x.doc}</button>` : ""}</div>`).join("")}
        ${fu(f.problemas.f)}</div>
      <div class="tarjeta"><h4>Medicación</h4>
        ${f.medicacion.filas.map(m => `<div class="fila-m"><span><b>${esc(m.f)}</b> ${esc(m.dosis || "")}</span>
          <span class="gris">${m.aviso ? `<span class="aviso-m">${esc(m.aviso)}</span>` : esc(m.pauta || "")}</span>
          ${m.doc ? `<button class="src" data-doc="${m.doc}">${m.doc}</button>` : ""}</div>`).join("")}
        <p class="adh">${esc(f.medicacion.adherencia)}</p>${fu(f.medicacion.f)}</div>
    </div>

    <h3 class="med-h">Constantes</h3>
    <div class="med-grid5">${f.constantes.map(c => `<div class="tarjeta ctes">
        <div class="rot">${esc(c.t)}</div>
        <div class="val${sinDato(c.v)}">${esc(c.v)}</div>
        <div class="pie-m">${esc(c.d)}${c.doc ? ` <button class="src" data-doc="${c.doc}">${c.doc}</button>` : ""}</div></div>`).join("")}</div>

    <h3 class="med-h">Su vida, en una línea</h3>
    <div class="med-grid3">${f.vida.map(v => `<div class="tarjeta${v.marco ? " marco" : ""}">
        <div class="rot">${esc(v.t)}</div><p>${esc(v.d)}</p>
        ${v.doc ? `<button class="src" data-doc="${v.doc}">${v.doc} · ${pid[v.doc] ? pid[v.doc].fecha : ""}</button>` : ""}</div>`).join("")}</div>

    <div class="med-grid2">
      <div class="tarjeta oscura">
        <h4>Pregunta, no busques</h4>
        <p>El resto del historial no se muestra: se pregunta. El lector responde con la fuente o dice «No consta».</p>
        <div class="burbuja-q">${esc(f.ejemplo.q)}</div>
        <div class="burbuja-a">${esc(f.ejemplo.a)}<span class="donde">${esc(f.ejemplo.donde)}</span></div>
        <button class="btn-med" id="ficha-preguntar">Preguntar otra cosa</button>
      </div>
      <div>
        <div class="tarjeta">
          <div class="rot">Memoria profunda · a demanda</div>
          <p>Toque un tema para preguntar por él.</p>
          <div class="temas">${f.temas.map(t => `<button class="chip tema">${esc(t)}</button>`).join("")}</div>
        </div>
        <div class="tarjeta como">
          <div class="rot">Cómo leer esta ficha</div>
          <p><sup>1</sup> Cada dato lleva su fuente: púlsela y se abre el documento.</p>
          <p><span class="nc">No consta</span> = no está en los datos. No se rellena.</p>
          <p>El lector solo lee: no diagnostica ni recomienda.</p>
        </div>
      </div>
    </div>
    <div class="med-pie">Maqueta · paciente y datos sintéticos, inventados para la demo</div>
  </div>`;
}

function pintaModal(){
  $("#modal-t").textContent = modoModal === "resumen" ? "Resumen de la historia" : "Ficha de contexto en 3 capas";
  $("#modal-sub").textContent = `${P().nombre} · ${P().nhc} · ${docsDe().length} documentos`;
  document.querySelectorAll("#modal .pest button").forEach(b => b.setAttribute("aria-selected", String(b.dataset.m === modoModal)));
  const c = $("#modal-cuerpo");
  c.innerHTML = modoModal === "resumen" ? htmlResumen() : htmlFicha();
  c.scrollTop = 0;
  enlaza(c);
  c.querySelectorAll(".tema").forEach(b => b.onclick = () => { cierraModal(); abreChat(); pregunta(b.textContent.trim()); });
  const pb = c.querySelector("#ficha-preguntar");
  if (pb) pb.onclick = () => { cierraModal(); abreChat(); };
}
function abreModal(modo){
  modoModal = modo || modoModal;
  $("#velo").hidden = false;
  pintaModal();
  $("#modal-cerrar").focus();
}
function cierraModal(){ $("#velo").hidden = true; }

function enlazaBotones(){
  enlazaChat();
$("#resumen-btn").onclick = () => abreModal("resumen");
$("#modal-cerrar").onclick = cierraModal;
$("#modal-ok").onclick = cierraModal;
$("#velo").onclick = e => { if (e.target === $("#velo")) cierraModal(); };
document.querySelectorAll("#modal .pest button").forEach(b => b.onclick = () => { modoModal = b.dataset.m; pintaModal(); });
document.addEventListener("keydown", e => { if (e.key === "Escape" && !$("#velo").hidden) cierraModal(); });
}

/* ---------- chat: vacío, sin sugerencias, espera pregunta ---------- */
function abreChat(){
  $("#chat").hidden = false;
  $("#chat-sub").textContent = `${P().nombre.split(" ")[0]} · ${docsDe().length} documentos leídos`;
  if (chatCuerpo().dataset.pac !== P().id){
    chatCuerpo().dataset.pac = P().id;
    chatCuerpo().innerHTML = `<p style="margin:auto 0;color:#8a98a3;font-size:12px;text-align:center">
      Pregunte lo que necesite de esta historia.<br>Responde solo con lo que está escrito, con la frase y el documento.</p>`;
  }
  $("#chat-q").focus();
}
function enlazaChat(){
$("#lanzador").onclick = () => { $("#chat").hidden ? abreChat() : ($("#chat").hidden = true); };
$("#chat-cerrar").onclick = () => { $("#chat").hidden = true; };
$("#chat-form").onsubmit = e => {
  e.preventDefault();
  const v = $("#chat-q").value.trim(); if (!v) return;
  $("#chat-q").value = "";
  if (/\bficha\b|contexto|3 capas|tres capas/i.test(v)){ dice(`<div class="yo">${esc(v)}</div>`); return abreModal("ficha"); }
  if (/\bresumen\b|resume|que le ha pasado|qué le ha pasado/i.test(v)){ dice(`<div class="yo">${esc(v)}</div>`); return abreModal("resumen"); }
  pregunta(v);
};
}


window.Agente = {
  montar(){
    document.body.insertAdjacentHTML("beforeend", MARKUP);
    enlazaBotones();
  },
  cambioDePaciente(){
    const c = chatCuerpo(), velo = $("#velo");
    if (!c || !velo) return;                 // todavía no está montado
    if (c.dataset.pac !== P().id){ c.innerHTML = ""; c.dataset.pac = ""; }
    if (!$("#chat").hidden) abreChat();
    if (!velo.hidden) pintaModal();
  }
};
})();
