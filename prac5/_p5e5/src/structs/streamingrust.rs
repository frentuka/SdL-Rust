/*

3 -La plataforma de streaming "StreamingRust" ofrece distintos tipos de suscripciones (Basic, Clasic, Super) a sus usuarios.
    Cada suscripción tiene un costo mensual, una duración en meses y una fecha de inicio.
    Además, los usuarios pueden pagar por sus suscripciones con distintos medios de pago
        que son Efectivo, MercadoPago, Tarjeta de Crédito, Transferencia Bancaria, o Cripto.
    Cada medio de pago tiene sus datos correspondientes a excepción de Efectivo.
    Los usuarios solo pueden tener una suscripción activa a la vez.
    Implemente las estructuras, funciones asociadas y traits necesarios para resolver las siguientes acciones:

➢ Crear un usuario con una determinada suscripción y medio de pago.
➢ Dado un usuario hacer un upgrade sobre la suscripción. Es decir si está a Basic pasa a Clasic y si está en Clasic pasa a Super.
➢ Dado un determinado usuario, hacer un downgrade sobre una suscripción, si la suscripción es del tipo Basic al hacerlo se cancelará la suscripción.
➢ Dado un usuario cancelar la suscripción.
➢ Saber el medio de pago que es más utilizado por los usuarios sobre las suscripciones activas
➢ Saber cual es la suscripción más contratada por los usuarios sobre las suscripciones activas.
➢ Saber cuál fue el medio de pago más utilizado.
➢ Saber cuál fue la suscripción más contratada.

*/
use std::collections::{BTreeMap, HashMap};
use std::mem::{discriminant, Discriminant};
use error_proc_macro::Error;
use serde::{Deserialize, Serialize};
use crate::structs::streamingrust_fm::{ErrorSobreescribirArchivo, StreamingRustFileManagement, UsuariosFile};
use crate::structs::suscripcion::{MedioDePago, TipoSuscripcion};
use crate::structs::usuario::Usuario;

type Usuarios = BTreeMap<u64, Usuario>;

const BASE_FOLDER: &str = "R:/appcrap/RustRover/SdL-Rust/prac5/_p5e5/res/";
const DEFAULT_FILE_NAME: &str = "streamingrust_usuarios.json";

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct StreamingRust {
    pub usuarios: Usuarios,
    pub file_name: String,
}
impl PartialEq for StreamingRust {
    fn eq(&self, other: &Self) -> bool {
        if self.usuarios.len() == other.usuarios.len() {
            self.usuarios.iter().all(|(id, user)| {
                let Some(other_user) = other.usuarios.get(id)
                else { return false };

                user == other_user
            });
        }

        true
    }
}

#[derive(Error)]
#[derive(PartialEq)]
pub enum ErrorRegistrarUsuario {
    UsuarioYaExiste,
    Archivo(ErrorSobreescribirArchivo)
}

#[derive(Error, PartialEq)]
pub enum ErrorMejorarSuscripcion {
    UsuarioInexistente,
    SuscripcionMaxima,
    SinSuscripcion,
    Archivo(ErrorSobreescribirArchivo)
}

#[derive(Error, PartialEq)]
pub enum ErrorDegradarSuscripcion {
    UsuarioInexistente,
    SuscripcionMinima,
    SinSuscripcion,
    Archivo(ErrorSobreescribirArchivo)
}

impl StreamingRust {
    pub fn new(file_name: &str, usuarios: Option<Usuarios>) -> Result<StreamingRust, ErrorSobreescribirArchivo> {
        let sr = StreamingRust { usuarios: usuarios.unwrap_or_leer(file_name), file_name: file_name.to_string() };
        sr.sobreescribir_archivo_usuarios(file_name)?;
        Ok(sr)
    }

    pub fn registrar_usuario(&mut self, user: Usuario) -> Result<(), ErrorRegistrarUsuario> {
        if self.usuarios.contains_key(&user.id) {
            return Err(ErrorRegistrarUsuario::UsuarioYaExiste)
        }

        self.usuarios.insert(user.id, user);
        match self.sobreescribir_archivo_usuarios(DEFAULT_FILE_NAME) {
            Ok(()) => { Ok(()) }
            Err(error) => { Err(ErrorRegistrarUsuario::Archivo(error)) }
        }
    }

    pub fn mejorar_suscripcion(&mut self, user_id: u64) -> Result<TipoSuscripcion, ErrorMejorarSuscripcion> {
        let Some(user) = self.usuarios.get_mut(&user_id)
            else { return Err(ErrorMejorarSuscripcion::UsuarioInexistente) };

        let Some(sus) = user.obtener_suscripcion()
            else { return Err(ErrorMejorarSuscripcion::SinSuscripcion) };

        match sus.mejorar() {
            Ok(nue_sus) => {
                match self.sobreescribir_archivo_usuarios(DEFAULT_FILE_NAME) {
                    Ok(()) => { Ok(nue_sus) }
                    Err(error) => { Err(ErrorMejorarSuscripcion::Archivo(error)) }
                }
            }
            Err(err) => { Err(err) }
        }
    }

    pub fn degradar_suscripcion(&mut self, user_id: u64) -> Result<TipoSuscripcion, ErrorDegradarSuscripcion> {
        let Some(user) = self.usuarios.get_mut(&user_id)
        else { return Err(ErrorDegradarSuscripcion::UsuarioInexistente) };

        let Some(sus) = user.obtener_suscripcion()
        else { return Err(ErrorDegradarSuscripcion::SinSuscripcion) };

        match sus.degradar() {
            Ok(nue_sus) => {
                match self.sobreescribir_archivo_usuarios(DEFAULT_FILE_NAME) {
                    Ok(()) => { Ok(nue_sus) }
                    Err(error) => {
                        // problema: el cambio está hecho localmente, no remotamente.
                        // posible solución: implementar una forma de "deshacer el cambio".
                        Err(ErrorDegradarSuscripcion::Archivo(error))
                    }
                }
            }
            Err(err) => { Err(err) }
        }
    }

    pub fn contabilizar_medios_de_pago(&self, contabilizar_inactivos: bool) -> HashMap<Discriminant<MedioDePago>, u32> {
        let mut contabilizador_mdp: HashMap<Discriminant<MedioDePago>, u32> = HashMap::new();

        self.usuarios.iter().for_each(|(_, usuario)| {

            if contabilizar_inactivos || usuario.suscripcion_activa.is_some() {
                if let Some(cant) = contabilizador_mdp.get_mut(&discriminant(&usuario.medio_de_pago)) {
                    *cant+= 1;
                } else {
                    contabilizador_mdp.insert(discriminant(&usuario.medio_de_pago), 1);
                }
            }

        });

        contabilizador_mdp
    }

    pub fn contabilizar_tipos_de_suscripcion(&self, contabilizar_inactivos: bool) -> HashMap<TipoSuscripcion, u32> {
        let mut contabilizador_ts: HashMap<TipoSuscripcion, u32> = HashMap::new();

        self.usuarios.iter().for_each(|(_, usuario)| {
            if contabilizar_inactivos || usuario.suscripcion_activa.is_some() {
                usuario.historial_suscripciones.iter().for_each(| (_, suscripcion) | {
                    if let Some(cant) = contabilizador_ts.get_mut(&suscripcion.tipo) {
                        *cant+= 1;
                    } else {
                        contabilizador_ts.insert(suscripcion.tipo, 1);
                    }
                });
            }
        });

        contabilizador_ts
    }

    // ➢ Saber el medio de pago que es más utilizado (opcion: por los usuarios sobre las suscripciones activas.)
    pub fn medio_de_pago_mas_utilizado(&self, contabilizar_inactivos: bool) -> Option<Discriminant<MedioDePago>> {
        //if self.usuarios.is_empty() { return None }

        let contabilizador_mdp = self.contabilizar_medios_de_pago(contabilizar_inactivos);

        contabilizador_mdp.iter()
            .max_by_key(|(_, c)| *c)
            .map(|(mdp, _)| mdp).copied()
    }

    // ➢ Saber cual es la suscripción más contratada (opcion: por los usuarios sobre las suscripciones activas.)
    pub fn suscripcion_mas_contratada(&self, contabilizar_inactivos: bool) -> Option<TipoSuscripcion> {
        if self.usuarios.is_empty() { return None }

        let contabilizador_ts = self.contabilizar_tipos_de_suscripcion(contabilizar_inactivos);

        contabilizador_ts.iter()
            .max_by_key(|(_, c)| *c)
            .map(|(ts, _)| ts).copied()
    }
}

#[cfg(test)]
mod tests {
    use std::collections::{HashMap};
    use std::fs;
    use std::fs::File;
    use std::mem::discriminant;
    use crate::structs::fecha::Fecha;
    use crate::structs::streamingrust::{ErrorDegradarSuscripcion, ErrorMejorarSuscripcion, ErrorRegistrarUsuario, StreamingRust, Usuarios, BASE_FOLDER, DEFAULT_FILE_NAME};
    use crate::structs::suscripcion::{F64Precio, MedioDePago, Suscripcion, TipoSuscripcion};
    use crate::structs::usuario::Usuario;

    const TEST_FILE_NAME: &str = "streamingrust_testfile";

    #[test]
    fn test_new() {
        let sr1 = StreamingRust::new(TEST_FILE_NAME, None).expect("Shouldn't be an error");
        let sr2 = StreamingRust::default();

        assert_eq!(sr1, sr2);
        let sr1 = StreamingRust::new(TEST_FILE_NAME, Some(Usuarios::default())).expect("Shouldn't be an error");
        assert_eq!(sr1, sr2);
    }

    // basic, classic(none), super(none), classic(none)
    // res tipo(false) -> basic
    // res tipo(true) -> classic

    // efectivo, mpago(none), mpago(none), debito(none)
    // res mdp(false) -> efectivo
    // res mdp(true) -> mpago
    fn streamingrust_mock(delete_file: bool) -> StreamingRust {
        if delete_file {
            delete_streamingrust_mock_json();
        }
        let sus_1 = Suscripcion {
            id: 0,
            tipo: TipoSuscripcion::Basic,
            costo_mensual: 0.0.as_precio(),
            fecha_inicio: Fecha::default(),
        };
        let sus_2 = Suscripcion {
            id: 1,
            tipo: TipoSuscripcion::Classic,
            costo_mensual: 0.0.as_precio(),
            fecha_inicio: Fecha::default(),
        };
        let sus_3 = Suscripcion {
            id: 2,
            tipo: TipoSuscripcion::Super,
            costo_mensual: 0.0.as_precio(),
            fecha_inicio: Fecha::default(),
        };
        let sus_4 = Suscripcion {
            id: 3,
            tipo: TipoSuscripcion::Classic,
            costo_mensual: 0.0.as_precio(),
            fecha_inicio: Fecha::default(),
        };

        let user_1 = Usuario {
            id: 0,
            email: "asd".to_string(),
            medio_de_pago: MedioDePago::Efectivo,
            suscripcion_activa: Some(sus_1.id),
            historial_suscripciones: HashMap::from([(sus_1.id, sus_1)]),
        };
        let user_2 = Usuario {
            id: 1,
            email: "asd".to_string(),
            medio_de_pago: MedioDePago::MercadoPago(1),
            suscripcion_activa: None,
            historial_suscripciones: HashMap::from([(sus_2.id, sus_2), (sus_3.id, sus_3)]),
        };
        let user_3 = Usuario {
            id: 2,
            email: "asd".to_string(),
            medio_de_pago: MedioDePago::Debito(123),
            suscripcion_activa: None,
            historial_suscripciones: HashMap::from([(sus_4.id, sus_4)]),
        };
        let user_4 = Usuario {
            id: 3,
            email: "asd".to_string(),
            medio_de_pago: MedioDePago::MercadoPago(1),
            suscripcion_activa: None,
            historial_suscripciones: HashMap::default(),
        };

        StreamingRust::new(
            TEST_FILE_NAME,
            Some(Usuarios::from([
                (user_1.id, user_1),
                (user_2.id, user_2),
                (user_3.id, user_3),
                (user_4.id, user_4)
            ])),
        ).expect("Shouldn't throw an error.")
    }

    fn delete_streamingrust_mock_json() -> bool {
        fs::remove_file(
            format!("{BASE_FOLDER}{TEST_FILE_NAME}")
        ).is_ok()
    }

    #[test]
    fn test_estadistica_sus_mdp() {
        let sr = streamingrust_mock(false);

        // basic, classic(none), super(none), classic(none)
        // res tipo(false) -> basic
        // res tipo(true) -> classic

        assert_eq!(sr.suscripcion_mas_contratada(false), Some(TipoSuscripcion::Basic));
        assert_eq!(sr.suscripcion_mas_contratada(true), Some(TipoSuscripcion::Classic));

        // efectivo, mpago(none), mpago(none), debito(none)
        // res mdp(false) -> efectivo
        // res mdp(true) -> mpago

        assert_eq!(sr.medio_de_pago_mas_utilizado(false), Some(discriminant(&MedioDePago::Efectivo)));
        assert_eq!(sr.medio_de_pago_mas_utilizado(true), Some(discriminant(&MedioDePago::MercadoPago(0))));
    }

    #[test]
    fn test_registrar_usuario() {
        let mut sr = streamingrust_mock(true);

        let user_1 = Usuario {
            id: 0,
            email: "asd".to_string(),
            medio_de_pago: MedioDePago::Efectivo,
            suscripcion_activa: None,
            historial_suscripciones: HashMap::new(),
        };

        let user_2 = Usuario {
            id: 13548,
            email: "asd".to_string(),
            medio_de_pago: MedioDePago::Efectivo,
            suscripcion_activa: None,
            historial_suscripciones: HashMap::new(),
        };

        let result_ins1 = sr.registrar_usuario(user_1);
        let result_ins2 = sr.registrar_usuario(user_2);

        assert_eq!(result_ins1, Err(ErrorRegistrarUsuario::UsuarioYaExiste), "El usuario ya existe");
        assert_eq!(result_ins2, Ok(()), "El usuario no existe");
    }

    #[test]
    fn test_mejorar_degradar_suscripcion() {
        let mut sr = streamingrust_mock(true);

        // user id 0 tiene suscripcion BASIC
        let res = sr.degradar_suscripcion(0);
        assert_eq!(res, Err(ErrorDegradarSuscripcion::SuscripcionMinima), "La suscripción ya es lo más baja posible");

        let res = sr.mejorar_suscripcion(0);
        assert_eq!(res, Ok(TipoSuscripcion::Classic), "Debe mejorar a Classic");

        let res = sr.mejorar_suscripcion(0);
        assert_eq!(res, Ok(TipoSuscripcion::Super), "Debe mejorar a Super");

        let res = sr.mejorar_suscripcion(0);
        assert_eq!(res, Err(ErrorMejorarSuscripcion::SuscripcionMaxima), "La suscripción ya es lo más alta posible");

        let res = sr.mejorar_suscripcion(13548);
        assert_eq!(res, Err(ErrorMejorarSuscripcion::UsuarioInexistente), "ID 13548 no existe");

        let res = sr.degradar_suscripcion(13548);
        assert_eq!(res, Err(ErrorDegradarSuscripcion::UsuarioInexistente), "ID 13548 no existe");

        let res = sr.mejorar_suscripcion(1);
        assert_eq!(res, Err(ErrorMejorarSuscripcion::SinSuscripcion), "ID 1 no tiene suscripción");

        let res = sr.degradar_suscripcion(1);
        assert_eq!(res, Err(ErrorDegradarSuscripcion::SinSuscripcion), "ID 1 no tiene suscripción");
    }

}