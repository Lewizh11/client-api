# MBS TUI
## TUI simples para o consumo da [API](https://github.com/Lewizh11/api_rest_ads)

[![Chat on Telegram](https://img.shields.io/badge/Telegram-ShuseiKagari-blue)](https://t.me/ShuseiKagari)
### Pré-requisitos

Você irá precisar destes softwares instalado em sua máquina para rodar localmente:
- [Git](https://git-scm.com)
- [Rust Language](https://www.rust-lang.org)


## "Instalação"

#### Primeiro, instale o ```build-essential``` em sua máquina

#### Hospede a  [API](https://github.com/Lewizh11/api_rest_ads), após vá no arquivo: src/reqs/mod.rs , linha 25 e substitua pela a URL de sua [API](https://github.com/Lewizh11/api_rest_ads)

#### Clone o repositório
```bash
$ git clone https://github.com/Lewizh11/client-api.git && cd client-api
```

### Compilando
```bash
$ cargo build
# Gerará um binário nomeado "client" que estará em target/debug
# caso queira rodar diretamente, use cargo run
```
