# tp-eng-software-2

 # Membros do grupo
- Gabriel Lopes Cancado Lira
- Vinicius Pinho Galvao
 
# Explicação do sistema

## Objetivo

Destacar arquivos “hotspots” que tendem a gerar custo de manutenção por reunirem **muita mudança**, **alta complexidade** e/ou **concentração de conhecimento** (poucos autores). Isso orienta refatorações, testes e disseminação de conhecimento na equipe.

### Como funciona

1. **Coleta (Git)**

   * Percorremos o histórico do repositório com a biblioteca **git2** para obter, por arquivo, **linhas adicionadas/removidas** (churn) e **autores** ao longo de um período (`--since/--until`). A git2 é uma biblioteca Rust que fornece bindings para libgit2, oferecendo acesso completo às funcionalidades do Git com excelente performance.
   * No momento, o projeto funciona apenas para git local.

2. **Complexidade**

   * Utilizamos a biblioteca **tokei** para análise de código e métricas de complexidade em múltiplas linguagens. A tokei é uma ferramenta rápida em Rust para contar linhas de código e fornecer estatísticas sobre a base de código.

3. **Cálculo do Score de Risco**

   * Normalizamos cada sinal em 0–1 e combinamos:

     * `churn_norm` = churn do arquivo normalizado
     * `complex_norm` = complexidade máxima normalizada
     * `authorship_penalty` = `1 / (1 + log1p(n_autores))`  *(menos autores ⇒ penalidade maior)*
     * **Score** = `100 * churn_norm * complex_norm * authorship_penalty`
   * Quanto maior o score, **mais crítico**.

4. **Saída**

   * **Tabela** no terminal (Top N com: caminho, churn, complexidade, nº de autores, score).
   * **Exportação** via `--json`, `--csv` e `--out report.md`.
   * **Filtros**: período (`--since`, `--until`), inclusão/exclusão de caminhos (`--include`, `--exclude`), `--top`.



# Explicação das ferramentas utilizadas

* Linguagem: **Rust**

* **git2**
  Biblioteca Rust que fornece bindings para libgit2, usada para percorrer commits, diffs e metadados do repositório Git com alta performance. Oferece acesso completo à API do Git de forma segura e eficiente.

* **tokei**
  Biblioteca rápida em Rust para contar linhas de código, comentários e análise de complexidade em múltiplas linguagens. Usada para obter métricas de manutenção e contextualizar o tamanho dos arquivos.

* **clap**
  Parser de linha de comando em Rust, rápido e com derive macros, para definir flags/subcomandos (`analyze`, `report`, etc.) de forma declarativa e type-safe.

* **serde / serde_json / csv**
  Bibliotecas para serialização e deserialização de dados. Usadas para exportar os relatórios em diferentes formatos (JSON, CSV).

* **tabled**
  Biblioteca para formatação de tabelas no terminal, usada para exibir os resultados do análise de forma legível e organizada.
---

## Como rodar o projeto

### Pré-requisitos

- **Rust** (versão 1.70 ou superior)
  - Instale através do [rustup](https://rustup.rs/): `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
- **Git** instalado no sistema

### Compilação

Para compilar o projeto em modo de desenvolvimento:

```bash
cargo build
```

Para compilar em modo de produção (otimizado):

```bash
cargo build --release
```

O binário será gerado em:
- Desenvolvimento: `target/debug/hotspot-analyzer`
- Produção: `target/release/hotspot-analyzer`

### Execução

Após compilar, você pode executar o analisador diretamente com:

```bash
cargo run -- [OPÇÕES]
```

Ou usando o binário compilado:

```bash
./target/debug/hotspot-analyzer [OPÇÕES]
# ou
./target/release/hotspot-analyzer [OPÇÕES]
```

**Exemplos de uso:**

```bash
# Analisar o repositório atual
cargo run -- /caminho/do/repositorio

# Analisar com filtro de período
cargo run -- /caminho/do/repositorio --since "2024-01-01" --until "2024-12-31"

# Exportar para JSON
cargo run -- /caminho/do/repositorio --json --out report.json

# Mostrar apenas os top 10 hotspots
cargo run -- /caminho/do/repositorio --top 10
```

### Testes

O projeto possui testes unitários para as principais funcionalidades.

**Rodar todos os testes:**

```bash
cargo test
```

**Rodar testes com output detalhado:**

```bash
cargo test -- --nocapture
```

**Rodar testes de um módulo específico:**

```bash
cargo test score           # Testa o módulo score
cargo test git_analyzer    # Testa o módulo git_analyzer
cargo test complexity      # Testa o módulo complexity
cargo test output          # Testa o módulo output
```

**Rodar um teste específico:**

```bash
cargo test test_calculate_scores_basic
```

**Verificar cobertura de testes:**

Os testes cobrem as funcionalidades principais:
- Cálculo de scores (`src/score.rs`)
- Análise de repositório Git (`src/git_analyzer.rs`)
- Análise de complexidade (`src/complexity.rs`)
- Formatação de saída (`src/output.rs`)

### CI/CD

O projeto está configurado com GitHub Actions para executar os testes automaticamente a cada push. O workflow:
1. Compila o projeto
2. Executa todos os testes unitários
3. Valida que o código está funcionando corretamente

Você pode ver o status dos testes no badge do GitHub Actions no topo deste README.

