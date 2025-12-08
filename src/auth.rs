use std::process::Command;
use crate::detect::InfoServico;

pub struct Credenciais {
    pub usuario: String,
    pub senha: String,
}

pub fn autenticar(servico: &InfoServico, porta: u16, cres: &Credenciais) -> bool {
    match servico {
        InfoServico::PostgreSQL =>
    }
}

fn autenticar_postgres(porta: u16, creds: &Credenciais) -> bool {
    let args = [
        "-h", "127.0.0.1",
        "-p", &porta.to_string(),
        "-U", &creds.usuario,
        "-t",
        "-c", "SELECT 1",
    ];

    let output = Command::new("psql")
        .args(&args)
        .env("PGPASSWORD", &creds.senha)
        .output();

    match output {
        Ok(o) => o.status.success(),
        Err(_) => false,
    }
}