# Cross-Market Gamma Engine

A production-oriented research framework for building **high-frequency options-flow analytics** across crypto and international derivatives markets.

This repository focuses on:
- Real-time implied volatility inversion
- Greeks and gamma exposure aggregation
- Dealer hedge-pressure estimation
- Cross-market signal generation
- Low-latency execution integration
- Replay-driven validation

The architecture combines:
- **Rust** for market-data ingestion, normalization, pricing, IV surfaces, and signal computation
- **C++** for ultra-low-latency execution, lock-free transport, and hot-path benchmarking

---

## Why this exists

Most trading systems treat options analytics as a dashboard or an end-of-day research layer.

This project treats options state as **streaming market structure**.

The core idea is simple:

> If gamma, implied volatility, and hedge pressure change faster than your execution horizon, those quantities belong inside the event loop.

This engine is designed for:
- intraday futures and perpetual execution
- gamma wall detection
- gamma flip detection
- negative gamma acceleration studies
- positive gamma pinning studies
- expiry-window hedge-flow analysis
- volatility shock and skew dislocation monitoring

---

## Architecture

```mermaid
flowchart LR
    A[Market Connectivity\nExchange Gateways\nBroker Feeds\nInternal Buses] --> B[Instrument Normalizer]
    B --> C[Event Bus\nLock-free / Async]
    C --> D[Fair Pricing]
    D --> E[IV Engine]
    E --> F[Greeks Engine]
    F --> G[GEX Engine]
    G --> H[Surface Engine]
    H --> I[Signal Engine]
    I --> J[Execution Bridge]
    C --> K[Replay + Storage]
    J --> L[C++ Execution Engine]
    L --> M[Risk Engine]
