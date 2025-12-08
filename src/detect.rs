use std::net::TcpStream;
use std::process::{Command, Output};

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
        "-h", "127.0.0.1",
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

pub fn testar_mysql(porta: u16) -> bool {
    let args = [
        "-h", "127.0.0.1",
        "-P", &porta.to_string(),
        "-u", "root",
        "-e", "SELECT 1",
    ];

    let output = Command::new("mysql").args(&args).output();

    match output {
        Ok(o) => o.status.success(),
        Err(_) => false, 
    }
}

pub fn testar_mariadb(porta: u16) -> bool {
    let args = [
        "-h", "127.0.0.1",
        "-P", &porta.to_string(),
        "-u", "root",
        "-e", "SELECT 1",
  ];

  let output = Command::new("mariadb")
    .args(&args)
    .output();

  match output {
    Ok(o ) => o.status.success(),
    Err(_) => false,
  }
    
}

pub fn buscar_portas_automatico() -> Vec<ResultadoPorta> {
    let portas = [5432, 5433, 3306];
    let mut resultados = Vec::new();

    for porta in portas{
        if porta_aberta(porta){
        
            let mut r = ResultadoPorta { 
                porta,
                info: InfoServico::ServicoInexistente,
            };

            if porta == 5432 || porta == 5433  && testar_postgres(porta) {
                r.info = InfoServico::PostgreSQL;
            }

            if porta == 3306 {
                if 
                testar_mariadb(porta) {
                    r.info = InfoServico::MariaDB;    
                }
                else if testar_mysql(porta) {
                    r.info = InfoServico::MySQL;
                } 
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

