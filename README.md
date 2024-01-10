# JiaoziFS
A version control file system for data centric applications & teams.

<p align="left">
  <a href="https://codecov.io/gh/jiaozifs/jiaozifs"><img src="https://codecov.io/gh/jiaozifs/jiaozifs/branch/main/graph/badge.svg"></a>
  <a href="https://goreportcard.com/report/github.com/jiaozifs/jiaozifs"><img src="https://goreportcard.com/badge/github.com/jiaozifs/jiaozifs" /></a>  
  <a href=""><img src="https://img.shields.io/badge/golang-%3E%3D1.20.10-blue.svg" /></a>
  <br>
</p>

### Basic Build And Usage

#### Requirement
1. To build JiaoziFS, you need a working installation of   [Go 1.20.10 or higher](https://golang.org/dl/)
2. JiaoziFS use postgres to store running data, you can install at  [postgres install installation guide](https://www.postgresql.org/docs/current/installation.html)

#### Build And Running

1. clone and build
```bash
git clone https://github.com/jiaozifs/jiaozifs.git
cd jiaozifs
make build
```

After following the above steps, you should be able to see an executable file named "jzfs."

2. init program and running
```bash
./jzfs init  --db postgres://<username>:<password>@localhost:5432/jiaozifs?sslmode=disable
./jzfs daemon
```

## License

Dual-licensed under [MIT](https://github.com/jiaozifs/jiaozifs/blob/main/LICENSE-MIT) + [Apache 2.0](https://github.com/jiaozifs/jiaozifs/blob/main/LICENSE-APACHE)
