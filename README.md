# 📈 quantx-engine

**quantx-engine** est un moteur d’analyse boursière modulaire écrit en **Rust**, conçu pour agréger différents signaux financiers (techniques, fondamentaux, sentiment, macro...) afin de produire une évaluation synthétique d’une action.

Le projet vise à fournir une base performante et extensible pour explorer, scorer et surveiller des actions sur les marchés financiers à court et moyen terme.

---

## 🔧 Modules principaux

### 🧩 `technical`
Analyse technique via des indicateurs comme **RSI**, **MACD**, **Moyennes mobiles**, etc.

### 📊 `fundamentals`
Analyse des ratios financiers de l’entreprise (ex. : **PER**, **ROE**, **BPA**, **marge brute**, etc.).

### 🗞️ `sentiment`
Analyse du **sentiment de marché** basé sur les actualités ou les réseaux sociaux (via NLP ou API externes).

### 🏛️ `macro`
Suivi des indicateurs économiques globaux : **taux d’intérêt**, **inflation**, **PIB**, etc.

### 🧑‍💼 `insiders`
Analyse de l’activité des **initiés** (achats/ventes des dirigeants déclarés).

### 📆 `calendar`
Récupère les événements importants : **résultats financiers à venir**, **dividendes**, **OPA**, etc.

### 📈 `scoring`
Fusionne les résultats des modules et génère un **score global** d’évaluation (Buy / Hold / Sell).

### 💃 `luxure Index`
Analyse les tendances du marché pour les activités de luxe et des strip club pour déterminer si il y a un penchant pour la luxure = forte consomation de produits hors marché de première nécessiter

---

## ✅ Objectifs

- Orchestration des modules via un moteur central
- Sortie **JSON unifiée** pour chaque action analysée
- Possibilité d’extension future (Web API, dashboard, trading bot...)

---

## 🚀 Lancer le projet

```bash
cargo run
