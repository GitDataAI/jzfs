
# JiaoZiFS (JZFS)
A version control file system for data linage & data collaboration.

<p align="left">
  <a href="https://codecov.io/gh/jiaozifs/jiaozifs"><img src="https://codecov.io/gh/gitdataai/jiaozifs/branch/main/graph/badge.svg" /></a>
  <a href="https://goreportcard.com/report/github.com/jiaozifs/jiaozifs"><img src="https://goreportcard.com/badge/github.com/gitdataai/jiaozifs" /></a>  
  <a href=""><img src="https://img.shields.io/badge/golang-%3E%3D1.22.0-blue.svg" /></a>
  <br/>
</p>

<a href="https://github.com/GitDataAI/jiaozifs"><img src="https://github.com/GitDataAI/jiaozifs/blob/main/docs/logo/jiaozifs.png?raw=true" width="100" /></a>

----
JiaoZiFS is an industry-leading **Data-Centric Version Control** File System, helps ensure Responsible AI Engineering by improving **Data Versioning**, **Provenance**, and **Reproducibility**.

Note:
* The name JiaoZi pays tribute to the world's earliest paper money: [Song Dynasty JiaoZi](https://en.wikipedia.org/wiki/Jiaozi_(currency)).
* JiaoZiFS is yet another implementation of [IPFS (InterPlanetary File System)](https://ipfs.tech/) as JiaoZiFS will be compatible with the [implementation requirements](https://specs.ipfs.tech/architecture/principles/#ipfs-implementation-requirements) of IPFS.
* As a filesystem of data versioning at scale, although JiaoZiFS is built for machine learning, It has a wide range of use scenarios (refer A Universe of Uses) and can be seamlessly integrated into all your data stack.

Data-centric AI is about the practice of iterating and collaborating on data, used to build AI systems, programmatically. Machine learning pioneer Andrew Ng [argues that focusing on the quality of data fueling AI systems will help unlock its full power](https://youtu.be/TU6u_T-s68Y).

----
### Features

In production systems with machine learning components, updates and experiments are frequent. New updates to models(data products) may be released every day or every few minutes, and different users may see the results of different models as part of A/B experiments or canary releases.

* **Version Everything**: Data scientists are often criticized for being less disciplined with versioning their experiments(versioning of data, pipeline, code, and models), especially when using computational notebooks.
* **Track Data Provenance**: This applies to all processing steps in an AI/ML pipeline, including data collection/acquisition, data merging, data cleaning, feature extraction, learning, or deployment.
* **Reproducibility**: A final question of AI/ML that is often relevant for debugging, audits, and also science more broadly is to what degree data, models, and decisions can be reproduced.

----
### Getting Started

#### Requirement

1. To build JiaoZiFS, you need a working installation of   [Go 1.22.0 or higher](https://golang.org/dl/)
2. JiaoZiFS use postgres to store running data, you can install at  [postgres install installation guide](https://www.postgresql.org/docs/current/installation.html)

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
#### Cloud

[Try without installing](https://cloud.jiaozifs.com)

Note: storage config for IPFS backend storage as you create a new repository in JiaoZiFS UI.

```
 {"type":"ipfs","ipfs":{"url":"/dns/kubo-service.ipfs.svc.cluster.local/tcp/5001"}}
```

#### Examples
Build AL/ML pipeline over JiaoZiFS   
[Face detection and recognition inference pipeline](https://colab.research.google.com/drive/1wsv-KMxTdsCLZ64eLq4W1MTfspid-vv6?usp=sharing)

----
### Documentation

[Official Documentation](https://docs.gitdata.ai)

----
### Users and Partners

[Lighthouse Permanent Storage](https://www.lighthouse.storage/)   
[MesoReef DAO: Decentralized Science for Regenerating](https://linktr.ee/mesoreefdao)    
[LunCo](https://www.lunco.space/)   
[Artizen Fund](https://artizen.fund/)   
[HaAI Labs](https://haai.info/)   

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


