//! Dictado: el micrófono se escucha solo desde que el médico pulsa el botón
//! hasta que lo vuelve a pulsar (o pasa un minuto). En la consulta está el
//! paciente hablando de su salud: nada se graba sin que el médico lo pida. El audio va al servidor de Medicci, que lo
//! transcribe con Whisper en local (mvp/voz.py) y no lo guarda.

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{FromSample, SizedSample};
use std::sync::{Arc, Mutex};

pub const MAX_SEGUNDOS: f32 = 60.0;

pub struct Grabacion {
    _flujo: cpal::Stream,           // mientras vive, el micrófono escucha
    muestras: Arc<Mutex<Vec<f32>>>,
    frecuencia: u32,
}

impl Grabacion {
    /// Empieza a escuchar el micrófono por defecto del sistema.
    pub fn empezar() -> Result<Self, String> {
        let dispositivo = cpal::default_host()
            .default_input_device()
            .ok_or("No encuentro ningún micrófono.")?;
        let config = dispositivo
            .default_input_config()
            .map_err(|e| format!("El micrófono no responde ({}).", e))?;
        let canales = config.channels() as usize;
        let frecuencia = config.sample_rate();
        let muestras = Arc::new(Mutex::new(Vec::new()));
        let flujo = match config.sample_format() {
            cpal::SampleFormat::F32 => abrir::<f32>(&dispositivo, config.config(), &muestras, canales),
            cpal::SampleFormat::I16 => abrir::<i16>(&dispositivo, config.config(), &muestras, canales),
            cpal::SampleFormat::I32 => abrir::<i32>(&dispositivo, config.config(), &muestras, canales),
            cpal::SampleFormat::U16 => abrir::<u16>(&dispositivo, config.config(), &muestras, canales),
            otro => return Err(format!("El micrófono da un formato que no sé leer ({:?}).", otro)),
        }?;
        flujo.play().map_err(|e| format!("No puedo abrir el micrófono ({}).", e))?;
        Ok(Self { _flujo: flujo, muestras, frecuencia })
    }

    pub fn segundos(&self) -> f32 {
        self.muestras.lock().map(|m| m.len()).unwrap_or(0) as f32 / self.frecuencia.max(1) as f32
    }

    /// Deja de escuchar y devuelve lo grabado como WAV, listo para mandar.
    pub fn terminar(self) -> Vec<u8> {
        let Grabacion { _flujo, muestras, frecuencia } = self;
        drop(_flujo);
        let muestras = muestras.lock().map(|m| m.clone()).unwrap_or_default();
        wav(&muestras, frecuencia)
    }
}

/// Abre el flujo del micrófono: cada trozo se pasa a mono y se guarda.
fn abrir<T: SizedSample>(dispositivo: &cpal::Device, config: cpal::StreamConfig,
                         muestras: &Arc<Mutex<Vec<f32>>>, canales: usize)
                         -> Result<cpal::Stream, String>
where
    f32: FromSample<T>,
{
    let destino = muestras.clone();
    let tope = (MAX_SEGUNDOS * config.sample_rate as f32) as usize;
    dispositivo
        .build_input_stream::<T, _, _>(
            config,
            move |datos: &[T], _| {
                if let Ok(mut m) = destino.lock() {
                    if m.len() < tope {
                        m.extend(datos.chunks(canales.max(1)).map(|marco| {
                            marco.iter().map(|x| f32::from_sample_(*x)).sum::<f32>() / marco.len() as f32
                        }));
                    }
                }
            },
            |e| eprintln!("micrófono: {}", e),
            None,
        )
        .map_err(|e| format!("No puedo abrir el micrófono ({}).", e))
}

/// WAV PCM de 16 bits y un canal: lo que lee cualquier transcriptor.
pub fn wav(muestras: &[f32], frecuencia: u32) -> Vec<u8> {
    let datos = (muestras.len() * 2) as u32;
    let mut b = Vec::with_capacity(44 + datos as usize);
    b.extend_from_slice(b"RIFF");
    b.extend_from_slice(&(36 + datos).to_le_bytes());
    b.extend_from_slice(b"WAVEfmt ");
    b.extend_from_slice(&16u32.to_le_bytes());          // tamaño del bloque fmt
    b.extend_from_slice(&1u16.to_le_bytes());           // PCM
    b.extend_from_slice(&1u16.to_le_bytes());           // mono
    b.extend_from_slice(&frecuencia.to_le_bytes());
    b.extend_from_slice(&(frecuencia * 2).to_le_bytes()); // bytes por segundo
    b.extend_from_slice(&2u16.to_le_bytes());           // bytes por muestra
    b.extend_from_slice(&16u16.to_le_bytes());          // bits por muestra
    b.extend_from_slice(b"data");
    b.extend_from_slice(&datos.to_le_bytes());
    for m in muestras {
        b.extend_from_slice(&((m.clamp(-1.0, 1.0) * i16::MAX as f32) as i16).to_le_bytes());
    }
    b
}

#[cfg(test)]
mod pruebas {
    use super::*;

    #[test]
    fn el_wav_tiene_su_cabecera_y_sus_muestras() {
        let w = wav(&[0.0, 1.0, -1.0, 2.0], 48_000);
        assert_eq!(&w[0..4], b"RIFF");
        assert_eq!(&w[8..16], b"WAVEfmt ");
        assert_eq!(u32::from_le_bytes(w[24..28].try_into().unwrap()), 48_000);
        assert_eq!(u32::from_le_bytes(w[40..44].try_into().unwrap()), 8);   // 4 muestras de 2 bytes
        assert_eq!(w.len(), 44 + 8);
        // lo que pasa de 1 se recorta, no da la vuelta
        assert_eq!(i16::from_le_bytes([w[50], w[51]]), i16::MAX);
        assert_eq!(i16::from_le_bytes([w[48], w[49]]), -i16::MAX);
    }
}
