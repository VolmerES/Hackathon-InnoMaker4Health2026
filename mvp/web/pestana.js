// Medici en la web. Escrita como lo haría la extensión del navegador: no sabe
// nada de la página salvo dónde está el NHC en la cinta del paciente. Lo lee,
// abre una sesión en el servidor de Medici y enseña la tarjeta y el chat de ese
// paciente. Si el médico cambia de paciente, borra todo y empieza de nuevo.
// El aspecto es el de la burbuja de escritorio de medicci-rs.
(function () {
  const SELECTOR_NHC = "#c-nhc";        // en el hospital, un ajuste que pone Informática
  const SELECTOR_NOMBRE = "#c-nombre";
  const API = "/api";
  const PREGUNTAS = ["¿Qué pruebas tiene pendientes?", "¿Por qué lo manda su médico de cabecera?",
                     "¿Ha faltado a alguna cita?", "¿Tiene alergias?"];

  const esc = s => String(s).replace(/[&<>"]/g, c => ({"&": "&amp;", "<": "&lt;", ">": "&gt;", '"': "&quot;"}[c]));
  // El médico de la burbuja, el mismo que dibuja medicci-rs (fn medico).
  const MEDICO = `<svg class="medico" viewBox="-28 -28 56 56" aria-hidden="true">
    <circle cx="0" cy="-7.5" r="6.6" fill="#fff"/>
    <path d="M-13.5 14.5A13.5 11.5 0 0 1 13.5 14.5Z" fill="#fff"/>
    <path d="M-4.6 3.2L4.6 3.2L0 10Z" fill="#1a5c44"/>
    <path d="M-6.2 4.4C-7.6 13.8 2.6 14.6 6.4 9.4" fill="none" stroke="#d9a32b" stroke-width="1.8" stroke-linecap="round"/>
    <circle cx="6.6" cy="9" r="2.5" fill="#d9a32b"/><circle cx="6.6" cy="9" r="1" fill="#fff"/></svg>`;
  const PLEGAR = `<svg class="plegar" viewBox="0 0 24 24" aria-hidden="true">
    <path d="M5 9l7 7 7-7" fill="none" stroke="#fff" stroke-width="2.4" stroke-linecap="round" stroke-linejoin="round"/></svg>`;
  const ENVIAR = `<svg width="14" height="14" viewBox="0 0 14 14" aria-hidden="true"><path d="M3 1.5L13 7L3 12.5L5 7Z" fill="#fff"/></svg>`;

  let nhcActual = null, sesion = null, pidiendo = false, tarjetaActual = "";

  document.body.insertAdjacentHTML("beforeend", `
    <div id="medici">
      <section class="panel" aria-label="Medici" id="m-panel">
        <header>
          <span class="av">${MEDICO}</span>
          <span class="tit"><b>Medici</b><small><span class="led" id="m-led"></span><span id="m-estado">IA local · nada sale del hospital</span></small></span>
          <button class="x" type="button" aria-label="Plegar Medici"><i></i></button>
        </header>
        <div class="pac"><b id="m-nombre">Abra un paciente en HCIS</b><small id="m-sub"></small></div>
        <div class="hilo" id="m-hilo" aria-live="polite"></div>
        <div class="pie">
          <div class="chips" id="m-chips"></div>
          <form id="m-form" autocomplete="off">
            <input id="m-q" placeholder="Pregunte por este paciente" aria-label="Pregunta">
            <button type="submit" aria-label="Preguntar">${ENVIAR}</button>
          </form>
        </div>
      </section>
      <button class="bola" type="button" aria-label="Abrir Medici" aria-controls="m-panel" aria-expanded="false">
        ${MEDICO}${PLEGAR}<span class="num"></span></button>
    </div>`);

  const $ = s => document.querySelector(s);
  const raiz = $("#medici");

  function abrir(si) {
    raiz.classList.toggle("abierto", si);
    raiz.querySelector(".bola").setAttribute("aria-expanded", String(si));
  }
  raiz.querySelector(".bola").onclick = () => abrir(!raiz.classList.contains("abierto"));
  raiz.querySelector(".x").onclick = () => abrir(false);

  // Las fuentes: «¹ P1»; al pulsarlas se ve la fila tal cual está.
  function refs(fuentes) {
    if (!fuentes.length) return "";
    return `<div class="fuentes">${fuentes.map(f =>
      `<button type="button" class="ref" data-texto="${esc(f.texto)}">¹ ${esc(f.ref)}</button>`).join("")}</div>`;
  }
  raiz.addEventListener("click", e => {
    const b = e.target.closest(".ref");
    if (!b) return;
    const caja = b.closest("li, .ella");
    const abierta = caja.querySelector(`.origen[data-ref="${b.textContent}"]`);
    if (abierta) return abierta.remove();
    caja.insertAdjacentHTML("beforeend", `<div class="origen" data-ref="${esc(b.textContent)}">«${esc(b.dataset.texto)}»</div>`);
  });

  async function pide(ruta, cuerpo) {
    const r = await fetch(API + ruta, cuerpo === undefined ? {} : {
      method: "POST", headers: {"Content-Type": "application/json"}, body: JSON.stringify(cuerpo)});
    const datos = await r.json();
    if (!r.ok) throw new Error(datos.error || r.statusText);
    return datos;
  }

  const hilo = () => $("#m-hilo");
  function vaciar(mensaje) {
    hilo().innerHTML = `<p class="vacio">${esc(mensaje)}</p>`;
    $("#m-chips").innerHTML = "";
    raiz.querySelector(".num").textContent = "";
  }
  function dice(html, clase) {
    hilo().querySelector(".vacio")?.remove();
    hilo().insertAdjacentHTML("beforeend", `<div class="${clase}">${html}</div>`);
    hilo().scrollTop = hilo().scrollHeight;
    return hilo().lastElementChild;
  }

  // La tarjeta se vuelve a pedir cada pocos segundos: si llega un mensaje HL7
  // (una cita anulada, un resultado), cambia sola y se marca un momento.
  async function pintaTarjeta(nhc) {
    const t = await pide(`/sesion/${sesion}/tarjeta`);
    if (nhc !== nhcActual) return;
    const clave = JSON.stringify(t);
    if (clave === tarjetaActual) return;
    const habiaOtra = tarjetaActual !== "";
    tarjetaActual = clave;
    const html = `<h3>ANTES DE QUE ENTRE</h3><ul>${t.alertas.length
      ? t.alertas.map(a => `<li class="${a.texto.startsWith("Nuevo:") ? "nuevo" : a.prioridad >= 75 ? "alta" : ""}">
          <span class="p"></span><span><span class="t">${esc(a.texto)}</span>${refs(a.fuentes)}</span></li>`).join("")
        + (t.mas ? `<li><span></span><span class="mas">y ${t.mas} cosa${t.mas > 1 ? "s" : ""} más: pregunte.</span></li>` : "")
      : `<li><span></span><span class="nada">Nada que destacar en sus datos.</span></li>`}</ul>`;
    let caja = hilo().querySelector(".tarjeta");
    if (!caja) {
      hilo().querySelector(".vacio")?.remove();
      hilo().insertAdjacentHTML("afterbegin", `<div class="tarjeta"></div>`);
      caja = hilo().querySelector(".tarjeta");
    }
    caja.innerHTML = html;
    raiz.querySelector(".num").textContent = t.alertas.length || "";
    if (habiaOtra) {
      caja.classList.add("cambio");
      setTimeout(() => caja.classList.remove("cambio"), 2500);
      abrir(true);
    }
  }
  setInterval(() => { if (sesion) pintaTarjeta(nhcActual).catch(() => {}); }, 4000);

  async function cambioDePaciente(nhc) {
    nhcActual = nhc; sesion = null; tarjetaActual = "";
    const nombre = ($(SELECTOR_NOMBRE)?.textContent || "").trim();
    $("#m-nombre").textContent = nombre ? nombre.toLowerCase().replace(/(^|\s)\S/g, c => c.toUpperCase())
                                        : (nhc ? `Paciente ${nhc}` : "Abra un paciente en HCIS");
    $("#m-sub").textContent = nhc ? `NHC ${nhc}` : "";
    if (!nhc) return vaciar("Medici se activa cuando abre un paciente en HCIS.");
    vaciar("Leyendo sus datos…");
    try {
      const s = await pide("/sesion", {nhc});
      if (nhc !== nhcActual) return;                // el médico ya ha cambiado de paciente
      sesion = s.sesion;
      $("#m-sub").textContent = `${s.edad} años · NHC ${nhc} · ${s.filas} registros`;
      hilo().innerHTML = "";
      await pintaTarjeta(nhc);
      if (nhc !== nhcActual) return;
      dice(`Pregunte lo que necesite: contesta solo con sus datos y cita la fila de la que sale.`, "ella");
      $("#m-chips").innerHTML = PREGUNTAS.map(p => `<button type="button">${esc(p)}</button>`).join("");
      abrir(true);
    } catch (e) {
      vaciar(e.message);
    }
  }

  async function preguntar(texto) {
    if (!sesion || pidiendo || !texto) return;
    pidiendo = true;
    $("#m-form button").disabled = true;
    dice(esc(texto), "yo");
    const espera = dice(`<span class="pensando">Buscando en sus datos…</span>`, "ella");
    const delPaciente = nhcActual;
    try {
      const r = await pide(`/sesion/${sesion}/pregunta`, {texto});
      if (delPaciente !== nhcActual) return;
      // Las referencias del texto ([C3]) se quitan: van debajo, como fuentes.
      const limpio = r.respuesta.replace(/\s*\[[A-Z]\d+\]/g, "");
      espera.className = "ella" + (r.consta ? "" : " no");
      espera.innerHTML = `${esc(limpio)}${refs(r.fuentes)}
        <span class="t">${r.fuentes.length ? `${r.fuentes.length} fuente${r.fuentes.length > 1 ? "s" : ""}` : "sin fuentes"} · ${r.segundos} s · IA local</span>`;
    } catch (e) {
      espera.className = "ella no";
      espera.textContent = e.message;
    } finally {
      pidiendo = false;
      $("#m-form button").disabled = false;
      hilo().scrollTop = hilo().scrollHeight;
    }
  }

  const form = $("#m-form");
  $("#m-q").addEventListener("input", () => form.classList.toggle("listo", !!$("#m-q").value.trim()));
  form.onsubmit = e => {
    e.preventDefault();
    const v = $("#m-q").value.trim();
    $("#m-q").value = "";
    form.classList.remove("listo");
    preguntar(v);
  };
  $("#m-chips").onclick = e => { const b = e.target.closest("button"); if (b) preguntar(b.textContent); };

  // Lo único que se lee de la pantalla: el NHC de la cinta del paciente.
  function revisar() {
    const nhc = ($(SELECTOR_NHC)?.textContent || "").trim() || null;
    if (nhc !== nhcActual) cambioDePaciente(nhc);
  }
  new MutationObserver(revisar).observe(document.body, {childList: true, subtree: true, characterData: true});
  revisar();

  async function estado() {
    try {
      const e = await pide("/estado");
      $("#m-led").className = "led " + (e.responde ? "si" : "no");
      $("#m-estado").textContent = e.responde
        ? `IA local (${e.modelo}) · nada sale del hospital`
        : "El modelo local no responde: la tarjeta sigue funcionando";
    } catch { /* el servidor no está: se verá al abrir un paciente */ }
  }
  estado();
  setInterval(estado, 30000);
})();
