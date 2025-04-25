
<!-- ALL-CONTRIBUTORS-BADGE:START - Do not remove or modify this section -->
[![All Contributors](https://img.shields.io/badge/all_contributors-1-orange.svg?style=flat-square)](#contributors-)
<!-- ALL-CONTRIBUTORS-BADGE:END -->
---

# JZFS

#### 🚧 Current Status: Incubating - JZFS is not ready for production usage. The API is still evolving and documentation is lacking.

JZFS is a distributed git storage service for the Rust programming language that prioritizes ease-of-use. It supports both Single Machine as well as some distributed environments, including Kubernetes and more. Note that JZFS does not hide the store; instead, JZFS exposes features based on the target distributed git storage service.

## Current Status and Roadmap

JZFS is still in the early development stages and is considered **incubating**. There is no commitment to ongoing maintenance or development. As the project evolves, this may change in the future. Therefore, we encourage you to explore, experiment, and contribute to JZFS, but do not attempt to use it in production.

The immediate next steps for the project are to fill obvious gaps, such as implementing error handling, removing panics throughout the codebase, supporting additional data types, and writing documentation. After that, development will be based on feedback and contributions.


### Added value
Our central use case is the DataHub(Like Github, buf for Data),which essentially consists of a Git version control for data and a Git collaboration for data.

In the JZFS DataHub model, each node maintains a copy of the files and all of the history of each file.
Thus, the node can make reads and writes of all past revisions offline.
When a node wants to share changes, it specifies a "remote".
The node can then send its state to that node, called "push", or receive new state from it, called "pull".
Writes are organized on branches.
Git is designed to compute differences between versions quickly.
Generally, Git relies on human action to share and merge changes.

Fault tolerance and trustless-ness are achieved via the separation of remotes from individual nodes. 
Each node in the network is maintaining it's own copy, its history and coordinating via one or many remotes. 
If a remote you trust gets corrupted, you have the ability to roll back to a previous good state and switch to a new remote. 
Even if you lose your copy you can rely on other nodes' copies to restore from.

This enables us to create transparency across internal and external data.

It forms the basis for a new way of practicing data exchange and contract design for distributed data sources.

It is crucial that data exchange works both within the company and in individually controllable data networks (data space).

JZFS data space consists of so-called “DataHubs”  like one or many remotes in Git, which are virtual data nodes for sharing data and building data networks.
A single DataHub manages various data connections and can join together with other hubs to form a network through targeted synchronization.
Based on data contracts mapped in the network, data can be released to other participants, enabling efficient data exchange.


![](./docs/jzfs-space.png)

JZFS offers git for data technology for exchanging data in data hub and data space. 
The added value is clear: simple, transparent data management combined with intuitive linking and sharing of data in decentralized networks – data space.
This enables secure, trustworthy data exchange across organizational boundaries.

JZFS's long-term goal is to build data ecosystems that enable new innovations.

### Deploy
```bash
docker compose up -d
```
### License

This project is licensed under the [MIT License].

[MIT License]: LICENSE

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in GitDataAi by you, Please refer to [Contribution Guidelines](Contributing.md), shall be licensed as MIT, without any additional terms or conditions.





<!-- ALL-CONTRIBUTORS-BADGE:START - Do not remove or modify this section -->
[![All Contributors](https://img.shields.io/badge/all_contributors-0-orange.svg?style=flat-square)](#contributors-)
<!-- ALL-CONTRIBUTORS-BADGE:END -->
## Contributors ✨

Thanks goes to these wonderful people ([emoji key](https://allcontributors.org/docs/en/emoji-key)):
<!-- ALL-CONTRIBUTORS-LIST:START - Do not remove or modify this section -->
<!-- prettier-ignore-start -->
<!-- markdownlint-disable -->
<table>
  <tbody>
    <tr>
      <td align="center" valign="top" width="14.28%"><a href="https://github.com/taoshengshi"><img src="https://avatars.githubusercontent.com/u/33315004?v=4?s=100" width="100px;" alt="taoshengshi"/><br /><sub><b>taoshengshi</b></sub></a><br /><a href="#content-taoshengshi" title="Content">🖋</a></td>
    </tr>
  </tbody>
</table>

<!-- markdownlint-restore -->
<!-- prettier-ignore-end -->

<!-- ALL-CONTRIBUTORS-LIST:END -->

<!-- ALL-CONTRIBUTORS-LIST:START - Do not remove or modify this section -->
<!-- prettier-ignore-start -->
<!-- markdownlint-disable -->
<!-- markdownlint-restore -->
<!-- prettier-ignore-end -->
<!-- ALL-CONTRIBUTORS-LIST:END -->

This project follows the [all-contributors](https://github.com/all-contributors/all-contributors) specification. Contributions of any kind welcome!