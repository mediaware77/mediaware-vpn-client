# Revisão — conexão automática no Windows

Revisão estática sem achados técnicos pendentes.

- Inicialização por usuário, após login; entrada Run sem credenciais.
- Opção desligada por padrão; exige rede sem MFA, postura ou local de serviço.
- Política validada ao salvar e ao iniciar; comando de conexão normal também recusa MFA.
- Erros do registro e da gravação das preferências são propagados; falha de gravação desfaz a mudança do registro.
- Desconexão manual ou troca da preferência cancela novas tentativas de inicialização.
- Cadastro sem MFA segue a regra informada pelo servidor; não altera tokens nem fatores existentes.

Validação inicial: interface compilada e arquivos alterados aprovados no Biome.
Testes Rust e instaladores: em validação no GitHub Actions; conferir a execução da branch antes de publicar.
O compilador mínimo foi corrigido para 1.96, exigido pelas dependências já fixadas.
Teste com reinício de um computador Windows real continua necessário.
