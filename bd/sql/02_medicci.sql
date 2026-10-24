-- =====================================================================
-- Medicci · 02 · Lo que añade el proyecto
-- La historia que se lee, lo que el modelo extrae de ella, la ficha que
-- se genera para la cita, lo que pregunta el médico y lo que valida.
-- Regla de oro: ningún dato extraído existe sin la frase literal y el
-- documento del que sale.
-- =====================================================================

-- ---------------------------------------------------------------------
-- DOCUMENTOS_HCE · la historia tal cual está en SELENE / HCIS
-- ---------------------------------------------------------------------
CREATE TABLE documentos_hce (
  id_documento   VARCHAR(20)  PRIMARY KEY,         -- D06, E13...
  id_paciente    VARCHAR(20)  NOT NULL REFERENCES pacientes (id_paciente),
  fecha_documento DATE        NOT NULL,
  tipo_documento VARCHAR(60)  NOT NULL,            -- Analítica, Informe de alta, Nota de enfermería...
  servicio       VARCHAR(80),
  autor          VARCHAR(120),
  origen         VARCHAR(30)  NOT NULL DEFAULT 'HCE'
                 CHECK (origen IN ('HCE','SELENE','HCIS','Primaria','Laboratorio','Externo')),
  texto          TEXT         NOT NULL,            -- el texto libre completo
  palabras       INTEGER
);

CREATE INDEX ix_doc_paciente ON documentos_hce (id_paciente, fecha_documento);
CREATE INDEX ix_doc_tipo     ON documentos_hce (tipo_documento);

-- ---------------------------------------------------------------------
-- LECTURAS · cada vez que el modelo local lee una historia
-- ---------------------------------------------------------------------
CREATE TABLE lecturas (
  id_lectura       VARCHAR(24) PRIMARY KEY,
  id_paciente      VARCHAR(20) NOT NULL REFERENCES pacientes (id_paciente),
  id_cita          VARCHAR(20) REFERENCES citas (id_cita),
  fecha_lectura    TIMESTAMP   NOT NULL,
  modelo           VARCHAR(80),           -- p. ej. bsc/roberta-base-biomedical-clinical-es
  version_modelo   VARCHAR(40),
  temperatura      DECIMAL(3,2) NOT NULL DEFAULT 0,
  documentos_leidos INTEGER,
  milisegundos     INTEGER,
  ejecutada_en     VARCHAR(60)            -- servidor del hospital; nunca sale de la red
);

-- ---------------------------------------------------------------------
-- HECHOS · lo que el modelo extrae, siempre con su cita literal
-- ---------------------------------------------------------------------
CREATE TABLE hechos (
  id_hecho      VARCHAR(24)  PRIMARY KEY,
  id_lectura    VARCHAR(24)  NOT NULL REFERENCES lecturas (id_lectura),
  id_paciente   VARCHAR(20)  NOT NULL REFERENCES pacientes (id_paciente),
  id_documento  VARCHAR(20)  REFERENCES documentos_hce (id_documento),
  categoria     VARCHAR(40)  NOT NULL,   -- alergias, medicacion, pruebas, contradiccion, alarma...
  afirmacion    VARCHAR(600) NOT NULL,   -- la frase que se le enseña al médico
  cita_literal  TEXT,                    -- subcadena exacta del documento: la prueba
  pos_inicio    INTEGER,                 -- offset dentro del texto, para resaltarlo
  pos_fin       INTEGER,
  confianza     DECIMAL(4,3),
  consta        CHAR(1) NOT NULL DEFAULT 'S' CHECK (consta IN ('S','N')),
  CONSTRAINT ck_hecho_prueba CHECK (
    (consta = 'N' AND id_documento IS NULL) OR
    (consta = 'S' AND id_documento IS NOT NULL AND cita_literal IS NOT NULL)
  )
);

CREATE INDEX ix_hechos_paciente ON hechos (id_paciente, categoria);
CREATE INDEX ix_hechos_lectura  ON hechos (id_lectura);

-- consta = 'N' guarda las ausencias comprobadas: «no consta ninguna gastroscopia».
-- Un hecho con consta = 'S' no puede existir sin documento y frase: lo impide el CHECK.

-- ---------------------------------------------------------------------
-- FICHAS · la ficha de primera consulta que se genera para una cita
-- ---------------------------------------------------------------------
CREATE TABLE fichas_consulta (
  id_ficha      VARCHAR(24) PRIMARY KEY,
  id_cita       VARCHAR(20) NOT NULL REFERENCES citas (id_cita),
  id_lectura    VARCHAR(24) NOT NULL REFERENCES lecturas (id_lectura),
  generada_en   TIMESTAMP   NOT NULL,
  formato       VARCHAR(30) NOT NULL DEFAULT 'medicci-v1',
  campos_con_dato INTEGER,
  campos_sin_dato INTEGER,
  vista_por     VARCHAR(60),             -- usuario del HCE que la abrió
  vista_en      TIMESTAMP
);

CREATE TABLE ficha_campos (
  id_campo    VARCHAR(24)  PRIMARY KEY,
  id_ficha    VARCHAR(24)  NOT NULL REFERENCES fichas_consulta (id_ficha),
  bloque      VARCHAR(40)  NOT NULL,     -- antes_de_empezar, por_que_viene, constantes, vida...
  seccion     VARCHAR(60),
  etiqueta    VARCHAR(80)  NOT NULL,     -- Alergias, Implantes, Peso · IMC...
  valor       VARCHAR(600) NOT NULL,     -- 'Metamizol' o 'No consta'
  detalle     VARCHAR(800),
  id_hecho     VARCHAR(24) REFERENCES hechos (id_hecho),
  id_documento VARCHAR(20) REFERENCES documentos_hce (id_documento),
  orden        INTEGER,
  destacado   CHAR(1) NOT NULL DEFAULT 'N' CHECK (destacado IN ('S','N'))
);

CREATE INDEX ix_campos_ficha ON ficha_campos (id_ficha, bloque, orden);

-- ---------------------------------------------------------------------
-- PREGUNTAS · lo que el médico le pregunta a la historia
-- ---------------------------------------------------------------------
CREATE TABLE preguntas_agente (
  id_pregunta    VARCHAR(24) PRIMARY KEY,
  id_cita        VARCHAR(20) REFERENCES citas (id_cita),
  id_paciente    VARCHAR(20) NOT NULL REFERENCES pacientes (id_paciente),
  usuario        VARCHAR(60),
  pregunta       VARCHAR(600) NOT NULL,
  tipo_respuesta VARCHAR(20)  NOT NULL
                 CHECK (tipo_respuesta IN ('valor','hechos','literal','no_consta')),
  respuesta      VARCHAR(1200),
  milisegundos   INTEGER,
  fecha_hora     TIMESTAMP   NOT NULL,
  util           CHAR(1) CHECK (util IN ('S','N'))   -- el médico puede marcar si le sirvió
);

CREATE TABLE pregunta_hechos (          -- qué hechos sostuvieron cada respuesta
  id_pregunta VARCHAR(24) NOT NULL REFERENCES preguntas_agente (id_pregunta),
  id_hecho    VARCHAR(24) NOT NULL REFERENCES hechos (id_hecho),
  PRIMARY KEY (id_pregunta, id_hecho)
);

-- ---------------------------------------------------------------------
-- VALIDACIONES · el especialista decide, el circuito solo propone
-- ---------------------------------------------------------------------
CREATE TABLE protocolos_servicio (
  id_protocolo VARCHAR(24)  PRIMARY KEY,
  servicio     VARCHAR(60)  NOT NULL,
  nombre       VARCHAR(160) NOT NULL,
  version      VARCHAR(10)  NOT NULL,
  aprobado_por VARCHAR(120),
  aprobado_el  DATE,
  vigente      CHAR(1) NOT NULL DEFAULT 'S' CHECK (vigente IN ('S','N'))
);

CREATE TABLE validaciones_derivacion (
  id_validacion   VARCHAR(24) PRIMARY KEY,
  id_derivacion   VARCHAR(20) NOT NULL REFERENCES derivaciones_ap (id_derivacion),
  id_lectura      VARCHAR(24) REFERENCES lecturas (id_lectura),
  id_protocolo    VARCHAR(24) REFERENCES protocolos_servicio (id_protocolo),
  ruta_propuesta  VARCHAR(30) NOT NULL
                  CHECK (ruta_propuesta IN ('prueba','revision','devolver','persona')),
  ruta_final      VARCHAR(30) NOT NULL
                  CHECK (ruta_final    IN ('prueba','revision','devolver','persona')),
  corregida       CHAR(1) NOT NULL DEFAULT 'N' CHECK (corregida IN ('S','N')),
  motivo_correccion VARCHAR(400),
  facultativo     VARCHAR(120) NOT NULL,
  fecha_hora      TIMESTAMP    NOT NULL,
  segundos_validacion INTEGER
);

CREATE INDEX ix_valid_derivacion ON validaciones_derivacion (id_derivacion);

-- Trazabilidad: quién abre qué. Los datos de salud no salen del hospital,
-- pero cada acceso queda registrado.
CREATE TABLE accesos (
  id_acceso   VARCHAR(24) PRIMARY KEY,
  usuario     VARCHAR(60) NOT NULL,
  id_paciente VARCHAR(20) NOT NULL REFERENCES pacientes (id_paciente),
  id_cita     VARCHAR(20) REFERENCES citas (id_cita),
  accion      VARCHAR(40) NOT NULL,    -- abrir_ficha, preguntar, abrir_documento, validar
  objeto      VARCHAR(40),
  fecha_hora  TIMESTAMP   NOT NULL,
  sistema     VARCHAR(20)             -- SELENE, HCIS, bandeja
);

CREATE INDEX ix_accesos_paciente ON accesos (id_paciente, fecha_hora);
