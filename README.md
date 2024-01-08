⚠️ **This project is still in development and not ready for production use.** ⚠️

# Contower: An All-in-One Client for Ethereum

![Contower Banner](assets/repo_banner.png)

[![build](https://github.com/SanderLoman/ConTower/actions/workflows/build.yml/badge.svg)](https://github.com/SanderLoman/ConTower/actions/workflows/build.yml)
[![tests](https://github.com/SanderLoman/ConTower/actions/workflows/tests.yml/badge.svg)](https://github.com/SanderLoman/ConTower/actions/workflows/tests.yml)
[![Codecov](https://img.shields.io/codecov/c/github/SanderLoman/ConTower?token=JT1850HR9J)](https://app.codecov.io/gh/SanderLoman/ConTower)
[![License](https://img.shields.io/badge/License-Apache_2.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)

## Overview

Contower is our latest development for the Ethereum network, designed to bring flexibility and efficiency to blockchain operations. It uniquely integrates execution and consensus clients into a versatile relay/proxy networking client. Developed with Rust, Contower stands out for its optional reliance on a traditional database, allowing for streamlined and adaptable operations.

This client serves as an adaptable intermediary, capable of either facilitating relay communication and keeping track of the latest blockchain traffic with transient caching or functioning as a full node with its own database. Users have the choice to run Contower as a lean relay/proxy client, a complete node client incorporating execution and consensus mechanisms, or even both simultaneously. This flexibility ensures that Contower can meet various user needs, enhancing network functionality, efficiency, and decentralization, tailored to individual preferences and requirements.

## Objectives

-   **Unified Client Architecture:** Contower integrates execution and consensus mechanisms into a cohesive framework, with additional relay functionality. Each component can be toggled independently, allowing users to choose between running a merged execution/consensus client, activating relay capabilities, or utilizing both features simultaneously. This design provides flexibility and paves the way for future innovations in the Ethereum ecosystem.

-   **Performance Enhancement:** Contower's design, free from the constraints of a traditional database, ensures fast processing and efficient resource use.

-   **Client Diversity:** Introducing Contower to the Ethereum network adds to its robustness and guards against systemic risks. Diverse client types contribute to a stronger network.

-   **Broad EVM Chain Support:** Designed to be compatible with a variety of EVM chains, Contower aims to be versatile across different blockchain networks, increasing its utility and applicability.

-   **Configurability and Accessibility:** Contower caters to a range of users, from large-scale node operators to individual enthusiasts. Its configurable nature ensures it meets various operational needs, whether optimizing for speed or adapting to limited hardware.

## Community and Support

Engage with our community for discussions, support, and collaboration.

-   [GitHub Issues](https://github.com/SanderLoman/rust-p2p/issues)
-   [Discord](https://discord.gg/vHWpWsjCqx)

## Documentation

See the [Book](https://nodura.github.io/Contower/) for more information.

## Contributing

Eager to witness your contributions and innovations!

See [CONTRIBUTING.md](CONTRIBUTING.md) for more information.

## Security

See [SECURITY.md](SECURITY.md) for more information.

## Gratitudes

As we continue to develop and enhance our project, we extend our sincere gratitude to several key players in the Ethereum ecosystem whose contributions have been invaluable.

-   [Lighthouse](https://github.com/sigp/lighthouse): A special thank you to the Lighthouse team for their remarkable work in the Ethereum space. Their efforts in creating a secure, high-performance Eth2 client have not only pushed the boundaries of innovation but also provided us with insights and inspiration for our own development. We deeply appreciate their commitment to the Ethereum community and their ongoing contributions.

-   [Reth](https://github.com/paradigmxyz/reth): Our heartfelt thanks go out to the Reth team for their pioneering work in Ethereum protocol implementation. Their inventive approaches and solutions have significantly influenced our project, offering us valuable perspectives and tools to enhance our own development. We are grateful for their dedication to advancing the Ethereum ecosystem.

-   The Community: We would also like to extend our thanks to all the individuals and teams who have directly or indirectly contributed to our project. Your insights, feedback, and support have been instrumental in our journey, and we look forward to continuing this collaborative effort. Your contributions are a testament to the power of community-driven development in the blockchain space.

To all of you, thank you for your invaluable contributions and for being an integral part of our journey.
