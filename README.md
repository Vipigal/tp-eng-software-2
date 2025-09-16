# tp-eng-software-2

 # Membros do grupo
- Gabriel Lopes Cancado Lira
- Vinicius Pinho Galvao
 
# Explicação do sistema

## Objetivo

Destacar arquivos “hotspots” que tendem a gerar custo de manutenção por reunirem **muita mudança**, **alta complexidade** e/ou **concentração de conhecimento** (poucos autores). Isso orienta refatorações, testes e disseminação de conhecimento na equipe.

### Como funciona

1. **Coleta (Git)**

   * Percorremos o histórico do repositório com uma lib Git em Rust para obter, por arquivo, **linhas adicionadas/removidas** (churn) e **autores** ao longo de um período (`--since/--until`). Usaremos o **gix (gitoxide)** como backbone Git em Rust, que fornece abstração de `Repository` com foco em performance. ([Docs.rs][1])
   * (Talvez) Possamos aceitar uma URL e clonar/localizar branch/tag. A ideia é fazer com o git local.

2. **Complexidade**

   * Rodamos **rust-code-analysis** para extrair **métricas de manutenção** baseadas em **tree-sitter** (ex.: complexidade ciclomática por arquivo). É multilinguagem e projetado justamente para análise de código. ([mozilla.github.io][2])

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

* **gix (gitoxide)**
  Biblioteca Git em Rust usada para percorrer commits, diffs e metadados com boa performance, oferecendo o `Repository` como hub de funcionalidades.

* **rust-code-analysis**
  Biblioteca em Rust (baseada em **tree-sitter**) para **extrair métricas de manutenção** e **complexidade** em múltiplas linguagens. Usamos principalmente a **complexidade ciclomática por arquivo** como sinal.

* **tokei** *(opcional)*
  Para contar **LOC** (linhas de código, comentários, blanks) e enriquecer os relatórios — útil para contextualizar churn e complexidade em arquivos muito grandes/pequenos.

* **clap**
  Parser de linha de comando em Rust, rápido e com derive macros, para definirmos flags/subcomandos (`analyze`, `report`, etc.) de forma declarativa. 

