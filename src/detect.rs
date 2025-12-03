use std::net::TcpStream;
use std::process::Command;

pub enum InfoServico {
    ServicoInexistente,
    PostgreSQL,
    MySQL,
    MariaDB
}
pub struct ResultadoPorta {
    pub porta: u16,
    pub info: InfoServico,
}

fn porta_aberta(porta: u16) -> bool {
    TcpStream::connect(("127.0.0.1", porta)).is_ok()
}

pub fn testar_postgres(porta: u16) -> bool {
    // SELECT 1 é neutro, só testa se aceita comando
    let args = [
        "-h", "12.0.0.1",
        "-p", &porta.to_string(),
        "-U", "postgres",
        "-t",
        "-c", "SELECT 1"
    ];

    let output = Command::new("psql").args(&args).env("PGPASSWORD", "").output();

    match output {
        Ok(o) => o.status.success(),
        Err(_) => false,
    }
}

pub fn buscar_portas_automatico() -> Vec<ResultadoPorta> {
    let portas = [5432, 6433, 3306];
    let mut resultados = Vec::new();

    for porta in portas{
        if porta_aberta(porta){

            let mut r = ResultadoPorta { 
                porta,
                info: InfoServico::ServicoInexistente,
            };

            if porta == 5432 && testar_postgres(porta) {
                r.info = InfoServico::PostgreSQL;
            }

            resultados.push(r);
        }
    }

    resultados

}

// struct BancoDetectado {
//     tipo: BancoTipo,
//     porta: u16,
// }

// enum BancoTipo {
//     Postgres,
//     MySQL,
//     MariaDB,
// }

