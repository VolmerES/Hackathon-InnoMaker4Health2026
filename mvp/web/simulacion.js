// La mañana de un médico con Medicci, de principio a fin, para grabarla en
// vídeo (mvp/simular.py). Se carga en la réplica de HCIS con ?simulacion=1:
// pone los rótulos, enseña el arranque del ordenador y abre los pacientes en
// HCIS. La burbuja nativa (medicci.exe --simulacion) sigue al paciente por el
// título de la ventana, como siempre, y hace lo que le toca en cada paso
// (abrirse, preguntar, ampliar), que le llega por /api/simulacion.
(function () {
  const params = new URLSearchParams(location.search);
  const enRegistro = location.pathname.endsWith("registro.html");
  const MEDICO = "med.demo01";

  // El título de esta ventana lleva «· simulación»: así simular.py y la
  // burbuja de la simulación la distinguen de otro HCIS que esté abierto.
  const MARCA = " · simulación";
  const marca = () => { if (!document.title.includes(MARCA)) document.title += MARCA; };
  marca();
  new MutationObserver(marca).observe(document.querySelector("title"), {childList: true});

  document.head.insertAdjacentHTML("beforeend", `<style>
    #sim-tapa{position:fixed;inset:0;z-index:50;display:flex;align-items:center;justify-content:center;
      font-family:"Segoe UI",Verdana,sans-serif;transition:opacity .6s}
    #sim-tapa.fuera{opacity:0;pointer-events:none}
    #sim-tapa.negro{background:#000}
    #sim-tapa.arranque{background:#000;flex-direction:column;gap:48px}
    .sim-logo{display:grid;grid-template-columns:repeat(2,44px);gap:5px}
    .sim-logo i{display:block;width:44px;height:44px;background:#3aa0f0}
    .sim-giro{width:38px;height:38px;border:4px solid #ffffff22;border-top-color:#fff;border-radius:50%;animation:sim-g 1s linear infinite}
    @keyframes sim-g{to{transform:rotate(360deg)}}
    #sim-tapa.windows{background:radial-gradient(circle at 30% 20%,#2c5c8f,#0b1e33 70%);color:#fff;flex-direction:column;gap:14px}
    .sim-avatar{width:120px;height:120px;border-radius:50%;background:#ffffff26;display:flex;align-items:center;justify-content:center;font-size:52px}
    .sim-nombre{font-size:28px}.sim-sub{font-size:14px;opacity:.8}
    .sim-campo{width:300px;height:38px;border-radius:4px;background:#ffffffdd;color:#222;display:flex;align-items:center;
      padding:0 12px;font-size:22px;letter-spacing:4px;margin-top:10px}
    #sim-tapa.escritorio{background:linear-gradient(160deg,#00704d,#009b6b 55%,#7fd1b0);color:#fff;align-items:flex-start;justify-content:flex-start}
    .sim-iconos{display:flex;flex-direction:column;gap:26px;padding:30px}
    .sim-icono{width:92px;text-align:center;font-size:12.5px;text-shadow:0 1px 2px #0006}
    .sim-icono b{display:block;width:54px;height:54px;margin:0 auto 6px;border-radius:10px;background:#fff;color:#00704d;
      font:bold 15px/54px Verdana;box-shadow:0 2px 6px #0004}
    .sim-icono.pulsado b{outline:3px solid #f3c418}
    .sim-barra{position:fixed;left:0;right:0;bottom:0;height:44px;background:#0b2a1ecc;display:flex;align-items:center;gap:14px;padding:0 14px;font-size:13px}
    .sim-barra .reloj{margin-left:auto;text-align:right;line-height:1.2}
    #sim-tapa.hcis{background:#ececec;font-family:Verdana,sans-serif}
    .sim-login{width:380px;background:#fff;border:1px solid #c6c6c6;border-radius:6px;overflow:hidden;box-shadow:0 6px 24px #0002}
    .sim-login h2{margin:0;background:#009b6b;color:#fff;font-size:16px;padding:12px 16px}
    .sim-login div{padding:16px;display:grid;gap:10px;font-size:13px}
    .sim-login span{border:1px solid #c6c6c6;border-radius:4px;padding:7px 9px;min-height:32px;background:#fafafa}
    .sim-login button{background:#009b6b;color:#fff;border:0;border-radius:4px;padding:9px;font-weight:bold}
    /* los rótulos, con la identidad de Medicci (marca/IDENTIDAD.md) */
    @font-face{font-family:"IBM Plex Sans";src:url(/fuentes/IBMPlexSans-Regular.ttf)}
    @font-face{font-family:"IBM Plex Sans";font-weight:600;src:url(/fuentes/IBMPlexSans-SemiBold.ttf)}
    #sim-rotulo{position:fixed;left:24px;bottom:84px;z-index:60;max-width:760px;background:#0E3B36f2;color:#F6F1E6;
      border-left:6px solid #D4AF37;border-radius:12px;padding:14px 20px;font:17px/1.45 "IBM Plex Sans",Arial,sans-serif;
      box-shadow:0 8px 30px #0005;transition:opacity .4s,transform .4s}
    #sim-rotulo.oculto{opacity:0;transform:translateY(12px)}
    #sim-rotulo .hora{display:block;font-size:13px;letter-spacing:.08em;color:#D4AF37;font-weight:600;margin-bottom:3px}
  </style>`);
  document.body.insertAdjacentHTML("beforeend", `<div id="sim-tapa" class="negro"></div>
    <div id="sim-rotulo" class="oculto"><span class="hora"></span><span class="texto"></span></div>`);

  const tapa = document.getElementById("sim-tapa");
  const espera = ms => new Promise(r => setTimeout(r, ms));
  const paso = (accion, texto = "") => fetch("/api/simulacion/paso", {method: "POST",
    headers: {"Content-Type": "application/json"}, body: JSON.stringify({accion, texto})}).catch(() => {});
  function escena(clase, html = "") { tapa.className = clase; tapa.innerHTML = html; }
  async function rotulo(hora, texto, ms) {
    const r = document.getElementById("sim-rotulo");
    r.classList.add("oculto");
    await espera(350);
    r.querySelector(".hora").textContent = hora;
    r.querySelector(".texto").textContent = texto;
    r.classList.remove("oculto");
    if (ms) await espera(ms);
  }
  async function escribe(el, texto, ms = 90) {
    for (let i = 1; i <= texto.length; i++) { el.textContent = texto.slice(0, i); await espera(ms); }
  }

  async function arranque() {
    escena("negro");
    await rotulo("07:55", "El médico llega a la consulta y enciende el ordenador.", 2500);
    escena("arranque", `<div class="sim-logo"><i></i><i></i><i></i><i></i></div><div class="sim-giro"></div>`);
    await espera(3000);
    escena("windows", `<div class="sim-avatar">👤</div><div class="sim-nombre">${MEDICO}</div>
      <div class="sim-sub">Hospital Universitario · Consultas externas</div><div class="sim-campo"></div>`);
    await rotulo("07:56", "Entra en Windows con su usuario del hospital, como cada día.", 900);
    await escribe(tapa.querySelector(".sim-campo"), "••••••••", 140);
    await espera(700);
    escena("escritorio", `<div class="sim-iconos">
        <div class="sim-icono" id="sim-hcis"><b>HCIS</b>HCIS</div>
        <div class="sim-icono"><b>@</b>Correo</div>
        <div class="sim-icono"><b>▤</b>Intranet</div></div>`);
    await paso("windows");                       // simular.py abre aquí medicci.exe, como al iniciar Windows
    await rotulo("07:57", "Medicci arranca sola con Windows: es el círculo verde de abajo a la derecha. No hay que instalar ni abrir nada más.", 6500);
    document.getElementById("sim-hcis").classList.add("pulsado");
    await rotulo("07:58", "Abre HCIS y entra con su usuario.", 1200);
    escena("hcis", `<div class="sim-login"><h2>HCIS · Consultas externas</h2><div>
      Usuario<span id="sim-u"></span>Contraseña<span id="sim-c"></span><button>Entrar</button></div></div>`);
    await escribe(document.getElementById("sim-u"), MEDICO);
    await escribe(document.getElementById("sim-c"), "••••••••", 70);
    await espera(600);
    tapa.classList.add("fuera");
  }

  async function manana() {
    while (agenda.length < 8) await espera(500);
    const maria = agenda[0];
    const cronico = agenda.find(p => p.cronico) || agenda[2];
    await rotulo("08:58", `Su agenda de hoy en HCIS: ${agenda.length} pacientes, ${agenda.filter(p => p.cronico).length} de ellos crónicos con años de historia.`, 5000);

    abrePaciente(maria);
    await rotulo("09:00", `Antes de que entre ${maria.nombre}, abre su historia en HCIS, como siempre.`, 4000);
    await paso("abrir");
    await rotulo("09:00", "La burbuja ve en HCIS qué paciente ha abierto y ya tiene su resumen: nadie ha escrito nada.", 6000);
    await rotulo("09:01", "Lo primero, lo que no se puede pasar: tres avisos, cada uno con la fila de la que sale (F1, P1, D1).", 6500);
    await paso("preguntar", "¿Qué pruebas tiene pendientes?");
    await rotulo("09:01", "Pregunta como lo diría en voz alta. Contesta una IA local, en este ordenador: los datos no salen del hospital.", 9000);
    await paso("ampliar");
    await rotulo("09:02", "Ampliar: todo el resumen en una ventana, con la historia y lo pendiente.", 6500);
    await paso("cerrar_ampliar");
    await rotulo("09:12", "Ya en consulta, HCIS manda por HL7 el resultado de la radiografía…", 2500);
    await fetch("/api/simulacion/evento", {method: "POST", headers: {"Content-Type": "application/json"},
      body: JSON.stringify({nhc: maria.nhc, tipo: "resultado"})}).catch(() => {});
    await rotulo("09:12", "…y Medicci lo enseña en el momento, con su fuente. Nadie ha tenido que buscarlo.", 7500);
    await paso("plegar");

    abrePaciente(cronico);
    await rotulo("09:40", `Siguiente: ${cronico.nombre}, crónico con años de historia y miles de registros.`, 4500);
    await paso("abrir");
    await rotulo("09:40", "Con el mismo ordenador y el mismo modelo pequeño: el resumen sale igual de rápido.", 5500);
    await paso("preguntar", "¿Cuántas citas se cerraron sin atenderle?");
    await rotulo("09:41", "Las cuentas no las hace la IA: las hace la base de datos. La IA las ordena y cita de dónde salen.", 9500);
    await paso("plegar");
    await rotulo("14:00", "Al terminar la mañana, todo lo que ha consultado queda registrado a su nombre.", 3500);
    location.href = `/registro.html?simulacion=1&medico=${encodeURIComponent(MEDICO)}`;
  }

  async function registro() {
    tapa.classList.add("fuera");
    await rotulo("14:00", `Registro de accesos: qué ha mirado ${MEDICO}, de qué paciente y qué le contestó Medicci.`, 7000);
    await rotulo("14:00", "Cada línea lleva la huella de la anterior: si alguien la borra o la cambia, se nota.", 7000);
    await rotulo("", "Medicci · el médico no abre nada nuevo: abre HCIS como siempre, y Medicci ya está ahí.", 6000);
    await paso("fin");
  }

  // espera a que simular.py diga «empezar», ya grabando
  async function senal() {
    for (;;) {
      const s = await fetch("/api/simulacion").then(r => r.json()).catch(() => ({}));
      if (s.accion === "empezar") return;
      await espera(300);
    }
  }

  // ?simulacion=libre: sin guion propio. Hace lo que le mandan por
  // /api/simulacion (mvp/gifs.py): rótulos, abrir un paciente, ir a otra página
  async function libre() {
    tapa.classList.add("fuera");
    let visto = null;
    for (;;) {
      const s = await fetch("/api/simulacion").then(r => r.json()).catch(() => null);
      if (s && visto !== null && s.n !== visto) {
        const [hora, texto] = s.texto.includes("|") ? s.texto.split("|") : ["", s.texto];
        if (s.accion === "rotulo") rotulo(hora, texto);
        if (s.accion === "sin_rotulo") document.getElementById("sim-rotulo").classList.add("oculto");
        if (s.accion === "paciente" && typeof abrePaciente === "function") {
          const p = agenda.find(x => x.nhc === s.texto);
          if (p) abrePaciente(p);
        }
        if (s.accion === "ir") location.href = s.texto;
      }
      if (s) visto = s.n;
      await espera(250);
    }
  }

  if (params.get("simulacion") === "libre") libre();
  else if (enRegistro) registro();
  else if (params.has("simulacion")) senal().then(arranque).then(manana);
})();
