// Pestaña Medici. Escrita como lo haría la extensión del navegador: no sabe nada
// de la página salvo dónde está el NHC en la cinta del paciente. Lo lee, abre una
// sesión en el servidor de Medici y enseña la tarjeta y el chat de ese paciente.
// Si el médico cambia de paciente, borra todo y empieza de nuevo.
(function () {
  const SELECTOR_NHC = "#c-nhc";        // en el hospital, un ajuste que pone Informática
  const SELECTOR_NOMBRE = "#c-nombre";
  const API = "/api";
  const PREGUNTAS = ["¿Qué pruebas tiene pendientes?", "¿Por qué lo manda su médico de cabecera?",
                     "¿Ha faltado a alguna cita?", "¿Tiene alergias?"];

  const esc = s => String(s).replace(/[&<>"]/g, c => ({"&": "&amp;", "<": "&lt;", ">": "&gt;", '"': "&quot;"}[c]));
  let nhcActual = null, sesion = null, pidiendo = false;

  document.body.insertAdjacentHTML("beforeend", `
    <aside id="medici" aria-label="Medici">
      <button class="asa" type="button" aria-expanded="true">MEDICI <span class="num"></span></button>
      <div class="panel">
        <header>
          <span class="logo">MEDICI</span>
          <span class="quien"><b id="m-nombre">Abra un paciente en HCIS</b><small id="m-sub"></small></span>
          <button class="x" type="button" aria-label="Cerrar Medici">×</button>
        </header>
        <section class="tarjeta" aria-live="polite">
          <h3>Antes de que entre</h3>
          <ul id="m-tarjeta"></ul>
        </section>
        <div class="chat" id="m-chat" aria-live="polite"></div>
        <div class="chips" id="m-chips"></div>
        <form id="m-form" autocomplete="off">
          <input id="m-q" placeholder="Pregunte por este paciente" aria-label="Pregunta">
          <button type="submit">Preguntar</button>
        </form>
        <footer><span class="led" id="m-led"></span><span id="m-estado">IA local · nada sale del hospital</span></footer>
      </div>
    </aside>`);

  const $ = s => document.querySelector(s);
  const raiz = $("#medici");

  function abrir(si) {
    raiz.classList.toggle("cerrada", !si);
    document.documentElement.classList.toggle("medici-abierta", si);
    raiz.querySelector(".asa").setAttribute("aria-expanded", String(si));
  }
  raiz.querySelector(".asa").onclick = () => abrir(raiz.classList.contains("cerrada"));
  raiz.querySelector(".x").onclick = () => abrir(false);
  abrir(true);

  // Las fuentes: un botón por fila; al pulsarlo se ve el dato tal cual está.
  function refs(fuentes) {
    return fuentes.map(f => `<button type="button" class="ref" data-texto="${esc(f.texto)}">${esc(f.ref)}</button>`).join("");
  }
  raiz.addEventListener("click", e => {
    const b = e.target.closest(".ref");
    if (!b) return;
    const sig = b.parentElement.querySelector(`.origen[data-ref="${b.textContent}"]`);
    if (sig) return sig.remove();
    b.parentElement.insertAdjacentHTML("beforeend", `<div class="origen" data-ref="${esc(b.textContent)}">${esc(b.dataset.texto)}</div>`);
  });

  async function pide(ruta, cuerpo) {
    const r = await fetch(API + ruta, cuerpo === undefined ? {} : {
      method: "POST", headers: {"Content-Type": "application/json"}, body: JSON.stringify(cuerpo)});
    const datos = await r.json();
    if (!r.ok) throw new Error(datos.error || r.statusText);
    return datos;
  }

  function vaciar(mensaje) {
    $("#m-tarjeta").innerHTML = "";
    $("#m-chat").innerHTML = `<p class="vacio">${esc(mensaje)}</p>`;
    $("#m-chips").innerHTML = "";
    raiz.querySelector(".num").textContent = "";
  }

  // La tarjeta se vuelve a pedir cada pocos segundos: si llega un mensaje HL7
  // (una cita anulada, un resultado), cambia sola y se marca un momento.
  let tarjetaActual = "";
  async function pintaTarjeta(nhc) {
    const t = await pide(`/sesion/${sesion}/tarjeta`);
    if (nhc !== nhcActual) return;
    const clave = JSON.stringify(t);
    if (clave === tarjetaActual) return;
    const habiaOtra = tarjetaActual !== "";
    tarjetaActual = clave;
    $("#m-tarjeta").innerHTML = t.alertas.length
      ? t.alertas.map(a => `<li class="${a.prioridad >= 75 ? "alta" : ""}"><span class="p"></span>
          <span>${esc(a.texto)}${refs(a.fuentes)}</span></li>`).join("")
        + (t.mas ? `<li class="mas">y ${t.mas} cosa${t.mas > 1 ? "s" : ""} más: pregunte.</li>` : "")
      : `<li class="nada">Nada que destacar en sus datos.</li>`;
    raiz.querySelector(".num").textContent = t.alertas.length || "";
    if (habiaOtra) {
      const caja = raiz.querySelector(".tarjeta");
      caja.classList.add("cambio");
      setTimeout(() => caja.classList.remove("cambio"), 2500);
    }
  }
  setInterval(() => { if (sesion) pintaTarjeta(nhcActual).catch(() => {}); }, 4000);

  async function cambioDePaciente(nhc) {
    tarjetaActual = "";
    nhcActual = nhc; sesion = null;
    const nombre = ($(SELECTOR_NOMBRE)?.textContent || "").trim();
    $("#m-nombre").textContent = nombre || (nhc ? `Paciente ${nhc}` : "Abra un paciente en HCIS");
    $("#m-sub").textContent = nhc ? `NHC ${nhc}` : "";
    if (!nhc) return vaciar("Medici se activa cuando abre un paciente en HCIS.");
    vaciar("Leyendo sus datos…");
    try {
      const s = await pide("/sesion", {nhc});
      if (nhc !== nhcActual) return;                // el médico ya ha cambiado de paciente
      sesion = s.sesion;
      $("#m-sub").textContent = `NHC ${nhc} · ${s.edad} años · ${s.filas} registros`;
      await pintaTarjeta(nhc);
      if (nhc !== nhcActual) return;
      $("#m-chat").innerHTML = `<p class="vacio">Pregunte lo que necesite de este paciente.<br>
        Responde solo con sus datos y enseña de dónde sale cada cosa.</p>`;
      $("#m-chips").innerHTML = PREGUNTAS.map(p => `<button type="button">${esc(p)}</button>`).join("");
    } catch (e) {
      vaciar(e.message);
    }
  }

  function dice(html, clase) {
    const chat = $("#m-chat");
    chat.querySelector(".vacio")?.remove();
    chat.insertAdjacentHTML("beforeend", `<div class="${clase}">${html}</div>`);
    chat.scrollTop = chat.scrollHeight;
    return chat.lastElementChild;
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
      // Las referencias del texto ([C3]) se quitan: van como botones debajo.
      const limpio = r.respuesta.replace(/\s*\[[A-Z]\d+\]/g, "");
      espera.className = "ella" + (r.consta ? "" : " no");
      espera.innerHTML = `${esc(limpio)}<div>${refs(r.fuentes)}</div>
        <span class="t">${r.fuentes.length ? `${r.fuentes.length} fuente${r.fuentes.length > 1 ? "s" : ""}` : "sin fuentes"} · ${r.segundos} s</span>`;
    } catch (e) {
      espera.className = "ella no";
      espera.textContent = e.message;
    } finally {
      pidiendo = false;
      $("#m-form button").disabled = false;
    }
  }

  $("#m-form").onsubmit = e => {
    e.preventDefault();
    const v = $("#m-q").value.trim();
    $("#m-q").value = "";
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
