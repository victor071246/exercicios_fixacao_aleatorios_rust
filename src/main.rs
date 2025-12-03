mod detect;

use crate::detect::{InfoServico, buscar_portas_automatico};


fn main(){
    let resultados = buscar_portas_automatico();

    for r in resultados {
        match r.info {
            InfoServico::ServicoInexistente => 
                println!("Porta {} -> aberta mas serviço inexistente ou inacessível", r.porta),
            
            InfoServico::PostgreSQL => 
                println!("Porta {} -> PostgreSQL detectado", r.porta),

            InfoServico::MySQL => 
                println!("Porta {} -> MySQL detectado", r.porta),
            
            InfoServico::MariaDB => 
                println!("Porta {} -> MariaDB detectado", r.porta)
        }
    }
}