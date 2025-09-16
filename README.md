# tp-eng-software-2

 # Membros do grupo
- Gabriel Lopes Cancado Lira
- Vinicius Pinho Galvao
 
# Explicação do sistema

## Objetivo

Identificar rapidamente **problemas de manutenção** em um repositório ao destacar **arquivos “hotspots”**: partes do código que mudam muito, são complexas e/ou dependem de um número pequeno de autores. Esses pontos tendem a acumular dívidas técnicas, bugs e custos de evolução.

## Como funciona

1. **Coleta (Git local)**
   Lemos o histórico do repositório com *PyDriller* para calcular métricas em um intervalo (ex.: últimos 90 dias, ou `--since <data>`).
   Extraímos dados como **autores por arquivo** e contagem de commits por autor/arquivo.

2. **Complexidade**
   Rodamos o *lizard* nos arquivos do código-fonte (ex.: `src/`, `app/`) para obter **complexidade ciclomática** (média e máximo por arquivo) e tamanho (LOC).

3. **Cálculo do Score de Risco**
   Normalizamos os sinais e calculamos um score simples:

   * `churn_norm` = churn do arquivo normalizado (0–1)
   * `complex_norm` = complexidade máxima normalizada (0–1)
   * `authorship_penalty` = `1 / (1 + log1p(n_autores))`  (menos autores ⇒ penalidade maior)
   * **Score** = `100 * churn_norm * complex_norm * authorship_penalty`
     Quanto maior, **mais crítico**.

4. **Saída**
   * **Tabela no terminal** com Top N hotspots (arquivo, churn, complexidade, nº de autores, score).
   * **Exportação opcional** (`--out report.md` | `--csv` | `--json`) para usar em relatórios/CI.

---

# Explicação das ferramentas utilizadas

* **Rust**
  Usado para **mineração de repositórios Git**. Facilita percorrer commits, arquivos alterados e metadados (autor, data, linhas adicionadas/removidas). É a base do nosso **cálculo de churn** e **autoria por arquivo**.

* **Clap**
  Ferramenta de **análise de complexidade**. Roda direto nos arquivos do projeto e retorna **complexidade ciclomática** (por função e por arquivo), além de **LOC**. Usamos a **complexidade máxima** por arquivo como sinal de risco.

* **Git** 
  O projeto opera sobre um repositório Git local. 

* **Typer**
  Framework para **CLI em Python**. Permite construir uma interface `msr-hotspots analyze --repo . --since 2025-06-01 --top 15 --out report.md` com ajuda embutida e validação de parâmetros, mantendo o código enxuto.

