# Revisão — conexão automática no Windows

Revisão estática sem achados técnicos pendentes.

- Inicialização por usuário, após login; entrada Run sem credenciais.
- Opção desligada por padrão; exige rede sem MFA, postura ou local de serviço.
- Política validada ao salvar e ao iniciar; comando de conexão normal também recusa MFA.
- Erros do registro e da gravação das preferências são propagados; falha de gravação desfaz a mudança do registro.
- Desconexão manual ou troca da preferência cancela novas tentativas de inicialização.
- Cadastro sem MFA segue a regra informada pelo servidor; não altera tokens nem fatores existentes.

Validação: interface compilada, Biome aprovado nos arquivos alterados e 23 testes do núcleo aprovados.
Conferência da interface com respostas simuladas: ativar/desativar, reabrir mantendo a escolha, falha na gravação, rede com MFA e ocultação no Linux.
O fluxo de cadastro foi exercitado sem MFA (Welcome → Password → Finish) e com MFA (etapa de configuração preservada).
Registro de inicialização: teste nativo do Windows aprovado.
Testes Rust e instaladores: em validação no GitHub Actions; conferir a execução da branch antes de publicar.
O compilador mínimo foi corrigido para 1.96, exigido pelas dependências já fixadas.
Teste com reinício de um computador Windows real continua necessário.
