
# JiaoziFS (JZFS)
A version control file system for data centric applications & teams.

<p align="left">
  <a href="https://codecov.io/gh/jiaozifs/jiaozifs"><img src="https://codecov.io/gh/gitdataai/jiaozifs/branch/main/graph/badge.svg" /></a>
  <a href="https://goreportcard.com/report/github.com/jiaozifs/jiaozifs"><img src="https://goreportcard.com/badge/github.com/gitdataai/jiaozifs" /></a>  
  <a href=""><img src="https://img.shields.io/badge/golang-%3E%3D1.22.0-blue.svg" /></a>
  <br/>
</p>

<a href="https://github.com/GitDataAI/jiaozifs"><img src="https://github.com/GitDataAI/jiaozifs/blob/main/docs/logo/jiaozifs.png?raw=true" width="100" /></a>

----
### What is JiaoziFS?
JiaoziFS is an industry-leading **Data-Centric Version Control** File System, helps ensure Responsible AI Engineering by improving **Data Versioning**, **Provenance**, and **Reproducibility**.

Note:
* The name Jiaozi pays tribute to the world's earliest paper money: [Song Dynasty Jiaozi](https://en.wikipedia.org/wiki/Jiaozi_(currency)).
* JiaoziFS is yet another implementation of [IPFS (InterPlanetary File System)](https://ipfs.tech/) as JiaoziFS will be compatible with the [implementation requirements](https://specs.ipfs.tech/architecture/principles/#ipfs-implementation-requirements) of IPFS.
* As a filesystem of data versioning at scale, although JiaoziFS is built for machine learning, It has a wide range of use scenarios (refer A Universe of Uses) and can be seamlessly integrated into all your data stack.

Data-centric AI is about the practice of iterating and collaborating on data, used to build AI systems, programmatically. Machine learning pioneer Andrew Ng [argues that focusing on the quality of data fueling AI systems will help unlock its full power](https://youtu.be/TU6u_T-s68Y).

----
### Why JiaoziFS?
In production systems with machine learning components, updates and experiments are frequent. New updates to models(data products) may be released every day or every few minutes, and different users may see the results of different models as part of A/B experiments or canary releases.

* **Version Everything**: Data scientists are often criticized for being less disciplined with versioning their experiments(versioning of data, pipeline, code, and models), especially when using computational notebooks.
* **Track Data Provenance**: This applies to all processing steps in an AI/ML pipeline, including data collection/acquisition, data merging, data cleaning, feature extraction, learning, or deployment.
* **Reproducibility**: A final question of AI/ML that is often relevant for debugging, audits, and also science more broadly is to what degree data, models, and decisions can be reproduced.

----
### A Universe of Uses
JiaoziFS's versatility shines across different industries – making it the multi-purpose tool for the **data centric applications and teams**.

* **Enterprise DataHub & Data Collaboration**: Depending on your operating scale, you may even be managing multiple team members, who may be spread across different locations. JiaoziFS enable Collaborative Datasets Version Management at Scale,Share & collaborate easily: Instantly share insights and co-edit with your team.
* **DataOps & Data Products & Data Mesh**: Augmenting Enterprise Data Development and Operations,JiaoziFS ensures Responsible DataOps/AIOps/MLOps by improving Data Versioning, Provenance, and Reproducibility. JiaoziFS makes a fusion of data science and product development and allows data to be containerized into shareable, tradeable, and trackable assets(data products or data NFTs). Versioning data products in a maturing Data Mesh environment via standard processes, data consumers can be informed about both breaking and non-breaking changes in a data product, as well as retirement of data products.
* **Industrial Digital Twin**: Developing digital twins for manufacturing involves managing tons of large files and multiple iterations of a project. All of the data collected and created in the digital twin process (and there is a lot of it) needs to be managed carefully. JiaoziFS allows you to manage changes to files over time and store these modifications in a database.
* **Data Lake Management**: Data lakes are dynamic.   New files and new versions of ex- isting files enter the lake at the ingestion stage.   Additionally, extractors can evolve over time and generate new versions of raw data.   As a result, data lake versioning is a cross-cutting concern across all stages of a data lake.   Of course vanilla dis- tributed file systems are not adequate for versioning-related operations.   For example, simply storing all versions may be too costly for large datasets, and without a good version manager, just using filenames to track versions can be error-prone.   In a data lake, for which there are usually many users, it is even more important to clearly maintain correct versions being used and evolving across different users.   Furthermore, as the number of versions increases, efficiently and cost-effectively providing storage and retrieval of versions is going to be an important feature of a successful data lake system.
----
### Specification

[JiaoziFS Specification](https://github.com/GitDataAI/Specification/blob/main/JiaoziFS)

----
### Basic Build And Usage

#### Requirement

1. To build JiaoziFS, you need a working installation of   [Go 1.22.0 or higher](https://golang.org/dl/)
2. JiaoziFS use postgres to store running data, you can install at  [postgres install installation guide](https://www.postgresql.org/docs/current/installation.html)

#### Build And Running

1. clone and build
```bash
git clone https://github.com/GitDataAI/jiaozifs.git
cd jiaozifs
make build
```

After following the above steps, you should be able to see an executable file named "jzfs."

2. init program and running
```bash
./jzfs init  --db postgres://<username>:<password>@localhost:5432/jiaozifs?sslmode=disable
./jzfs daemon
```

#### run with docker

```bash
docker run -v <data>:/app -p 34913:34913 gitdatateam/jzfs:latest  --db "postgres://<user>:<password>@192.168.1.16:5432/jiaozifs?sslmode=disable" --bs_path /app/data --listen http://0.0.0.0:34913 --config /app/config.toml
```

----
### Cloud

[Try without installing](https://cloud.jiaozifs.com)

----
### Contributors

<a href="https://github.com/hunjixin" target="_blank"><img src="https://avatars.githubusercontent.com/u/41407352?v=4" width="5%" height="5%"/> </a>
<a href="https://github.com/Brownjy" target="_blank"><img src="https://avatars.githubusercontent.com/u/54040689?v=4" width="5%" height="5%"/> </a>
<a href="https://github.com/TsumikiQAQ" target="_blank"><img src="https://avatars.githubusercontent.com/u/116857998?v=4" width="5%" height="5%"/> </a>
<a href="https://github.com/taoshengshi" target="_blank"><img src="https://avatars.githubusercontent.com/u/33315004?v=4" width="5%" height="5%"/> </a>
<a href="https://github.com/gitdata001" target="_blank"><img src="https://avatars.githubusercontent.com/u/157772574?v=4" width="5%" height="5%"/> </a>

----
### License

Dual-licensed under [MIT](https://github.com/GitDataAI/jiaozifs/blob/main/LICENSE-MIT) + [Apache 2.0](https://github.com/GitDataAI/jiaozifs/blob/main/LICENSE-APACHE)


