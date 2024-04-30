<!-- README-zh-CN.md -->


```

 __    __     ______     __   __     ______     _____     __     ______     ______     ______     ______     __    __    
/\ "-./  \   /\  __ \   /\ "-.\ \   /\  __ \   /\  __-.  /\ \   /\  __ \   /\  ___\   /\  == \   /\  __ \   /\ "-./  \   
\ \ \-./\ \  \ \ \/\ \  \ \ \-.  \  \ \ \/\ \  \ \ \/\ \ \ \ \  \ \  __ \  \ \ \__ \  \ \  __<   \ \  __ \  \ \ \-./\ \  
 \ \_\ \ \_\  \ \_____\  \ \_\\"\_\  \ \_____\  \ \____-  \ \_\  \ \_\ \_\  \ \_____\  \ \_\ \_\  \ \_\ \_\  \ \_\ \ \_\ 
  \/_/  \/_/   \/_____/   \/_/ \/_/   \/_____/   \/____/   \/_/   \/_/\/_/   \/_____/   \/_/ /_/   \/_/\/_/   \/_/  \/_/ 
                                                                                                                         

```
<h1 align="center">
<br>
Mono-Diagram
<br>
</h1>

<p align="center">
<a href="https://crates.io/crates/mono-diagram"><img alt="crates.io" src="https://img.shields.io/crates/v/mono-diagram.svg"></a>
<a><img alt="License" src="https://img.shields.io/badge/License-MIT-blue.svg"></a>
</p>

<div align="center">

[English](./README.md) / [简体中文](./README-zh-CN.md) 

</div>

mono-diagram 是一个跨平台的生成纯文本图表的工具。纯文本图表的优势在于可以在任何地方显示，比如用在代码注释中。

## 使用方法

### 定义图表

使用 mono-diagram 的基本思路是写一个包含一个或多个图表定义的文件，然后传入程序，程序会生成图表。每个图表开头需要用标签声明图标类型。

<details>
<summary> 二叉树 </summary>

标签: `[binary_tree]`

输入: 

```
[binary_tree]   // 声明标签
a->b,c          // 节点a有左子树b和右子树c
b->d,f          // 节点名字类似于变量名
f->fa,fb
c->k,m
k->e,           // k只有左子树
m->,x

a:2             // 给每个节点名赋值
b:0.42
c:9.5
f:-3
k:abc
m:2             // 不同的节点可以有相同的值
d:001
fa:451
fb:8.90
x:1.2
```

输出:

```
            ___2___
        ___/       \___
     0.42             9.5
    _/   \_         _/   \_
  001     -3      abc      2
          / \     /         \
        451 8.90  e          1.2
```

</details>

<details>
<summary> 有向非循环图 (DAG) </summary>

标签: `[dag]`

输入:

```
[dag]
a->b    // 节点名->节点名 代表一条边
a->c    // 有向非循环图不能含有循环
b->d
c->f
c->g
a->f
d->da
d->db
g->gg
a->gg


a:Home Page     // 赋值
b:Main Section 1
c:Main Section 2
d:Subsection 1
f:Subsection 2
g:Subsection 3
da:Sub-sub
db:Sub-sub
gg:#page#
```

输出: 

```
 ┌───────────────────────────────────────────────────┐
 │ Home Page                                         │
 └┬─────────────────┬──┬────────────────────────────┬┘
 ┌V───────────────┐ │ ┌V───────────────┐            │
 │ Main Section 1 │ │ │ Main Section 2 │            │
 └┬───────────────┘ │ └┬────────────┬──┘            │
 ┌V─────────────┐ ┌─V──V─────────┐ ┌V─────────────┐ │
 │ Subsection 1 │ │ Subsection 2 │ │ Subsection 3 │ │
 └┬───────────┬─┘ └──────────────┘ └┬─────────────┘ │
 ┌V────────┐ ┌V────────┐ ┌──────────V───────────────V┐
 │ Sub-sub │ │ Sub-sub │ │ #page#                    │
 └─────────┘ └─────────┘ └───────────────────────────┘

```

*注：此生成不稳定，每次生成可能会得到不同的图*

</details>

</details>

<details>
<summary> 表格 </summary>

标签: `[table]`

输入:

```
[table]     \\ 每列用 '|' 分隔，每行用换行分隔
Base Class Member|Public Inheritance|Protected Inheritance|Private Inheritance
Public|Public|Protected|Private
Protected|Protected|Protected|Private
Private|Hidden|Hidden|Hidden
```

输出: 

```
+-------------------+--------------------+-----------------------+---------------------+
| Base Class Member | Public Inheritance | Protected Inheritance | Private Inheritance |
+-------------------+--------------------+-----------------------+---------------------+
| Public            | Public             | Protected             | Private             |
+-------------------+--------------------+-----------------------+---------------------+
| Protected         | Protected          | Protected             | Private             |
+-------------------+--------------------+-----------------------+---------------------+
| Private           | Hidden             | Hidden                | Hidden              |
+-------------------+--------------------+-----------------------+---------------------+
```

</details>

</details>

<details>
<summary> 网格 </summary>

标签: `[grid]`

输入:

```
[grid] {10, 7}      // The grid has 10 colums and 7 rows

1,1:a 
6,2:l               // The cell at column 6, row 2 has content 'l'
3,3:j
10,5:m
2,7:k
```

输出: 

```
+---+---+---+---+---+---+---+---+---+---+
| a |   |   |   |   |   |   |   |   |   |
+---+---+---+---+---+---+---+---+---+---+
|   |   |   |   |   | l |   |   |   |   |
+---+---+---+---+---+---+---+---+---+---+
|   |   | j |   |   |   |   |   |   |   |
+---+---+---+---+---+---+---+---+---+---+
|   |   |   |   |   |   |   |   |   |   |
+---+---+---+---+---+---+---+---+---+---+
|   |   |   |   |   |   |   |   |   | m |
+---+---+---+---+---+---+---+---+---+---+
|   |   |   |   |   |   |   |   |   |   |
+---+---+---+---+---+---+---+---+---+---+
|   | k |   |   |   |   |   |   |   |   |
+---+---+---+---+---+---+---+---+---+---+
```

</details>

### 命令行参数

```
用法: mono-diagram [OPTIONS] <FILE_PATH>

参数:
  <FILE_PATH>
          文件路径

选项:
  -p, --prefix <PREFIX>
          给输出的每行加一个前缀

          可用于代码注释

  -c, --copy
          复制到剪贴板

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
```

#### 示例命令 

```bash
mono-diagram examples/test -c -p "# " 
```

## 安装

安装 [rust](https://www.rust-lang.org/tools/install) 

然后运行命令：

~~~bash
cargo install mono-diagram
~~~

## TODO

1. dag 中的节点可以是表格 (这样就可以画类图)
2. 优化 dag 中边的绘制
3. 增加韦恩图
4. 增加时序图
5. 增加表格的单元格合并

## 贡献和帮助

非常欢迎 contribution，就算只是 contribute idea 也很欢迎

如有 bug 请开 issue （dag 应该是有 bug 的）

