一些参考的资料和引用

1. lakefs实现 https://github.com/treeverse/lakeFS
2. git内部原理 https://git-scm.com/book/zh/v2/Git-%E5%86%85%E9%83%A8%E5%8E%9F%E7%90%86-%E5%BA%95%E5%B1%82%E5%91%BD%E4%BB%A4%E4%B8%8E%E4%B8%8A%E5%B1%82%E5%91%BD%E4%BB%A4
3. go-git代码实现 https://github.com/go-git/go-git
4. gitlab实现 https://github.com/gitlabhq/gitlabhq
5. https://github.com/dolthub



git设计

1. objects 
    数据对象 100644普通文件/100755可执行文件/120000符号链接
    树对象
    提交对象

2. refs
    HEAD     .git/HEAD
    引用      .git/refs/heads
    标签引用   .git/refs/tags
    远程引用   .git/refs/remotes

3. 包文件 压缩上述文件到包里面
 
4. 传输协议 smart http/ssh
