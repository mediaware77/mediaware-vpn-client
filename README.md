# Clientes VPN — Mediaware e PMCG

Edições **Mediaware VPN** e **PMCG VPN · by Mediaware** para Windows e Linux.
O projeto é um fork do
[cliente desktop do Defguard](https://github.com/DefGuard/client) (versão 2.1.2),
distribuído sob a mesma licença, **AGPL-3.0**. O código-fonte completo desta versão
está neste repositório.

Funciona com um servidor [Defguard](https://github.com/DefGuard/defguard) 2.1:
cadastro do aparelho por token, MFA na conexão (TOTP, e-mail, OIDC) e várias
instâncias e locais no mesmo app.

## Downloads

Para a Prefeitura Municipal de Campina Grande, use **PMCG VPN**.

| Edição | Versão | Windows x64 | Linux amd64 | Publicação |
|---|---|---|---|---|
| **PMCG VPN · by Mediaware** | **2.1.5** | [Baixar MSI](https://github.com/mediaware77/mediaware-vpn-client/releases/download/pmcg-v2.1.5/PMCG.VPN.by.Mediaware._2.1.5_x64_en-US.msi) | [Baixar DEB](https://github.com/mediaware77/mediaware-vpn-client/releases/download/pmcg-v2.1.5/PMCG.VPN.by.Mediaware._2.1.5_amd64.deb) | [Notas e SHA-256](https://github.com/mediaware77/mediaware-vpn-client/releases/tag/pmcg-v2.1.5) |
| Mediaware VPN | 2.1.2 | [Baixar MSI](https://github.com/mediaware77/mediaware-vpn-client/releases/download/mediaware-v2.1.2/Mediaware.VPN_2.1.2_x64_en-US.msi) | [Baixar DEB](https://github.com/mediaware77/mediaware-vpn-client/releases/download/mediaware-v2.1.2/mediaware-vpn_2.1.2_amd64.deb) | [Notas e SHA-256](https://github.com/mediaware77/mediaware-vpn-client/releases/tag/mediaware-v2.1.2) |

O destaque **Latest** do GitHub aponta para a edição Mediaware. A publicação da
PMCG tem a tag **pmcg-v2.1.5** e os links próprios acima.

Código-fonte: [PMCG — ramo codex/pmcg-vpn](https://github.com/mediaware77/mediaware-vpn-client/tree/codex/pmcg-vpn)
e [Mediaware — ramo mediaware](https://github.com/mediaware77/mediaware-vpn-client/tree/mediaware).
O repositório privado `vpn-pmcg` guarda a infraestrutura dos servidores; os
instaladores do aplicativo são publicados neste repositório público.

## O que muda em relação ao original

- **Marca:** nome, ícones, banners e textos da interface são da Mediaware. Os nomes
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

## Build

O workflow `.github/workflows/build.yaml` gera, a cada push no ramo `mediaware`,
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
