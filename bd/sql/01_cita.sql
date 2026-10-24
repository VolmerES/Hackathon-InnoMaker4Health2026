-- =====================================================================
-- Medicci · 01 · El núcleo de la cita
-- SQL estándar: funciona en SQLite, PostgreSQL y MySQL sin cambios.
-- Transcribe el modelo de los 11 CSV del reto (docs/CONTEXTO.md).
-- Todo gira alrededor de CITAS.
-- =====================================================================

-- ---------------------------------------------------------------------
-- PACIENTES
-- ---------------------------------------------------------------------
CREATE TABLE pacientes (
  id_paciente               VARCHAR(20)  PRIMARY KEY,
  nhc                       VARCHAR(20)  UNIQUE,
  cipa                      VARCHAR(30),
  fecha_nacimiento          DATE,
  edad                      INTEGER,
  sexo                      VARCHAR(10)  CHECK (sexo IN ('Mujer','Hombre','Otro','No consta')),
  preferencias_contacto     VARCHAR(40),
  necesidades_accesibilidad VARCHAR(400),   -- intérprete, silla de ruedas, horario, acompañante
  telefono                  VARCHAR(20),
  centro_salud              VARCHAR(120),
  medico_ap                 VARCHAR(120)
);

-- ---------------------------------------------------------------------
-- DERIVACIONES_AP · lo que escribe Primaria, en texto libre
-- ---------------------------------------------------------------------
CREATE TABLE derivaciones_ap (
  id_derivacion        VARCHAR(20)  PRIMARY KEY,
  id_paciente          VARCHAR(20)  NOT NULL REFERENCES pacientes (id_paciente),
  fecha_solicitud      TIMESTAMP    NOT NULL,
  especialidad_destino VARCHAR(60)  NOT NULL,
  prioridad            VARCHAR(20)  NOT NULL DEFAULT 'Normal'
                                    CHECK (prioridad IN ('Normal','Preferente','Urgente')),
  motivo_derivacion    VARCHAR(400),
  observaciones_medico TEXT,          -- el texto tal cual: es lo que lee el agente
  centro_origen        VARCHAR(120),
  medico_solicitante   VARCHAR(120),
  estado_solicitud     VARCHAR(30)  NOT NULL DEFAULT 'Pendiente de validar'
     CHECK (estado_solicitud IN ('Pendiente de validar','Validada','Corregida',
                                 'Devuelta a Primaria','A revisar por persona','Anulada'))
);

CREATE INDEX ix_deriv_paciente ON derivaciones_ap (id_paciente);
CREATE INDEX ix_deriv_estado   ON derivaciones_ap (estado_solicitud, fecha_solicitud);

-- ---------------------------------------------------------------------
-- AGENDAS y HUECOS · la oferta
-- ---------------------------------------------------------------------
CREATE TABLE agendas (
  id_agenda            VARCHAR(20)  PRIMARY KEY,
  servicio             VARCHAR(60)  NOT NULL,
  especialidad         VARCHAR(60),
  tipo_consulta        VARCHAR(40),
  profesional_ficticio VARCHAR(120),
  duracion_cita        INTEGER,
  horario_inicio       VARCHAR(5),
  horario_fin          VARCHAR(5),
  capacidad            INTEGER
);

CREATE TABLE huecos_agenda (
  id_hueco     VARCHAR(20) PRIMARY KEY,
  id_agenda    VARCHAR(20) NOT NULL REFERENCES agendas (id_agenda),
  fecha        DATE        NOT NULL,
  hora_inicio  VARCHAR(5),
  hora_fin     VARCHAR(5),
  estado_hueco VARCHAR(20) NOT NULL DEFAULT 'Libre'
               CHECK (estado_hueco IN ('Libre','Ocupado','Bloqueado','Anulado')),
  tipo_hueco   VARCHAR(30)
);

CREATE INDEX ix_huecos_busqueda ON huecos_agenda (id_agenda, fecha, estado_hueco);

-- ---------------------------------------------------------------------
-- CITAS · la entidad central
-- ---------------------------------------------------------------------
CREATE TABLE citas (
  id_cita          VARCHAR(20) PRIMARY KEY,
  id_paciente      VARCHAR(20) NOT NULL REFERENCES pacientes (id_paciente),
  id_derivacion    VARCHAR(20) REFERENCES derivaciones_ap (id_derivacion),
  id_hueco         VARCHAR(20) REFERENCES huecos_agenda (id_hueco),
  fecha_asignacion DATE,
  tipo_visita      VARCHAR(30) NOT NULL
                   CHECK (tipo_visita IN ('Primera','Sucesiva','Revisión','Acto único','Prueba')),
  estado_actual    VARCHAR(30) NOT NULL DEFAULT 'Planificada'
                   CHECK (estado_actual IN ('Planificada','Confirmada','Atendida',
                                            'No presentada','Anulada','Reprogramada')),
  prioridad        VARCHAR(20) NOT NULL DEFAULT 'Normal',
  servicio         VARCHAR(60),
  episodio         VARCHAR(30),      -- el episodio del HCE: CEX-2026-118442
  motivo_cita      VARCHAR(400)
);

CREATE INDEX ix_citas_paciente ON citas (id_paciente, fecha_asignacion);
CREATE INDEX ix_citas_estado   ON citas (estado_actual, fecha_asignacion);

-- ---------------------------------------------------------------------
-- La vida de la cita
-- ---------------------------------------------------------------------
CREATE TABLE movimientos_citas (
  id_movimiento         VARCHAR(24) PRIMARY KEY,
  id_cita               VARCHAR(20) NOT NULL REFERENCES citas (id_cita),
  fecha_hora_movimiento TIMESTAMP   NOT NULL,
  tipo_movimiento       VARCHAR(30) NOT NULL,
  origen_movimiento     VARCHAR(30) CHECK (origen_movimiento IN
                        ('Paciente','Admisión','Servicio','Medicci','Sistema')),
  motivo                VARCHAR(400),
  estado_anterior       VARCHAR(30),
  estado_nuevo          VARCHAR(30),
  id_hueco_anterior     VARCHAR(20) REFERENCES huecos_agenda (id_hueco),
  id_hueco_nuevo        VARCHAR(20) REFERENCES huecos_agenda (id_hueco)
);

CREATE INDEX ix_mov_cita ON movimientos_citas (id_cita, fecha_hora_movimiento);

CREATE TABLE ocupaciones_hueco (
  id_ocupacion     VARCHAR(24) PRIMARY KEY,
  id_hueco         VARCHAR(20) NOT NULL REFERENCES huecos_agenda (id_hueco),
  id_cita          VARCHAR(20) NOT NULL REFERENCES citas (id_cita),
  inicio_ocupacion TIMESTAMP   NOT NULL,
  fin_ocupacion    TIMESTAMP,
  motivo_fin       VARCHAR(200),
  ocupacion_activa CHAR(1)     NOT NULL DEFAULT 'S' CHECK (ocupacion_activa IN ('S','N'))
);

-- como mucho una ocupación activa por hueco
CREATE UNIQUE INDEX ux_ocupacion_activa ON ocupaciones_hueco (id_hueco, ocupacion_activa);

CREATE TABLE comunicaciones (
  id_comunicacion    VARCHAR(24) PRIMARY KEY,
  id_paciente        VARCHAR(20) NOT NULL REFERENCES pacientes (id_paciente),
  id_cita            VARCHAR(20) REFERENCES citas (id_cita),
  id_hueco_ofertado  VARCHAR(20) REFERENCES huecos_agenda (id_hueco),
  canal              VARCHAR(30) NOT NULL
                     CHECK (canal IN ('SMS','Llamada','App','Correo','Carta','Presencial')),
  motivo             VARCHAR(200),
  fecha_envio        TIMESTAMP   NOT NULL,
  respuesta_paciente VARCHAR(200),
  resultado          VARCHAR(60)
);

CREATE INDEX ix_com_paciente ON comunicaciones (id_paciente, fecha_envio);

CREATE TABLE tareas_administrativas (
  id_tarea            VARCHAR(24) PRIMARY KEY,
  id_cita             VARCHAR(20) REFERENCES citas (id_cita),
  perfil_profesional  VARCHAR(60),
  tipo_tarea          VARCHAR(80),
  fecha_hora_inicio   TIMESTAMP,
  duracion_minutos    DECIMAL(5,1),
  automatizable       CHAR(1) NOT NULL DEFAULT 'N' CHECK (automatizable IN ('S','N')),
  automatizada_por    VARCHAR(30),      -- NULL = la hizo una persona; 'Medicci' = la resolvió el circuito
  incidencia_asociada VARCHAR(200)
);

CREATE TABLE circuito_consulta (
  id_registro            VARCHAR(24) PRIMARY KEY,
  id_cita                VARCHAR(20) NOT NULL UNIQUE REFERENCES citas (id_cita),
  hora_llegada           TIMESTAMP,
  hora_admision          TIMESTAMP,
  hora_entrada_consulta  TIMESTAMP,
  hora_salida_consulta   TIMESTAMP,
  tiempo_espera          INTEGER,
  tiempo_atencion        INTEGER,
  resuelta_en_una_visita CHAR(1) CHECK (resuelta_en_una_visita IN ('S','N'))
);

-- ---------------------------------------------------------------------
-- PRUEBAS
-- ---------------------------------------------------------------------
CREATE TABLE pruebas (
  id_prueba              VARCHAR(20)  PRIMARY KEY,
  id_paciente            VARCHAR(20)  NOT NULL REFERENCES pacientes (id_paciente),
  id_cita_origen         VARCHAR(20)  REFERENCES citas (id_cita),
  id_cita_revision       VARCHAR(20)  REFERENCES citas (id_cita),
  tipo_prueba            VARCHAR(120) NOT NULL,
  fecha_solicitud        DATE,
  fecha_programada       DATE,
  fecha_realizacion      DATE,
  estado_prueba          VARCHAR(30)  NOT NULL DEFAULT 'Solicitada'
     CHECK (estado_prueba IN ('Solicitada','Programada','Realizada','Informada','Anulada')),
  resultado_disponible   CHAR(1) NOT NULL DEFAULT 'N' CHECK (resultado_disponible IN ('S','N')),
  adelantada_por_medicci    CHAR(1) NOT NULL DEFAULT 'N' CHECK (adelantada_por_medicci IN ('S','N')),
  evitada_por_duplicidad CHAR(1) NOT NULL DEFAULT 'N' CHECK (evitada_por_duplicidad IN ('S','N'))
);

CREATE INDEX ix_pruebas_paciente ON pruebas (id_paciente, fecha_realizacion);
