mod detect;
mod auth;

use crate::detect::{InfoServico, buscar_portas_automatico};
use crate::auth::{*};


fn main(){
    let resultados = buscar_portas_automatico();

    if (resultados.is_empty()) {
        println!("Nenhuma porta de banco de dados está aberta ou detectável automaticamente, procure por uma porta manual ou verifique se o processo está rodando");
        return;
    } 

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