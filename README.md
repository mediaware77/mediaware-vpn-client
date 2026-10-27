# PMCG VPN · by Mediaware

Edição da Mediaware VPN para a Prefeitura Municipal de Campina Grande, para Windows e Linux. É um fork do
[cliente desktop do Defguard](https://github.com/DefGuard/client) (versão 2.1.2),
distribuído sob a mesma licença, **AGPL-3.0**. O código-fonte completo desta versão
está neste repositório.

Funciona com um servidor [Defguard](https://github.com/DefGuard/defguard) 2.1:
cadastro do aparelho por token, MFA na conexão (TOTP, e-mail, OIDC) e várias
instâncias e locais no mesmo app.

## O que muda em relação ao original

- **Marca:** brasão da prefeitura, nome **PMCG VPN** e crédito **by Mediaware** no app e no instalador. A origem do brasão está em [branding/README.md](branding/README.md). Os nomes
  internos (`defguard-client`, `defguard-service`, o esquema de link `defguard://`)
  ficaram iguais para manter compatibilidade com o servidor e com os instaladores.
- **Sem código enterprise.** A pasta `enterprise/` do projeto original tem licença
  própria, que não permite redistribuir nem criar produto derivado, e foi retirada.
  Ficam de fora: checagem de postura do aparelho, locais de serviço (túnel antes do
  login no Windows), provisionamento automático por AD/Entra ID e a sincronização
  automática de configuração. A atualização de configuração por token continua, com
  implementação própria em `src-tauri/src/instance_update.rs`.
- **Protocolo:** os arquivos `.proto` abertos ficam em `src-tauri/proto/`, sem a parte
  enterprise (o campo de postura virou `reserved`).
- **Atualização:** o app não consulta o servidor de atualizações do Defguard.
- **macOS:** não é compilado. A versão 2.x conecta por uma Network Extension, que
  exige conta Apple Developer com essa permissão.

## Conexão automática no Windows

Em **Settings → Conectar ao entrar no Windows**, selecione a rede. O app inicia
em segundo plano quando esse usuário entra no Windows, sem abrir a janela ou
solicitar código. Não conecta antes do login. A opção começa desligada e pode
ser desativada selecionando **Desligado**; uma desconexão manual permanece até
o próximo login. Não há cadastro de tarefa privilegiada nem cópia de senha ou
segredo MFA: o app reutiliza o aparelho já cadastrado.

Somente redes regulares sem exigência de MFA ou postura podem ser selecionadas.
O app verifica a regra novamente no início; não contorna uma exigência do servidor.
Se o serviço ainda não estiver pronto, tenta novamente por até dez minutos.
A inicialização usa uma entrada **PMCGVPN** em `HKCU\Software\Microsoft\Windows\CurrentVersion\Run`.
Quem controlar a conta Windows pode usar esse acesso. As chaves permanecem no
armazenamento local que o cliente já utiliza.

O administrador precisa configurar a rede PMCG com MFA desativado. Quando o
servidor informa `mfa_required=false`, o cadastro já permite concluir sem
configurar autenticador. MFA previamente cadastrado não é apagado; continua
podendo proteger o painel. Aparelhos antigos devem atualizar a configuração
pelo token de **Update Instance** para receber a nova regra.

## Instalação e compatibilidade

Esta edição **substitui a Mediaware VPN** no computador. O identificador dos dados
e o código de atualização MSI foram mantidos para preservar o cadastro existente.
Os dois aplicativos não podem ser instalados lado a lado: compartilham os
binários, o serviço de conexão e o protocolo `defguard://`.
No Linux o pacote declara conflito e substituição de `mediaware-vpn`; use o
gerenciador de pacotes para a troca. Não instalar junto com o cliente Defguard.
A preservação dos cadastros e a conexão precisam ser conferidas em um computador
com a versão anterior instalada antes de distribuir para os usuários.

Os instaladores não têm assinatura de código. O Windows pode exibir aviso do
SmartScreen; com Smart App Control ativo, a instalação pode ser bloqueada.

## Build

O workflow `.github/workflows/build.yaml` gera, a cada push no ramo `codex/pmcg-vpn`,
um `.deb` (Linux) e um `.msi` (Windows), ainda sem assinatura de código.

Para compilar localmente, siga os pré-requisitos do [Tauri 2](https://tauri.app/start/prerequisites/)
e instale `protobuf-compiler`. Depois:

```sh
cd new-ui && pnpm install && pnpm build
cd ../src-tauri && SQLX_OFFLINE=1 cargo build --release
```

## Licença

AGPL-3.0 — veja [LICENSE.md](LICENSE.md). Copyright do código original: defguard sp. z o.o.
Modificações: Mediaware. "Defguard" e "WireGuard" são marcas de seus titulares; este
projeto não é afiliado a eles.
