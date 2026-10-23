# Mediaware VPN — cliente desktop

Cliente de VPN da Mediaware para Windows e Linux. É um fork do
[cliente desktop do Defguard](https://github.com/DefGuard/client) (versão 2.1.2),
distribuído sob a mesma licença, **AGPL-3.0**. O código-fonte completo desta versão
está neste repositório.

Funciona com um servidor [Defguard](https://github.com/DefGuard/defguard) 2.1:
cadastro do aparelho por token, MFA na conexão (TOTP, e-mail, OIDC) e várias
instâncias e locais no mesmo app.

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
